use syn::{Attribute, Meta, Expr, Lit, parse_quote, Item};

pub struct Docs {
  pub strings: Vec<String>
}

impl Docs {
  pub fn process_item(item: Item, is_sync: bool) -> Item {
    macro_rules! process_item {
      ($x: expr, $enum: expr) => {
        {
          let mut new = $x.clone();
          new.attrs = Docs::new().process_attributes(new.attrs, is_sync);
          $enum(new)
        }
      }
    }

    match item {
      Item::Const(x) => process_item!(x, Item::Const),
      Item::Enum(x) => process_item!(x, Item::Enum),
      Item::ExternCrate(x) => process_item!(x, Item::ExternCrate),
      Item::Fn(x) => process_item!(x, Item::Fn),
      Item::ForeignMod(x) => process_item!(x, Item::ForeignMod),
      Item::Impl(x) => process_item!(x, Item::Impl),
      Item::Macro(x) => process_item!(x, Item::Macro),
      Item::Mod(x) => process_item!(x, Item::Mod),
      Item::Static(x) => process_item!(x, Item::Static),
      Item::Struct(x) => process_item!(x, Item::Struct),
      Item::Trait(x) => process_item!(x, Item::Trait),
      Item::TraitAlias(x) => process_item!(x, Item::TraitAlias),
      Item::Type(x) => process_item!(x, Item::Type),
      Item::Union(x) => process_item!(x, Item::Union),
      Item::Use(x) => process_item!(x, Item::Use),
      item => item
    }
  }

  fn new() -> Self {
    Self { strings: vec![] }
  }

  fn is_empty(&self) -> bool {
    self.strings.is_empty()
  }

  fn process_attributes(&mut self, v: Vec<Attribute>, is_sync: bool) -> Vec<Attribute> {
    let mut result = vec![];
    for attr in v.into_iter() {
      if !self.collect(&attr) {
        result.push(attr);
      }
    }

    if !self.is_empty() {
      result.push(self.as_attribute(is_sync));
    }

    result
  }

  fn collect(&mut self, attribute: &Attribute) -> bool {
    let name_value = match &attribute.meta {
      Meta::NameValue(x) => x,
      _ => return false
    };
    
    if !name_value.path.is_ident("doc") {
      return false;
    }

    let lit = match &name_value.value {
      Expr::Lit(x) => x,
      _ => return false
    };

    let lit_str = match &lit.lit {
      Lit::Str(x) => x,
      _ => return false
    };

    for a in lit_str.value().split("\n") {
      self.strings.push(a.to_string());
    }

    true
  }

  fn as_attribute(&self, is_sync: bool) -> Attribute {
    let str = self.as_string(is_sync);
    
    parse_quote!(#[doc = #str])
  }

  fn as_string(&self, is_sync: bool) -> String {
    let mut result = vec![];

    let mut state = 0;
    for s in self.strings.iter() {
      let trim = s.trim();
      if trim == "[synca::sync]" {
        if state != 0 {
          panic!("Unprocessed [synca::sync]: prev command not closed")
        }

        state = 1;
        continue;
      }

      if trim == "[synca::async]" {
        if state != 0 {
          panic!("Unprocessed [synca::async]: prev command not closed")
        }

        state = -1;
        continue;
      }

      if trim == "[/synca::sync]" {
        if state != 1 {
          panic!("Unprocessed [/synca::sync]: synca::sync not opened")
        }

        state = 0;
        continue;
      }

      if trim == "[/synca::async]" {
        if state != -1 {
          panic!("Unprocessed [/synca::async]: synca::async not opened")
        }

        state = 0;
        continue;
      }
      
      if state == 0 || (is_sync && state == 1) || (!is_sync && state == -1) {
        result.push(Self::process_str(s, state, is_sync));
      }
    }

    result.join("\n")
  }

  fn process_str(s: &str, state: i32, is_sync: bool) -> String {
    if state != 0 {
      return s.to_string();
    }

    s.chars().fold(
      (MatchState::Start(0), vec![]), 
      |(state, mut result), c| {
        (
          match state {
            MatchState::Start(x) => {
              result.push(c);
  
              match SYNCA_MATCH_START[x] == c {
                true => {
                  if x == 13 { 
                    result.truncate(result.len().saturating_sub(14));
                    MatchState::Left 
                  } else { 
                    MatchState::Start(x + 1) 
                  }
                },
                false => MatchState::Start(0)
              }
            },
            MatchState::Left => {
              if c != SYNCA_MATCH_DELIMITER {
                if !is_sync {
                  result.push(c);
                }
    
                MatchState::Left
              } else {
                MatchState::Right(0)
              }
            },
            MatchState::Right(x) => {
              if is_sync { result.push(c); }
    
              match SYNCA_MATCH_END[x] == c {
                true => {
                  if x == 14 { 
                    if is_sync { result.truncate(result.len().saturating_sub(15)); }
    
                    MatchState::Start(0)
                  } else {
                    MatchState::Right(x + 1)
                  }
                },
                false => MatchState::Right(0),
              }
            }
          },
          result
        )
      }
    ).1.iter().collect()
  }
}

const SYNCA_MATCH_START: [ char; 14 ] = [ '[', 's', 'y', 'n', 'c', 'a', ':', ':', 'm', 'a', 't', 'c', 'h', ']' ];
const SYNCA_MATCH_DELIMITER: char = '|';
const SYNCA_MATCH_END: [ char; 15 ] = [ '[', '/', 's', 'y', 'n', 'c', 'a', ':', ':', 'm', 'a', 't', 'c', 'h', ']' ];

enum MatchState {
  Start(usize),
  Left,
  Right(usize),

}

#[cfg(test)]
mod tests {
  use super::Docs;

  #[test]
  pub fn synca_sync() {
    assert_eq!(
      process("Before\n[synca::sync]\nSync text\n[/synca::sync]\nAfter"),
      (
        "Before\nAfter".into(),
        "Before\nSync text\nAfter".into(), 
      )
    );
  }

  #[test]
  pub fn synca_async() {
    assert_eq!(
      process("Before\n[synca::async]\nAsync text\n[/synca::async]\nAfter"),
      (
        "Before\nAsync text\nAfter".into(), 
        "Before\nAfter".into(),
      )
    );
  }

  #[test]
  pub fn synca_match() {
    assert_eq!(
      process("Before [synca::match]Async text|Sync text[/synca::match] After"),
      (
        "Before Async text After".into(), 
        "Before Sync text After".into(),
      )
    );

    assert_eq!(
      process("[synca::match]0|1[/synca::match][synca::match]2|3[/synca::match][synca::match]4|5[/synca::match]"),
      ("024".into(), "135".into())
    );
  }

  pub fn process(s: &str) -> (String, String) {
    let docs = Docs { strings: s.split("\n").map(|x| x.to_string()).collect() };
    (docs.as_string(false), docs.as_string(true))
  }
}