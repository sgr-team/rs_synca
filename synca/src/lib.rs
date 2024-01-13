mod attr;
mod fold;

use proc_macro::TokenStream;
use syn::{parse_macro_input, fold::Fold, Item};
use quote::quote;

use crate::attr::SyncAAttribute;

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