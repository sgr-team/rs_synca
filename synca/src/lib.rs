//! # synca
//! 
//! Write asynchronous code, and synca will create a synchronous version.
//! 
//! Macro synca::synca can be applied to the declaration of structures, enums, implementations, traits, 
//! modules, functions, macros, etc.
//! 
//! The macro generates 2 versions of code: async with attribute #[cfg(feature)] and 
//! sync with attribute #[cfg(not(feature))].
//! 
//! ## Example
//! 
//! ```
//! #[synca::synca(
//!   feature = "tokio",
//!   tokio_postgres::Client => postgres::Client,
//!   #[tokio::test] => #[test],
//! )] 
//! mod my_mod { 
//!   /// MyStruct docs
//!   /// [synca:sync]
//!   /// Sync comment
//!   /// [/synca:sync]
//!   /// [synca:async]
//!   /// Async comment
//!   /// [/synca:async]
//!   /// 
//!   /// This struct use [synca:match]tokio_postgres::Client|postgres::Client[/synca:match]
//!   pub struct MyStruct {
//!     client: tokio_postgres::Client
//!   }
//! 
//!   impl MyStruct {
//!     async fn query(&mut self) -> i32 {
//!       let row = self.client.query_one("SQL", &[]).await.unwrap();
//!       
//!       row.get("result")
//!     }
//!   }
//! }
//! ```
mod attr;
mod docs;
mod fold_async;
mod fold_sync;

use fold_async::AsyncFold;
use proc_macro::TokenStream;
use syn::{parse_macro_input, fold::Fold, Item};
use quote::quote;

use crate::attr::SyncAAttribute;

/// ## About
/// 
/// ```no_run
/// #[synca::synca(
///   feature = "my_async_feature",
///   async_type => sync_type,
///   #[async_attribute] => #[sync_attribute],
/// )] 
/// mod my_mod { }
/// ```
/// 
/// This macro can be applied to the declaration of structures, enums, implementations, traits, 
/// modules, functions, types, macros and etc. 
/// 
/// The macro will create a synchronous duplicate of the code, enabled from the feature "cfg(not($features_expr))"
/// 
/// ## Example
/// ```rust
/// #[synca::synca(
///   feature = "tokio",
///   tokio_postgres::Client => postgres::Client,
///   tokio_postgres::Error => postgres::Error,
///   #[tokio::test] => #[test],
/// )]
/// mod my_mod {
///   type Err = tokio_postgres::Error;
/// 
///   pub async fn select(client: &mut tokio_postgres::Client) -> Result<i32, Err> {
///     let row = client.query_one("SELECT 1 + 2 result", &[]).await?;
/// 
///     Ok(row.get("result"))
///   }
/// }
/// ```
#[proc_macro_attribute]
pub fn synca(attr: TokenStream, input: TokenStream) -> TokenStream {
  let attr = parse_macro_input!(attr as SyncAAttribute);
  let features = attr.features.clone();
  
  let item: Item = parse_macro_input!(input);
  
  let item_async = Fold::fold_item(&mut AsyncFold::new(), item.clone());
  let item_sync = Fold::fold_item(&mut attr.fold(), item.clone());
  
  quote! {
    #[cfg(#features)]
    #item_async

    #[cfg(not(#features))]
    #item_sync
  }.into()
}