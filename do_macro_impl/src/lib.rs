extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn;
use syn::fold::{self, Fold};
use syn::Expr;
use syn::ItemFn;

pub fn do_scope_impl(tokens: TokenStream) -> TokenStream {
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
    fn replace_expr(&mut self, i: Expr) -> Expr {
        if let Expr::Macro(i) = &i {
            println!("{}", i.to_token_stream());
            if i.mac.path.is_ident("do_") {
                return Expr::Verbatim(quote! {
                    42
                });
            }
        }
        i
    }
}

impl Fold for State {
    fn fold_expr(&mut self, i: Expr) -> Expr {
        match &i {
            Expr::Block(i) => {}
            Expr::Break(i) => {}
            Expr::Closure(i) => {}
            Expr::Continue(i) => {}
            Expr::ForLoop(i) => {}
            Expr::Loop(i) => {}
            Expr::Return(i) => {}
            Expr::While(i) => {}
            _ => {}
        }
        let i = self.replace_expr(i);
        fold::fold_expr(self, i)
    }
}
