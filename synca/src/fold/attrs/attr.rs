use syn::{parse_quote, Attribute, Ident};

#[derive(Debug, PartialEq)]
pub enum SyncAAttribute {
  Other(Attribute),
  Cfg(Ident),
  Doc(String),
  Ignore,
}

impl From<Attribute> for SyncAAttribute {
  fn from(value: Attribute) -> Self {
    if value == parse_quote!(#[synca::ignore]) {
      return SyncAAttribute::Ignore;
    }

    if value.path() == &parse_quote!(synca::cfg) {
      let i: Ident = value.parse_args().unwrap();
      return SyncAAttribute::Cfg(i.clone());
    }

    let name_value = match &value.meta {
      syn::Meta::NameValue(x) => x,
      _ => return Self::Other(value)
    };

    if !name_value.path.is_ident("doc") {
      return Self::Other(value)
    }

    let lit = match &name_value.value {
      syn::Expr::Lit(x) => x,
      _ => return Self::Other(value)
    };

    match &lit.lit {
      syn::Lit::Str(x) => Self::Doc(x.value()),
      _ => Self::Other(value)
    }
  }
}

#[cfg(test)]
mod from {
  use syn::{parse_quote, Attribute};

  use crate::SyncAAttribute;

  #[test]
  fn other() {
    let attr: Attribute = parse_quote!(#[test]);

    assert_eq!(SyncAAttribute::from(attr.clone()), SyncAAttribute::Other(attr));
  }

  #[test]
  fn cfg() {
    let attr: Attribute = parse_quote!(#[synca::cfg(tokio)]);

    assert_eq!(SyncAAttribute::from(attr), SyncAAttribute::Cfg(parse_quote!(tokio)));
  }

  #[test]
  fn doc() {
    let attr: Attribute = parse_quote!(#[doc = "my text"]);

    assert_eq!(SyncAAttribute::from(attr.clone()), SyncAAttribute::Doc("my text".into()));
  }

  #[test]
  fn ignore() {
    let attr: Attribute = parse_quote!(#[synca::ignore]);

    assert_eq!(SyncAAttribute::from(attr.clone()), SyncAAttribute::Ignore);
  }
}