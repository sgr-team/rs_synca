//! # synca
//! 
//! Write asynchronous code, and synca will create a synchronous version.
//! 
//! ## Concept
//! 
//! The crate contains one attribute macro "synca" which takes the feature expression
//! and replaceable types and attributes.
//! 
//! This macro can be applied to the declaration of structures, enums, implementations, traits, 
//! modules, functions or macros.
//! 
//! ```rust
//! #[synca::synca(
//!   feature = "tokio",
//!   tokio_postgres::Client => postgres::Client,
//!   tokio_postgres::Error => postgres::Error,
//!   #[tokio::test] => #[test],
//! )]
//! mod my_mod {
//!   type Err = tokio_postgres::Error;
//! 
//!   pub async fn select(client: &mut tokio_postgres::Client) -> Result<i32, Err> {
//!     let row = client.query_one("SELECT 1 + 2 result", &[]).await?;
//! 
//!     Ok(row.get("result"))
//!   }
//! }
//! ```
//! 
//! The macro generates the next code
//! 
//! ```rust
//! #[cfg(not(feature = "tokio"))]
//! mod my_mod {
//!   type Err = postgres::Error;
//! 
//!   pub fn select(client: &mut postgres::Client) -> Result<i32, Err> {
//!     let row = client.query_one("SELECT 1 + 2 result", &[])?;
//! 
//!     Ok(row.get("result"))
//!   }
//! }
//! 
//! #[cfg(feature = "tokio")]
//! mod my_mod {
//!   type Err = tokio_postgres::Error;
//! 
//!   pub async fn select(client: &mut tokio_postgres::Client) -> Result<i32, Err> {
//!     let row = client.query_one("SELECT 1 + 2 result", &[]).await?;
//! 
//!     Ok(row.get("result"))
//!   }
//! }
//! ```
//! 
//! ## Examples
//! 
//! ### pg_as_calc
//! 
//! The module allows to use Postgres as a calculator.
//! Contains two features: sync & tokio

mod attr;
mod fold;

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
/// The macro will create a synchronous duplicate of the code under the feature "cfg(not($features_expr))"
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
  let item_sync = Fold::fold_item(&mut attr.fold(), item.clone());
  
  quote! {
    #[cfg(#features)]
    #item

    #[cfg(not(#features))]
    #item_sync
  }.into()
}