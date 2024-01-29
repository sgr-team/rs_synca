use std::collections::HashMap;

use quote::ToTokens;
use syn::{parse::ParseStream, punctuated::Punctuated, parse_quote, Attribute, Token, Type};

use crate::SyncAFold;

pub struct Replace<'a> {
  pub types: &'a mut HashMap<Type, Type>,
  pub attributes: &'a mut HashMap<Attribute, Attribute>,
}

pub enum ReplaceItem {
  Type((Type, Type)),
  Attribute((Attribute, Attribute)),
}

impl syn::parse::Parse for ReplaceItem {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if let Ok(source) = input.parse::<Type>() {
      input.parse::<Token![=>]>()?;
      return Ok(ReplaceItem::Type((source, input.parse()?)))
    }

    if let Ok(attrs) = Attribute::parse_outer(input) {
      if attrs.len() != 1 {
        return Err(
          syn::Error::new(
            input.span(), 
            "SuncA expected one attribute line #[tokio::test] => #[test]"
          )
        );
      }

      let source = attrs[0].clone();
      input.parse::<Token![=>]>()?;

      let new_attrs = Attribute::parse_outer(input)?;
      if new_attrs.len() != 1 {
        return Err(
          syn::Error::new(
            input.span(), 
            "SuncA expected one attribute line #[tokio::test] => #[test]"
          )
        );
      }

      return Ok(ReplaceItem::Attribute((source, new_attrs[0].clone())))
    }
    
    Err(syn::Error::new(input.span(), "Unhandled replace item (supported types and attributes)"))
  }
}

impl<'a> Replace<'a> {
  pub fn new(fold: &'a mut SyncAFold) -> Self {
    Self { types: &mut fold.types, attributes: &mut fold.attributes }
  }

  pub fn apply(&mut self, m: &syn::ItemMacro) -> syn::Result<()> {
    let ts = m.mac.tokens.clone().into_token_stream();
    let items: Punctuated::<ReplaceItem, Token![,]> = parse_quote!(#ts);

    for item in items.iter() {
      match item {
        ReplaceItem::Type(x) => { self.types.insert(x.0.clone(), x.1.clone()); },
        ReplaceItem::Attribute(x) => { self.attributes.insert(x.0.clone(), x.1.clone()); },
      }
    }

    Ok(())
  }
}