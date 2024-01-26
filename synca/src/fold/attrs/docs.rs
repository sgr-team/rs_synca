use syn::{parse_quote, Attribute};

use crate::SyncAFoldAttributes;

impl SyncAFoldAttributes {
  pub fn docs_attribute(&self, docs: Vec<String>) -> Option<Attribute> {
    let mut is_empty = true;
    let mut result = vec![];
    let mut state = DocState::None;

    macro_rules! process_doc_state {
      ($trimmed: expr, $token: expr) => {
        if $trimmed == $token.to_string() {
          if state != DocState::None {
            panic!(
              r#"async::docs unhandled open {} - "{}" not closed"#, 
              $token.to_string(),
              state.to_string()
            );
          }
  
          state = $token;
          continue;
        }

        if $trimmed == $token.end_token() {
          if state != $token {
            panic!("async::docs {} unhandled close", state.end_token());
          }
  
          state = DocState::None;
          continue;
        }
      };
    }

    for s in docs.iter() {
      let trimmed = s.trim();

      process_doc_state!(trimmed, DocState::Async);
      process_doc_state!(trimmed, DocState::Sync);

      if (self.is_async && state == DocState::Sync) || (!self.is_async && state == DocState::Async) {
        continue;
      }

      let processed = self.process_str(s);
      if processed.contains(|x| x != ' ') {
        is_empty = false;
      }
      
      result.push(processed);
    }

    if is_empty { 
      return None; 
    }

    let processed = result.join("\n");
    Some(parse_quote!(#[doc = #processed]))
  }

  fn process_str(&self, s: &str) -> String {
    let mut state = DocStringState::Start(0);
    let mut result = vec![];

    for c in s.chars() {
      state = match state {
        DocStringState::Start(x) => {
          result.push(c);

          if SYNCA_MATCH_START[x] == c {
            if x == 13 { 
              result.truncate(result.len().saturating_sub(14));
              DocStringState::Left 
            } else { 
              DocStringState::Start(x + 1) 
            }
          }
          else { 
            DocStringState::Start(0) 
          }
        },
        DocStringState::Left => {
          if c != SYNCA_MATCH_DELIMITER {
            if self.is_async {
              result.push(c);
            }

            DocStringState::Left
          } else {
            DocStringState::Right(0)
          }
        },
        DocStringState::Right(x) => {
          if !self.is_async { result.push(c); }
  
          match SYNCA_MATCH_END[x] == c {
            true => {
              if x == 14 { 
                if !self.is_async { result.truncate(result.len().saturating_sub(15)); }

                DocStringState::Start(0)
              } else {
                DocStringState::Right(x + 1)
              }
            },
            false => DocStringState::Right(0),
          }
        },
      };
    }
    
    match state {
      DocStringState::Start(_) => result.iter().collect(),
      _ => panic!(r#"[synca::match]: not closed "{}""#, s),
    }
  }
}

#[derive(PartialEq)]
enum DocState {
  None,
  Async,
  Sync
}

impl DocState {
  pub fn end_token(&self) -> String {
    match self {
      DocState::None => "None".into(),
      DocState::Async => "[/synca::async]".into(),
      DocState::Sync => "[/synca::sync]".into(),
    }
  }
}

impl ToString for DocState {
  fn to_string(&self) -> String {
    match self {
      DocState::None => "None".into(),
      DocState::Async => "[synca::async]".into(),
      DocState::Sync => "[synca::sync]".into(),
    }
  }
}

enum DocStringState {
  Start(usize),
  Left,
  Right(usize)
}

const SYNCA_MATCH_START: [ char; 14 ] = [ '[', 's', 'y', 'n', 'c', 'a', ':', ':', 'm', 'a', 't', 'c', 'h', ']' ];
const SYNCA_MATCH_DELIMITER: char = '|';
const SYNCA_MATCH_END: [ char; 15 ] = [ '[', '/', 's', 'y', 'n', 'c', 'a', ':', ':', 'm', 'a', 't', 'c', 'h', ']' ];

#[cfg(test)]
mod docs_attribute {
  use syn::{parse_quote, Attribute};

  use crate::SyncAFoldAttributes;

  #[test]
  fn simple() {
    assert_eq!(
      process(" # Header\n \n My text"),
      (
        Some(parse_quote!(#[doc = " # Header\n \n My text"])),
        Some(parse_quote!(#[doc = " # Header\n \n My text"]))
      )
    );
  }

  #[test]
  fn complex() {
    assert_eq!(
      process(" # Header\n [synca::async]\n Async\n [/synca::async]\n [synca::sync]\n Sync\n [/synca::sync]\n Postfix"),
      (
        Some(parse_quote!(#[doc = " # Header\n Async\n Postfix"])),
        Some(parse_quote!(#[doc = " # Header\n Sync\n Postfix"]))
      )
    );
  }

  #[test]
  fn empty() {
    assert_eq!(process("     \n    \n   \n \n\n"), (None, None));
  }

  #[test]
  fn synca_async() {
    assert_eq!(
      process(" # Header\n \n [synca::async]  \n My text \n Multiline \n [/synca::async]\n Postfix"),
      (
        Some(parse_quote!(#[doc = " # Header\n \n My text \n Multiline \n Postfix"])),
        Some(parse_quote!(#[doc = " # Header\n \n Postfix"]))
      )
    );
  }

  #[test]
  fn synca_sync() {
    assert_eq!(
      process(" # Header\n \n [synca::sync]  \n My text \n Multiline \n [/synca::sync]\n Postfix"),
      (
        Some(parse_quote!(#[doc = " # Header\n \n Postfix"])),
        Some(parse_quote!(#[doc = " # Header\n \n My text \n Multiline \n Postfix"]))
      )
    );
  }

  fn process(s: &str) -> (Option<Attribute>, Option<Attribute>) {
    (
      SyncAFoldAttributes { is_async: true, ignored: true, only_async: true, only_sync: true, new_attrs: vec![] }
        .docs_attribute(s.split("\n").map(|x| x.into()).collect()),
      SyncAFoldAttributes { is_async: false, ignored: true, only_async: true, only_sync: true, new_attrs: vec![] }
        .docs_attribute(s.split("\n").map(|x| x.into()).collect())
    )
  }
}

#[cfg(test)]
mod process_str {
  use crate::SyncAFoldAttributes;

  #[test]
  fn simple() {
    assert_eq!(
      process("Simple string"),
      (
        "Simple string".into(), 
        "Simple string".into()
      )
    );
  }

  #[test]
  fn match_one() {
    assert_eq!(
      process("Match: [synca::match]Async text|Sync text[/synca::match] - postfix"),
      (
        "Match: Async text - postfix".into(), 
        "Match: Sync text - postfix".into()
      )
    );
  }

  #[test]
  fn match_multi() {
    assert_eq!(
      process("Numbers: [synca::match]0|1[/synca::match][synca::match]2|3[/synca::match];"),
      (
        "Numbers: 02;".into(), 
        "Numbers: 13;".into()
      )
    );
  }

  #[test]
  #[should_panic]
  fn match_not_closed() {
    process("Numbers: [synca::match];");
  }

  fn process(s: &str) -> (String, String) {
    (
      SyncAFoldAttributes { is_async: true, ignored: true, only_async: true, only_sync: true, new_attrs: vec![] }
        .process_str(s.into()),
      SyncAFoldAttributes { is_async: false, ignored: true, only_async: true, only_sync: true, new_attrs: vec![] }
        .process_str(s.into())
    )
  }
}