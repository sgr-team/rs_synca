use std::collections::HashMap;

use quote::ToTokens;
use syn::{punctuated::Punctuated, Expr, Token};

use crate::{replace::Replace, SyncAFold};

#[derive(Debug, PartialEq)]
pub struct SyncA {
  pub modules: HashMap<String, SyncAModule>
}

#[derive(Debug, PartialEq)]
pub struct SyncAModule {
  pub cfg: syn::Expr,
  pub item_mod: syn::ItemMod,
  pub fold: SyncAFold
}

impl syn::parse::Parse for SyncA {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut synca = SyncA { modules: HashMap::new() };

    let parsed = Punctuated::<syn::ItemMod, Token![,]>::parse_terminated(input)?;
    for mod_desc in parsed.iter() {
      synca.modules.insert(mod_desc.ident.clone().to_string(), mod_desc.into());
    }

    Ok(synca)
  }
}

impl From<&syn::ItemMod> for SyncAModule {
  fn from(value: &syn::ItemMod) -> Self {
    let mut item_mod = value.clone();
    if let Some(x) = &mut item_mod.content {
      x.1.clear();
    }

    let mut expr_cfg: Option<Expr> = None;
    for attr in value.attrs.iter() {
      if attr.path().is_ident("cfg") {
        expr_cfg = Some(attr.parse_args().unwrap())
      }
    }
    let cfg = match expr_cfg {
      Some(x) => x,
      None => panic!("Module {} without cfg attribute", item_mod.ident.to_string())
    };
    
    let mut fold = SyncAFold {
      module_name: item_mod.ident.to_token_stream().to_string(),
      is_async: true,
      types: HashMap::new(),
      attributes: HashMap::new(),
      cfg: cfg.clone()
    };

    for content in value.content.clone().map(|x| x.1).unwrap_or(vec![]).iter() {
      match content {
        syn::Item::Macro(m) => {
          let macro_path = m.mac.path.clone().to_token_stream().to_string();

          if m.mac.path.is_ident("sync") {
            fold.is_async = false;
            continue;
          }

          if m.mac.path.is_ident("replace") {
            Replace::new(&mut fold).apply(m).unwrap();
            continue;
          }

          panic!(
            "Unhandled module item {}: supported only sync! and replace! macro\n\n  More about it: https://synca.sgr-team.dev/usage/index.html \n\n", 
            macro_path, 
          );          
        },
        _ => panic!(
          "Unhandled module item {}: supported only sync! and replace! macro\n\n  More about it: https://synca.sgr-team.dev/usage/index.html \n\n", 
          content.clone().to_token_stream().to_string(), 
        )
      }
    }

    SyncAModule { cfg, item_mod, fold }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use syn::parse_quote;

  use crate::SyncAFold;
  use super::{SyncA, SyncAModule};

  #[test]
  pub fn main() {
    let synca: SyncA = syn::parse_quote!(
      #[cfg(feature = "tokio")]
      mod my_mod_tokio { },
      #[cfg(feature = "sync")]
      mod my_mod_sync { 
        sync!();
        replace!(
          tokio_postgres::Client => postgres::Client,
          tokio_postgres::Error => postgres::Error,
          #[tokio::test] => #[test],
        );
      }
    );
    assert_eq!(
      synca,
      SyncA {
        modules: HashMap::from([
          (
            "my_mod_tokio".into(),
            SyncAModule { 
              cfg: parse_quote!(feature = "tokio"),
              item_mod: parse_quote!(#[cfg(feature = "tokio")] mod my_mod_tokio { }), 
              fold: SyncAFold { 
                module_name: "my_mod_tokio".into(), 
                is_async: true, 
                types: HashMap::new(), 
                attributes: HashMap::new(),
                cfg: parse_quote!(feature = "tokio")
              } 
            }
          ),
          (
            "my_mod_sync".into(),
            SyncAModule { 
              cfg: parse_quote!(feature = "sync"),
              item_mod: parse_quote!(#[cfg(feature = "sync")] mod my_mod_sync { }), 
              fold: SyncAFold { 
                module_name: "my_mod_sync".into(), 
                is_async: false, 
                types: HashMap::from([
                  (parse_quote!(tokio_postgres::Client), parse_quote!(postgres::Client)),
                  (parse_quote!(tokio_postgres::Error), parse_quote!(postgres::Error)),
                ]), 
                attributes: HashMap::from([
                  (parse_quote!(#[tokio::test]), parse_quote!(#[test])),
                ]),
                cfg: parse_quote!(feature = "sync")
              } 
            }
          )
        ])
      }
    )
  }
}