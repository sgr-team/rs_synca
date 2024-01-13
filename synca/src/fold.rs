use std::collections::HashMap;

use quote::ToTokens;
use syn::{
  fold::{self, Fold},
  Expr, ExprBlock, Type, Attribute
};

pub struct SyncAFold {
  pub types: HashMap<String, Type>,
  pub attributes: HashMap<String, Attribute>,
}

impl Fold for SyncAFold {
  fn fold_item_fn(&mut self, i: syn::ItemFn) -> syn::ItemFn {
    let mut new_fn = i.clone();
    new_fn.sig.asyncness = None;

    fold::fold_item_fn(self, new_fn)
  }

  fn fold_impl_item_fn(&mut self, i: syn::ImplItemFn) -> syn::ImplItemFn {
    let mut new_item = i.clone();
    new_item.sig.asyncness = None;
    
    fold::fold_impl_item_fn(self, new_item)
  }

  fn fold_expr(&mut self, e: Expr) -> Expr {
    match e {
      Expr::Await(e) => self.fold_expr(*e.base),
      Expr::Async(e) => self.fold_expr(Expr::Block(ExprBlock {
        attrs: e.attrs,
        label: None,
        block: e.block,
      })),
      _ => fold::fold_expr(self, e),
    }
  }

  fn fold_type(&mut self, ty: Type) -> syn::Type {
    match ty {
      Type::Path(path) => {
        let hash = path.clone().into_token_stream().to_string();
        match self.types.get(&hash) {
          Some(new_ty) => new_ty.clone(),
          None => fold::fold_type(self, Type::Path(path)),
        }
      },
      _ => fold::fold_type(self, ty),
    }
  }

  fn fold_trait_item_fn(&mut self, i: syn::TraitItemFn) -> syn::TraitItemFn {
    let mut new_trait_item_fn = i.clone();
    new_trait_item_fn.sig.asyncness = None;

    new_trait_item_fn
  }

  fn fold_attribute(&mut self, i: Attribute) -> Attribute {
    let mut new_attr = i;
    if let Some(x) = self.attributes.get(&new_attr.clone().to_token_stream().to_string()) {
      new_attr = x.clone();
    }

    fold::fold_attribute(self, new_attr)
  }

  fn fold_macro(&mut self, i: syn::Macro) -> syn::Macro {
    let macro_str = fold::fold_macro(self, i)
      .to_token_stream()
      .to_string()
      .replace(". await", "")
      .replace(".await", "");
    
    syn::parse_str(&macro_str).unwrap()
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use syn::{parse_quote, fold::Fold};

  use super::SyncAFold;

  #[test]
  fn fold_enum() {
    assert_eq!(
      create_synca_fold().fold_item(parse_quote!(
        enum Status { 
          Error(String),
          Connected(tokio_postgres::Client)
        }
      )),
      parse_quote!(
        enum Status { 
          Error(String),
          Connected(postgres::Client)
        }
      )
    );
  }
  
  #[test]
  fn fold_fn() {
    assert_eq!(
      create_synca_fold().fold_item(parse_quote!(
        async fn select(client: &mut tokio_postgres::Client) { 
          let row = client.queryOne(r#"SELECT name FROM "Books""#).await;

          row.get("name")
        }
      )),
      parse_quote!(
        fn select(client: &mut postgres::Client) { 
          let row = client.queryOne(r#"SELECT name FROM "Books""#);

          row.get("name")
        }
      )
    );
  }

  #[test]
  fn fold_impl() {
    assert_eq!(
      create_synca_fold().fold_item(parse_quote!(
        impl Status { 
          fn new() -> Self { Self { } }

          pub type MainClient = tokio_postgres::Client;

          async fn select(client: &mut tokio_postgres::Client) { 
            let row = client.queryOne(r#"SELECT name FROM "Books""#).await;
  
            row.get("name")
          }
        }
      )),
      parse_quote!(
        impl Status { 
          fn new() -> Self { Self { } }

          pub type MainClient = postgres::Client;

          fn select(client: &mut postgres::Client) { 
            let row = client.queryOne(r#"SELECT name FROM "Books""#);
  
            row.get("name")
          }
        }
      )
    );
  }

  #[test]
  fn fold_struct() {
    assert_eq!(
      create_synca_fold().fold_item(parse_quote!(
        struct MyStruct {
          client: tokio_postgres::Client,
          other: usize
        }
      )),
      parse_quote!(
        struct MyStruct {
          client: postgres::Client,
          other: usize
        }
      )
    );
  }

  #[test]
  fn fold_trait() {
    assert_eq!(
      create_synca_fold().fold_item(parse_quote!(
        trait MyTrait {
          type Client = tokio_postgres::Client;

          fn new() -> Self;
          async fn select() -> String;
        }
      )),
      parse_quote!(
        trait MyTrait {
          type Client = postgres::Client;

          fn new() -> Self;
          fn select() -> String;
        }
      )
    );
  }

  #[test]
  fn fold_type() {
    assert_eq!(
      create_synca_fold().fold_item(parse_quote!(
        type Client = tokio_postgres::Client;
      )),
      parse_quote!(
        type Client = postgres::Client;
      )
    );

    assert_eq!(
      create_synca_fold().fold_item(parse_quote!(
        type MyResult = Result<tokio_postgres::Client, String>;
      )),
      parse_quote!(
        type MyResult = Result<postgres::Client, String>;
      )
    );
  }

  #[test]
  fn fold_mod() {
    assert_eq!(
      create_synca_fold().fold_item(parse_quote!(
        mod my_mod {
          #[tokio::test]
          async fn my_test() {
            let a = connect().await.unwrap();

            assert_eq!(a.await.unwrap(), 42);
          }
        }
      )),
      parse_quote!(
        mod my_mod {
          #[test]
          fn my_test() {
            let a = connect().unwrap();

            assert_eq!(a.unwrap(), 42);
          }
        }
      )
    );
  }

  #[test]
  fn fold_attribute() {
    assert_eq!(
      create_synca_fold().fold_item(parse_quote!(
        #[tokio::test]
        async fn my_test() {
          let _client = connect().await.unwrap();
        }
      )),
      parse_quote!(
        #[test]
        fn my_test() {
          let _client = connect().unwrap();
        }
      )
    );
  }

  fn create_synca_fold() -> SyncAFold {
    SyncAFold { 
      types: HashMap::from([ 
        ("tokio_postgres :: Client".into(), parse_quote!(postgres::Client)),
        ("tokio_postgres :: NoTls".into(), parse_quote!(postgres::NoTls)),
      ]),
      attributes: HashMap::from([ 
        ("# [tokio :: test]".into(), parse_quote!(#[test]))
      ])
    }
  }
}