extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn;
use syn::fold::{self, Fold};
use syn::ItemFn;

#[proc_macro_attribute]
pub fn do_scope(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as Function);

    let new_item = State {}.fold_item_fn(input.item);
    new_item.to_token_stream().into()
}

struct Function {
    item: ItemFn,
}

impl syn::parse::Parse for Function {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(Function {
            item: input.parse()?,
        })
    }
}

struct State {}

impl State {
    fn replace_expr(&mut self, i: syn::Expr) -> syn::Expr {
        if let syn::Expr::Macro(i) = &i {
            println!("{}", i.to_token_stream());
            if i.mac.path.is_ident("do_") {
                return syn::Expr::Verbatim(quote! {
                    42
                });
            }
        }
        i
    }
}

impl Fold for State {
    fn fold_expr(&mut self, i: syn::Expr) -> syn::Expr {
        let i = self.replace_expr(i);
        fold::fold_expr(self, i)
    }
}
