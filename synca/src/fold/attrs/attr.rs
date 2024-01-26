use syn::{parse_quote, Attribute};

#[derive(Debug, PartialEq)]
pub enum SyncAAttribute {
  Other(Attribute),
  Doc(String),
  Ignore,
  OnlyAsync,
  OnlySync,
}

impl From<Attribute> for SyncAAttribute {
  fn from(value: Attribute) -> Self {
    if value == parse_quote!(#[synca::ignore]) {
      return SyncAAttribute::Ignore;
    }

    if value == parse_quote!(#[synca::only(async)]) {
      return SyncAAttribute::OnlyAsync;
    }

    if value == parse_quote!(#[synca::only(sync)]) {
      return SyncAAttribute::OnlySync;
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
  fn doc() {
    let attr: Attribute = parse_quote!(#[doc = "my text"]);

    assert_eq!(SyncAAttribute::from(attr.clone()), SyncAAttribute::Doc("my text".into()));
  }

  #[test]
  fn ignore() {
    let attr: Attribute = parse_quote!(#[synca::ignore]);

    assert_eq!(SyncAAttribute::from(attr.clone()), SyncAAttribute::Ignore);
  }

  #[test]
  fn only_async() {
    let attr: Attribute = parse_quote!(#[synca::only(async)]);

    assert_eq!(SyncAAttribute::from(attr.clone()), SyncAAttribute::OnlyAsync);
  }

  #[test]
  fn only_sync() {
    let attr: Attribute = parse_quote!(#[synca::only(sync)]);

    assert_eq!(SyncAAttribute::from(attr.clone()), SyncAAttribute::OnlySync);
  }
}