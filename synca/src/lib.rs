//! # SyncA
//!
//! ## Docs
//! 
//! [SyncA Book](https://synca.sgr-team.dev)
//! 
//! ## Example
//! 
//! ```rust
//! #[synca::synca(
//!   feature = "my_async_feature",
//!   async_type => sync_type,
//!   #[async_attribute] => #[sync_attribute],
//! )] 
//! mod my_mod { }
//! ```
//! 
//! This macro can be applied to the declaration of structures, enums, implementations, traits, 
//! modules, functions, types, macros and etc. 
//! 
//! The macro will create a synchronous duplicate of the code, enabled from the feature "cfg(not($features_expr))"
//! 
//! ```rust
//! /// The macro will create 2 versions of the my_mod
//! /// With the asynchrony feature enabled - asynchronous version, 
//! /// without it - synchronous.
//! /// #[synca::synca(feature = "async")]
//! /// mod my_mod { }
//! 
//! #[synca::synca(
//!   feature = "async",
//!   tokio_postgres::Client => postgres::Client
//! )]
//! struct MyStruct {
//!   client: tokio_postgres::Client
//! } 
//! 
//! /// # Functions
//! /// 
//! /// The macro turns asynchronous functions into synchronous 
//! /// implementations - removing all the .await calls
//! #[synca::synca(feature = "async")]
//! impl MyStruct {
//!   pub async fn first_name(&mut self) -> String {
//!     let row = self.client.query_one("SQL", &[]).await.unwrap();
//! 
//!     row.get("name")
//!   }
//! }
//! 
//! /// # Types
//! /// 
//! /// synca supports type substitution 
//! /// The synchronous version will use type postgres::Client
//! #[synca::synca(
//!   feature = "async",
//!   tokio_postgres::Client => postgres::Client
//! )]
//! struct MyStructTypes {
//!   client: tokio_postgres::Client
//! } 
//! 
//! /// # Tests
//! /// 
//! /// synca support attributes substitution too.
//! /// This allows you to test not only the asynchronous version, but also the synchronous one
//! #[synca::synca(
//!   feature = "async",
//!   tokio_postgres::Client => postgres::Client,
//!   #[tokio::test] => #[test]
//! )]
//! mod tests {
//!   #[tokio::test]
//!   pub async fn my_test() {
//!   }
//! }
//! 
//! /// # Traits
//! ///
//! /// synca also knows how to work with traits
//! #[synca::synca(
//!   feature = "async",
//!   tokio_postgres::Client => postgres::Client,
//! )]
//! #[async_trait::async_trait]
//! pub trait MyTrait { 
//!   async fn get_name(client: &mut tokio_postgres::Client);
//! }
//! 
//! /// # Docs
//! /// 
//! /// synca also contains a documentation processor that allows to generate 
//! /// different documentation for the synchronous and asynchronous versions
//! /// 
//! /// [synca::sync]
//! /// It's sync doc
//! /// [/synca::sync]
//! /// [synca::async]
//! /// It's async doc
//! /// [/synca::async]
//! /// 
//! /// # Match
//! /// 
//! /// You can also use "synca::match" to replace part of a string.
//! /// 
//! /// Arguments 
//! /// - [synca::match]tokio_postgres|postgres[/synca::match]::Client - postgres client
//! #[synca::synca(
//!   feature = "async",
//!   tokio_postgres::Client => postgres::Client,
//! )]
//! pub async fn my_fn(client: &mut tokio_postgres::Client) { }
//! 
//! /// # Virtual attributes
//! /// 
//! /// - synca::ignore - ignore in code fold
//! /// - synca::sync_only - the code will only be available in the synchronous version
//! /// - synca::async_only - the code will only be available in the asynchronous version
//! #[synca::synca(
//!   feature = "async",
//!   tokio_postgres::Client => postgres::Client,
//!   #[tokio::test] => #[test],
//! )]
//! mod my_mod {
//!   #[tokio::test]
//!   async fn my_test() { 
//!     #[synca::ignore]
//!     assert_eq!(format!(".aw{}", "ait"), ".await");
//!   }
//! 
//!   #[test]
//!   #[synca::sync_only]
//!   fn sync_test() { 
//!   }
//! 
//!   #[test]
//!   #[synca::async_only]
//!   fn async_test() { 
//!   }
//! }
//! ```
mod fold;
mod input;

pub(crate) use fold::*;

use proc_macro::TokenStream;
use syn::{fold::Fold, parse_macro_input, Item};
use quote::quote;

use crate::input::SyncAInput;

/// Macro for generating a synchronous version of the code.
/// 
/// Can be applied to any item of the rust language.
/// 
/// # Example
/// 
/// ```no_run
/// #[synca::synca(
///   feature = "my_async_feature",
///   tokio_postgres::Client => postgres::Client,
///   tokio_postgres::Error => postgres::Error,
///   #[tokio::test] => #[test],
/// )] 
/// mod my_mod { 
///   struct MyStructFn {
///     pub client: tokio_postgres::Client
///   }
/// 
///   #[cfg(test)]
///   mod tests {
///     #[tokio::test]
///     async fn my_test() {
///     }
///   }
/// }
/// ```
#[proc_macro_attribute]
pub fn synca(attr: TokenStream, input: TokenStream) -> TokenStream {
  let attr = parse_macro_input!(attr as SyncAInput);
  let item = parse_macro_input!(input as Item);

  let features = attr.features.clone();
  let mut fold = attr.fold();
  
  let item_async = fold.fold_item(item.clone());
  
  fold.is_async = false;
  let item_sync = fold.fold_item(item);
  
  quote! {
    #[cfg(#features)]
    #item_async

    #[cfg(not(#features))]
    #item_sync
  }.into()
}

/// Allows you to ignore a marked item.
/// 
/// Can be applied to any item of the rust language.
/// 
/// ```rust
/// #[synca::synca(
///   feature = "tokio",
///   #[tokio::test] => #[test],
/// )] 
/// mod tests {
///   #[tokio::test]
///   pub async fn my_test() { 
///     #[synca::ignore]
///     assert_eq!(format!(".aw{}", "ait"), ".await");
///   }
/// }
#[proc_macro_attribute]
pub fn ignore(_attr: TokenStream, input: TokenStream) -> TokenStream {
  input
}

/// Allows you to leave a marked item in only one of the code versions.
/// 
/// Can be applied to any item of the rust language.
/// 
/// ```rust
/// #[synca::synca(feature = "tokio")] 
/// mod my_mod {
///   #[synca::only(async)]
///   pub async fn connect_async() { 
///     // async code here
///   }
/// 
///   #[synca::only(sync)]
///   pub fn connect_sync() { 
///     // sync code here
///   }
/// }
/// ```
#[proc_macro_attribute]
pub fn only(attr: TokenStream, input: TokenStream) -> TokenStream {
  let ident: syn::Ident = syn::parse(attr).unwrap();
  let ident_async: syn::Ident = syn::parse_quote!(async);
  let ident_sync: syn::Ident = syn::parse_quote!(sync);
  
  if ident != ident_async && ident != ident_sync {
    panic!("[synca::only] unhandled ident: allowed sync or async")
  }

  input
}