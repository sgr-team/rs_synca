use std::collections::HashMap;

use quote::ToTokens;
use syn::{fold::{self, Fold}, Expr};

use crate::SyncAFoldAttributes;

pub struct SyncAFold {
  pub is_async: bool,
  pub features: Expr,
  pub types: HashMap<syn::Type, syn::Type>,
  pub attributes: HashMap<syn::Attribute, syn::Attribute>,
}

macro_rules! impl_fold_fn {
  ($fn_name: ident, $ty: ty) => {
    fn $fn_name(&mut self, i: $ty) -> $ty {
      let attrs = SyncAFoldAttributes::new(&self, &i.attrs);
      if attrs.ignored { 
        return i;
      } 
    
      let mut new_fn = i.clone();
      new_fn.attrs = attrs.new_attrs;
      if !self.is_async {
        new_fn.sig.asyncness = None;
      }

      fold::$fn_name(self, new_fn)
    }
  };
}

macro_rules! impl_fold_attrs {
  ($fn_name: ident, $ty: ty) => {
    fn $fn_name(&mut self, i: $ty) -> $ty {
      let attrs = SyncAFoldAttributes::new(&self, &i.attrs);
      if attrs.ignored { 
        return i;
      }
      
      let mut new_i = i.clone();
      new_i.attrs = attrs.new_attrs;
      
      fold::$fn_name(self, new_i)
    }
  };
}

impl Fold for SyncAFold {
  fn fold_type(&mut self, ty: syn::Type) -> syn::Type {
    if self.is_async {
      return fold::fold_type(self, ty);
    }

    match &ty {
      syn::Type::Path(path) => {
        match self.types.get(&ty) {
          Some(new_ty) => new_ty.clone(),
          None => fold::fold_type(self, syn::Type::Path(path.clone())),
        }
      },
      _ => fold::fold_type(self, ty),
    }
  }

  fn fold_macro(&mut self, mac: syn::Macro) -> syn::Macro {
    if self.is_async {
      return fold::fold_macro(self, mac);
    }

    let macro_str = fold::fold_macro(self, mac)
      .to_token_stream()
      .to_string()
      .replace(". await", "")
      .replace(".await", "");
      
    syn::parse_str(&macro_str).unwrap()
  }

  fn fold_expr(&mut self, exp: Expr) -> Expr {
    if self.is_async {
      return fold::fold_expr(self, exp);
    }

    match exp {
      Expr::Await(e) => self.fold_expr(*e.base),
      Expr::Async(e) => self.fold_expr(Expr::Block(syn::ExprBlock {
        attrs: e.attrs,
        label: None,
        block: e.block,
      })),
      _ => fold::fold_expr(self, exp),
    }
  }

  impl_fold_fn!(fold_item_fn, syn::ItemFn);
  impl_fold_fn!(fold_impl_item_fn, syn::ImplItemFn);
  impl_fold_fn!(fold_trait_item_fn, syn::TraitItemFn);
  impl_fold_fn!(fold_foreign_item_fn, syn::ForeignItemFn);
  
  impl_fold_attrs!(fold_arm, syn::Arm);
  impl_fold_attrs!(fold_bare_fn_arg, syn::BareFnArg);
  impl_fold_attrs!(fold_bare_variadic, syn::BareVariadic);
  impl_fold_attrs!(fold_const_param, syn::ConstParam);
  impl_fold_attrs!(fold_derive_input, syn::DeriveInput);
  impl_fold_attrs!(fold_expr_array, syn::ExprArray);
  impl_fold_attrs!(fold_expr_assign, syn::ExprAssign);
  impl_fold_attrs!(fold_expr_async, syn::ExprAsync);
  impl_fold_attrs!(fold_expr_await, syn::ExprAwait);
  impl_fold_attrs!(fold_expr_binary, syn::ExprBinary);
  impl_fold_attrs!(fold_expr_block, syn::ExprBlock);
  impl_fold_attrs!(fold_expr_break, syn::ExprBreak);
  impl_fold_attrs!(fold_expr_call, syn::ExprCall);
  impl_fold_attrs!(fold_expr_cast, syn::ExprCast);
  impl_fold_attrs!(fold_expr_closure, syn::ExprClosure);
  impl_fold_attrs!(fold_expr_const, syn::ExprConst);
  impl_fold_attrs!(fold_expr_continue, syn::ExprContinue);
  impl_fold_attrs!(fold_expr_field, syn::ExprField);
  impl_fold_attrs!(fold_expr_for_loop, syn::ExprForLoop);
  impl_fold_attrs!(fold_expr_group, syn::ExprGroup);
  impl_fold_attrs!(fold_expr_if, syn::ExprIf);
  impl_fold_attrs!(fold_expr_index, syn::ExprIndex);
  impl_fold_attrs!(fold_expr_infer, syn::ExprInfer);
  impl_fold_attrs!(fold_expr_let, syn::ExprLet);
  impl_fold_attrs!(fold_expr_lit, syn::ExprLit);
  impl_fold_attrs!(fold_expr_loop, syn::ExprLoop);
  impl_fold_attrs!(fold_expr_macro, syn::ExprMacro);
  impl_fold_attrs!(fold_expr_match, syn::ExprMatch);
  impl_fold_attrs!(fold_expr_method_call, syn::ExprMethodCall);
  impl_fold_attrs!(fold_expr_paren, syn::ExprParen);
  impl_fold_attrs!(fold_expr_path, syn::ExprPath);
  impl_fold_attrs!(fold_expr_range, syn::ExprRange);
  impl_fold_attrs!(fold_expr_reference, syn::ExprReference);
  impl_fold_attrs!(fold_expr_repeat, syn::ExprRepeat);
  impl_fold_attrs!(fold_expr_return, syn::ExprReturn);
  impl_fold_attrs!(fold_expr_struct, syn::ExprStruct);
  impl_fold_attrs!(fold_expr_try, syn::ExprTry);
  impl_fold_attrs!(fold_expr_try_block, syn::ExprTryBlock);
  impl_fold_attrs!(fold_expr_tuple, syn::ExprTuple);
  impl_fold_attrs!(fold_expr_unary, syn::ExprUnary);
  impl_fold_attrs!(fold_expr_unsafe, syn::ExprUnsafe);
  impl_fold_attrs!(fold_expr_while, syn::ExprWhile);
  impl_fold_attrs!(fold_expr_yield, syn::ExprYield);
  impl_fold_attrs!(fold_field, syn::Field);
  impl_fold_attrs!(fold_field_pat, syn::FieldPat);
  impl_fold_attrs!(fold_field_value, syn::FieldValue);
  impl_fold_attrs!(fold_file, syn::File);
  impl_fold_attrs!(fold_foreign_item_macro, syn::ForeignItemMacro);
  impl_fold_attrs!(fold_foreign_item_static, syn::ForeignItemStatic);
  impl_fold_attrs!(fold_foreign_item_type, syn::ForeignItemType);
  impl_fold_attrs!(fold_impl_item_const, syn::ImplItemConst);
  impl_fold_attrs!(fold_impl_item_macro, syn::ImplItemMacro);
  impl_fold_attrs!(fold_impl_item_type, syn::ImplItemType);
  impl_fold_attrs!(fold_item_const, syn::ItemConst);
  impl_fold_attrs!(fold_item_enum, syn::ItemEnum);
  impl_fold_attrs!(fold_item_extern_crate, syn::ItemExternCrate);
  impl_fold_attrs!(fold_item_foreign_mod, syn::ItemForeignMod);
  impl_fold_attrs!(fold_item_impl, syn::ItemImpl);
  impl_fold_attrs!(fold_item_macro, syn::ItemMacro);
  impl_fold_attrs!(fold_item_mod, syn::ItemMod);
  impl_fold_attrs!(fold_item_static, syn::ItemStatic);
  impl_fold_attrs!(fold_item_struct, syn::ItemStruct);
  impl_fold_attrs!(fold_item_trait, syn::ItemTrait);
  impl_fold_attrs!(fold_item_trait_alias, syn::ItemTraitAlias);
  impl_fold_attrs!(fold_item_type, syn::ItemType);
  impl_fold_attrs!(fold_item_union, syn::ItemUnion);
  impl_fold_attrs!(fold_item_use, syn::ItemUse);
  impl_fold_attrs!(fold_lifetime_param, syn::LifetimeParam);
  impl_fold_attrs!(fold_local, syn::Local);
  impl_fold_attrs!(fold_pat_ident, syn::PatIdent);
  impl_fold_attrs!(fold_pat_or, syn::PatOr);
  impl_fold_attrs!(fold_pat_paren, syn::PatParen);
  impl_fold_attrs!(fold_pat_reference, syn::PatReference);
  impl_fold_attrs!(fold_pat_rest, syn::PatRest);
  impl_fold_attrs!(fold_pat_slice, syn::PatSlice);
  impl_fold_attrs!(fold_pat_struct, syn::PatStruct);
  impl_fold_attrs!(fold_pat_tuple, syn::PatTuple);
  impl_fold_attrs!(fold_pat_tuple_struct, syn::PatTupleStruct);
  impl_fold_attrs!(fold_pat_type, syn::PatType);
  impl_fold_attrs!(fold_pat_wild, syn::PatWild);
  impl_fold_attrs!(fold_receiver, syn::Receiver);
  impl_fold_attrs!(fold_stmt_macro, syn::StmtMacro);
  impl_fold_attrs!(fold_trait_item_const, syn::TraitItemConst);
  impl_fold_attrs!(fold_trait_item_macro, syn::TraitItemMacro);
  impl_fold_attrs!(fold_trait_item_type, syn::TraitItemType);
  impl_fold_attrs!(fold_variadic, syn::Variadic);
  impl_fold_attrs!(fold_variant, syn::Variant);
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use quote::ToTokens;
  use syn::{fold::Fold, parse_quote};

  use crate::SyncAFold;
  
  macro_rules! assert_as_str {
    (
      $fn_name: ident, 
      $ty: ty, 
      $a: expr, 
      $b_async: expr, 
      $b_sync: expr
    ) => {
      let (mut fold_async, mut fold_sync) = synca_fold();
      let b_async_typed: $ty = $b_async;
      let b_sync_typed: $ty = $b_sync;
      
      assert_eq!(
        fold_async.$fn_name($a).to_token_stream().to_string(),
        b_async_typed.to_token_stream().to_string()
      );
      assert_eq!(
        fold_sync.$fn_name($a).to_token_stream().to_string(),
        b_sync_typed.to_token_stream().to_string()
      );
    }
  }

  #[test]
  fn fold_type() {
    assert_as_str!(
      fold_type, 
      syn::Type,
      parse_quote!(std::collections::HashMap),
      parse_quote!(std::collections::HashMap),
      parse_quote!(std::collections::HashMap)
    );

    assert_as_str!(
      fold_type, 
      syn::Type,
      parse_quote!(tokio_postgres::Client),
      parse_quote!(tokio_postgres::Client),
      parse_quote!(postgres::Client)
    );

    assert_as_str!(
      fold_type, 
      syn::Type,
      parse_quote!(&mut tokio_postgres::Client),
      parse_quote!(&mut tokio_postgres::Client),
      parse_quote!(&mut postgres::Client)
    );
  }

  #[test]
  fn fold_macro() {
    assert_as_str!(
      fold_macro, 
      syn::Macro,
      parse_quote!(assert_eq!(dao.answer().await, 42)),
      parse_quote!(assert_eq!(dao.answer().await, 42)),
      parse_quote!(assert_eq!(dao.answer(), 42))
    );
  }

  #[test]
  fn fold_expr() {
    assert_as_str!(
      fold_expr, 
      syn::Expr,
      parse_quote!(dao.answer().await?.id),
      parse_quote!(dao.answer().await?.id),
      parse_quote!(dao.answer()?.id)
    );
  }

  #[test]
  fn macro_impl_fold_fn() {
    assert_as_str!(
      fold_item_fn, 
      syn::ItemFn,
      parse_quote!(fn my_fn() { }),
      parse_quote!(fn my_fn() { }),
      parse_quote!(fn my_fn() { })
    );

    assert_as_str!(
      fold_item_fn, 
      syn::ItemFn,
      parse_quote!(
        /// # FN get_name
        /// 
        /// Args
        /// - [synca::match]tokio_postgres::Client|postgres::Client[/synca::match]
        async fn get_name(client: &mut tokio_postgres::Client) -> String { 
          let row = client.query_one(r#"SELECT 'My name' "name""#, &[]).await?;

          row.get("name")
        }
      ),
      parse_quote!(
        #[doc = " # FN get_name\n \n Args\n - tokio_postgres::Client"]
        async fn get_name(client: &mut tokio_postgres::Client) -> String { 
          let row = client.query_one(r#"SELECT 'My name' "name""#, &[]).await?;

          row.get("name")
        }
      ),
      parse_quote!(
        #[doc = " # FN get_name\n \n Args\n - postgres::Client"]
        fn get_name(client: &mut postgres::Client) -> String { 
          let row = client.query_one(r#"SELECT 'My name' "name""#, &[])?;

          row.get("name")
        }
      )
    );
  }

  #[test]
  fn macro_impl_fold_attrs() {
    assert_as_str!(
      fold_item_mod, 
      syn::ItemMod,
      parse_quote!(
        /// # my_mod
        /// 
        /// - [synca::match]tokio_postgres::Client|postgres::Client[/synca::match]
        mod my_mod {
          type Client = tokio_postgres::Client;
          
          async fn name() {

          }
        }
      ),
      parse_quote!(
        #[doc = " # my_mod\n \n - tokio_postgres::Client"]
        mod my_mod {
          type Client = tokio_postgres::Client;
          
          async fn name() {

          }
        }
      ),
      parse_quote!(
        #[doc = " # my_mod\n \n - postgres::Client"]
        mod my_mod {
          type Client = postgres::Client;
          
          fn name() {

          }
        }
      )
    );
  }

  fn synca_fold() -> (SyncAFold, SyncAFold) {
    let features: syn::Expr = parse_quote!(feature = "async");
    let types: HashMap<syn::Type, syn::Type> = HashMap::from([
      (parse_quote!(tokio_postgres::Client), parse_quote!(postgres::Client)),
      (parse_quote!(tokio_postgres::NoTls), parse_quote!(postgres::NoTls)),
    ]);
    let attributes: HashMap<syn::Attribute, syn::Attribute> = HashMap::from([
      (parse_quote!(#[tokio::test]), parse_quote!(#[test])),
    ]);
    
    (
      SyncAFold {
        is_async: true,
        features: features.clone(),
        types: types.clone(),
        attributes: attributes.clone(),
      },
      SyncAFold {
        is_async: false,
        features,
        types,
        attributes,
      }
    )
  }
}
