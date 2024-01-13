use std::collections::HashMap;

use quote::ToTokens;
use syn::{Expr, Type, parse::Parse, Token, Attribute};

use crate::fold::SyncAFold;

#[derive(Debug, PartialEq)]
pub struct SyncAAttribute {
  pub features: Expr,
  pub types: HashMap<String, Type>,
  pub attributes: HashMap<String, Attribute>,
}

impl SyncAAttribute {
  pub fn fold(self) -> SyncAFold {
    SyncAFold { types: self.types, attributes: self.attributes }
  }
}

impl Parse for SyncAAttribute {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let features: Expr = input.parse()?;
    let mut types = HashMap::new();
    let mut attributes = HashMap::new();

    while input.parse::<Token![,]>().is_ok() {
      if input.is_empty() {
        break;
      }

      if let Ok(async_ty) = input.parse::<Type>() {
        input.parse::<Token![=>]>()?;
        types.insert(async_ty.to_token_stream().to_string(), input.parse::<Type>()?);
        continue;
      }

      if let Ok(attrs) = Attribute::parse_outer(input) {
        if attrs.len() > 0 {
          if attrs.len() > 1 {
            panic!("SuncA expected one attribute line synca(feature = tokio, #[tokio::test] => #[test])");
          }
          let at = attrs[0].clone();
          input.parse::<Token![=>]>()?;

          let new_attrs = Attribute::parse_outer(input)?;
          if new_attrs.len() != 1 {
            panic!("SuncA expected one attribute line synca(feature = tokio, #[tokio::test] => #[test])");
          }

          attributes.insert(at.to_token_stream().to_string(), new_attrs[0].clone());
          continue;  
        }
      }
    }

    Ok(Self { features, types, attributes })
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use syn::parse_quote;

  use super::SyncAAttribute;

  #[test]
  fn parse_one_feature() {
    let attr: SyncAAttribute = parse_quote!(feature = "async");

    assert_eq!(
      attr,
      SyncAAttribute {
        features: parse_quote!(feature = "async"),
        types: HashMap::new(),
        attributes: HashMap::new(),
      }
    );
  }

  #[test]
  fn parse_all() {
    let attr: SyncAAttribute = parse_quote!(all(feature = "foo", feature = "bar"));

    assert_eq!(
      attr,
      SyncAAttribute {
        features: parse_quote!(all(feature = "foo", feature = "bar")),
        types: HashMap::new(),
        attributes: HashMap::new(),
      }
    );
  }

  #[test]
  fn parse_not() {
    let attr: SyncAAttribute = parse_quote!(not(all(feature = "sync", feature = "sync_super")));
    
    assert_eq!(
      attr,
      SyncAAttribute {
        features: parse_quote!(not(all(feature = "sync", feature = "sync_super"))),
        types: HashMap::new(),
        attributes: HashMap::new(),
      }
    );
  }

  #[test]
  fn parse_with_types() {
    let attr: SyncAAttribute = parse_quote!(
      not(all(feature = "sync", feature = "sync_super")), 
      tokio_postgres::Client => postgres::Client,
      tokio_postgres::NoTls => postgres::NoTls
    );
    let attr2: SyncAAttribute = parse_quote!(
      not(all(feature = "sync", feature = "sync_super")), 
      tokio_postgres::Client => postgres::Client,
      tokio_postgres::NoTls => postgres::NoTls,
    );
    
    assert_eq!(
      attr,
      SyncAAttribute {
        features: parse_quote!(not(all(feature = "sync", feature = "sync_super"))),
        types: HashMap::from([
          ("tokio_postgres :: Client".into(), parse_quote!(postgres::Client)),
          ("tokio_postgres :: NoTls".into(), parse_quote!(postgres::NoTls))
        ]),
        attributes: HashMap::new(),
      }
    );
    assert_eq!(
      attr2,
      SyncAAttribute {
        features: parse_quote!(not(all(feature = "sync", feature = "sync_super"))),
        types: HashMap::from([
          ("tokio_postgres :: Client".into(), parse_quote!(postgres::Client)),
          ("tokio_postgres :: NoTls".into(), parse_quote!(postgres::NoTls)),
        ]),
        attributes: HashMap::new(),
      }
    );
  }

  #[test]
  fn parse_with_types_and_attrs() {
    let attr: SyncAAttribute = parse_quote!(
      not(all(feature = "sync", feature = "sync_super")), 
      tokio_postgres::Client => postgres::Client,
      tokio_postgres::NoTls => postgres::NoTls,
      #[tokio::test] => #[test]
    );

    assert_eq!(
      attr,
      SyncAAttribute {
        features: parse_quote!(not(all(feature = "sync", feature = "sync_super"))),
        types: HashMap::from([
          ("tokio_postgres :: Client".into(), parse_quote!(postgres::Client)),
          ("tokio_postgres :: NoTls".into(), parse_quote!(postgres::NoTls))
        ]),
        attributes: HashMap::from([
          ("# [tokio :: test]".into(), parse_quote!(#[test])),
        ]),
      }
    );
  }
}