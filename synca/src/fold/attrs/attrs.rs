use syn::{parse_quote, Attribute};

use crate::{SyncAFold, SyncAAttribute};

pub struct SyncAFoldAttributes {
  pub is_async: bool,
  pub ignored: bool,
  pub new_attrs: Vec<Attribute>,
}

impl SyncAFoldAttributes {
  pub fn new(fold: &SyncAFold, attrs: &Vec<Attribute>) -> Self {
    let mut result = SyncAFoldAttributes { 
      is_async: fold.is_async,
      ignored: false, 
      new_attrs: vec![]
    };
    let mut docs = vec![];

    for attr in attrs.into_iter() {
      let synca_attr: SyncAAttribute = attr.clone().into();
      match &synca_attr {
        SyncAAttribute::Other(x) => {
          result.new_attrs.push(
            match fold.attributes.get(x) {
              Some(n) => if fold.is_async { x.clone() } else { n.clone() },
              None => x.clone(),
            }
          );
        },
        SyncAAttribute::Cfg(x) => {
          if x != &fold.module_name {
            let cfg = &fold.cfg;
            result.new_attrs.push(parse_quote!(#[cfg(all(#cfg, not(#cfg)))]));
          }
        },
        SyncAAttribute::Doc(s) => 
          for str in s.split("\n") { 
            docs.push(str.to_string());
          },
        SyncAAttribute::Ignore => result.ignored = true,
      }
    }

    match result.docs_attribute(docs) {
      Some(x) => result.new_attrs.push(x),
      None => { }
    }

    result
  }
}

#[cfg(test)]
mod from {
  use std::collections::HashMap;

  use quote::ToTokens;
  use syn::{parse_quote, Attribute};

  use crate::{SyncAFoldAttributes, SyncAFold};

  #[test]
  fn new_attrs() {
    let attrs_async: Vec<Attribute> = vec![
      parse_quote!(#[custom]),
      parse_quote!(#[tokio::test]),
    ];
    let attrs_sync: Vec<Attribute> = vec![
      parse_quote!(#[custom]),
      parse_quote!(#[test]),
    ];

    let attrs = |is_async| {
      SyncAFoldAttributes::new(
        &SyncAFold { 
          module_name: "tokio".into(),
          is_async: is_async, 
          types: HashMap::new(), 
          attributes: HashMap::from([
            (parse_quote!(#[tokio::test]), parse_quote!(#[test]))
          ]),
          cfg: parse_quote!(feature = "tokio")
        },
        &vec![
          parse_quote!(#[custom]),
          parse_quote!(#[tokio::test]),
        ]
      )
    };

    assert_eq!(
      attrs(true)
        .new_attrs
        .iter()
        .map(|x| x.to_token_stream().to_string())
        .collect::<Vec<_>>(),
      attrs_async
        .iter()
        .map(|x| x.to_token_stream().to_string())
        .collect::<Vec<_>>()
    );
    assert_eq!(
      attrs(false)
        .new_attrs
        .iter()
        .map(|x| x.to_token_stream().to_string())
        .collect::<Vec<_>>(),
      attrs_sync
        .iter()
        .map(|x| x.to_token_stream().to_string())
        .collect::<Vec<_>>()
    );
  }

  #[test]
  fn ignored() {
    let attrs_ignored: Vec<Attribute> = vec![ parse_quote!(#[synca::ignore]), parse_quote!(#[custom]) ];
    let attrs_simple: Vec<Attribute> = vec![ parse_quote!(#[custom]) ];

    let attrs = |is_ignored| {
      SyncAFoldAttributes::new(
        &SyncAFold { 
          module_name: "tokio".into(),
          is_async: true, 
          types: HashMap::new(), 
          attributes: HashMap::from([
            (parse_quote!(#[tokio::test]), parse_quote!(#[test]))
          ]),
          cfg: parse_quote!(feature = "tokio")
        },
        if is_ignored { &attrs_ignored } else { &attrs_simple }
      )
    };

    assert_eq!(attrs(true).ignored, true);
    assert_eq!(attrs(false).ignored, false);
  }

  #[test]
  fn docs() {
    let new_attrs_async: Vec<Attribute> = vec![ 
      parse_quote!(#[custom]), 
      parse_quote!(#[doc = " # Header\n async"]),
    ];
    let new_attrs_sync: Vec<Attribute> = vec![ 
      parse_quote!(#[custom]), 
      parse_quote!(#[doc = " # Header\n sync"]),
    ];

    let attrs = |is_async| {
      SyncAFoldAttributes::new(
        &SyncAFold { 
          module_name: "sync".into(),
          is_async, 
          types: HashMap::new(), 
          attributes: HashMap::from([
            (parse_quote!(#[tokio::test]), parse_quote!(#[test]))
          ]),
          cfg: parse_quote!(feature = "sync")
        },
        &vec![ 
          parse_quote!(#[custom]), 
          parse_quote!(#[doc = " # Header"]),
          parse_quote!(#[doc = " [synca::match]async|sync[/synca::match]"]),
        ]
      )
    };

    assert_eq!(
      attrs(true).new_attrs.iter().map(|x| x.to_token_stream().to_string()).collect::<Vec<_>>(), 
      new_attrs_async.iter().map(|x| x.to_token_stream().to_string()).collect::<Vec<_>>()
    );
    assert_eq!(
      attrs(false).new_attrs.iter().map(|x| x.to_token_stream().to_string()).collect::<Vec<_>>(), 
      new_attrs_sync.iter().map(|x| x.to_token_stream().to_string()).collect::<Vec<_>>()
    );
  }
}