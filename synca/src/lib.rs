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
//!   #[cfg(feature = "tokio")]
//!   pub mod tokio { },
//!   #[cfg(feature = "sync")]
//!   pub mod sync { 
//!     sync!();
//!     replace!(
//!       tokio_postgres::Client => postgres::Client,
//!       tokio_postgres::Error => postgres::Error,
//!       #[tokio::test] => #[test]
//!     );
//!   }
//! )] 
//! mod my_mod { 
//!   struct MyStruct {
//!     client: tokio_postgres::Client
//!   } 
//!   
//!   pub async fn get_name(client: &mut tokio_postgres::Client) -> String {
//!     let row = self.client.query_one("SQL", &[]).await.unwrap();
//! 
//!     row.get("name")
//!   }
//! 
//!   #[cfg(test)]
//!   mod tests {
//!     use super::get_name;
//! 
//!     #[tokio::test]
//!     pub async fn get_name_test() {
//!       assert_eq!(get_name(&mut client()).await, "My name");
//!     }
//! 
//!     fn client() -> tokio_postgres::Client { todo!(); }
//!   }
//! }
//! ```
//! 
//! Generated code
//! 
//! ```rust
//! #[cfg(feature = "tokio")]
//! pub mod tokio { 
//!   // Copy of my_mod content
//! }
//! 
//! #[cfg(feature = "sync")]
//! mod sync { 
//!   struct MyStruct {
//!     client: postgres::Client
//!   } 
//!   
//!   pub fn get_name(client: &mut postgres::Client) -> String {
//!     let row = self.client.query_one("SQL", &[]).unwrap();
//! 
//!     row.get("name")
//!   }
//! 
//!   #[cfg(test)]
//!   mod tests {
//!     use super::get_name;
//! 
//!     #[test]
//!     pub async fn get_name_test() {
//!       assert_eq!(get_name(&mut client()), "My name");
//!     }
//! 
//!     fn client() -> postgres::Client { todo!(); }
//!   }
//! }
//! ```

mod fold;
mod replace;
mod synca;

pub(crate) use fold::*;

use proc_macro::TokenStream;
use syn::{fold::Fold, parse_macro_input};
use quote::quote;
use synca::SyncA;


/// # Macro synca::synca
/// 
/// The macro creates copies of the module, as described.
/// 
/// The macro argument is a comma-separated description of the modules.
/// Only modifier macros are allowed in the module content. There are two of them: 
/// 
/// - sync!() - turns the module code into synchronous code, 
/// - replace!(my_async_type => my_sync_type) - allows you to replace types and attributes
/// 
/// ## Example
/// 
/// ```rust
/// #[synca::synca(
///   #[cfg(feature = "tokio")]
///   pub mod tokio { },
///   #[cfg(feature = "sync")]
///   pub mod sync { 
///     sync!();
///     replace!(
///       tokio_postgres::Client => postgres::Client,
///       tokio_postgres::Error => postgres::Error,
///       #[tokio::test] => #[test]
///     );
///   }
/// )] 
/// mod my_mod { 
///   struct MyStruct {
///     client: tokio_postgres::Client
///   } 
///   
///   pub async fn get_name(client: &mut tokio_postgres::Client) -> String {
///     let row = self.client.query_one("SQL", &[]).await.unwrap();
/// 
///     row.get("name")
///   }
/// 
///   #[cfg(test)]
///   mod tests {
///     use super::get_name;
/// 
///     #[tokio::test]
///     pub async fn get_name_test() {
///       assert_eq!(get_name(&mut client()).await, "My name");
///     }
/// 
///     fn client() -> tokio_postgres::Client { todo!(); }
///   }
/// }
/// ```
#[proc_macro_attribute]
pub fn synca(attr: TokenStream, input: TokenStream) -> TokenStream {
  let mut sa:SyncA = parse_macro_input!(attr);
  let item_mod: syn::ItemMod = parse_macro_input!(input);

  let mut modules = vec![];
  for (_, module) in sa.modules.iter_mut() {
    let mut new_module = module.item_mod.clone();
    new_module.content = item_mod.content.clone();

    modules.push(module.fold.fold_item_mod(new_module));
  }

  quote! { #(#modules)* }.into()
}


#[proc_macro_attribute]
pub fn ignore(_attr: TokenStream, input: TokenStream) -> TokenStream {
  input
}