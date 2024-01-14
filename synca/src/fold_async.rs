use syn::{
  fold::{self, Fold},
  Item
};

use crate::docs::Docs;

pub struct AsyncFold {
}

impl AsyncFold {
  pub fn new() -> Self { Self { } }
}

impl Fold for AsyncFold {
  fn fold_item(&mut self, i: Item) -> Item {
    fold::fold_item(self, Docs::process_item(i, false))
  }
}

#[cfg(test)]
mod tests {
  use quote::ToTokens;
  use syn::{parse_quote, fold::Fold, Item};

  use super::AsyncFold;

  #[test]
  fn fold_fn_docs() {
    assert_eq!(
      AsyncFold::new().fold_item(parse_quote!(
        /// # My header
        /// 
        /// We use [synca::match]tokio_postgres|postgres[/synca::match]
        /// 
        /// Any text
        /// [synca::sync]
        /// sync text
        /// [/synca::sync]
        /// [synca::async]
        /// async text
        /// [/synca::async]
        pub fn any() { }
      )).into_token_stream().to_string(),
      {
        let result: Item = parse_quote!(
          #[doc = " # My header\n \n We use tokio_postgres\n \n Any text\n async text"]
          pub fn any() { }
        );
        result.into_token_stream().to_string()
      }
    );
  }
}