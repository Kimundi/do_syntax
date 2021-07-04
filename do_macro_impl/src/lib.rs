extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn;
use syn::fold::{self, Fold};
use syn::Expr;
use syn::ItemFn;

pub fn do_scope_impl(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as Function);

    let new_item = State::new().fold_item_fn(input.item);
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

struct State {
    stack: Vec<String>,
}

impl State {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn replace_expr(&mut self, i: Expr) -> Expr {
        if let Expr::Macro(i) = &i {
            println!("--------");
            println!("{:#?}", self.stack);
            println!("{}", i.to_token_stream());
            println!("--------");
            println!();
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
        let mut pushed = false;
        match &i {
            Expr::Block(i) => {
                self.stack.push(format!("Block"));
                pushed = true;
            }
            Expr::Break(i) => {
                self.stack.push(format!("Break"));
                pushed = true;
            }
            Expr::Closure(i) => {
                self.stack.push(format!("Closure"));
                pushed = true;
            }
            Expr::Continue(i) => {
                self.stack.push(format!("Continue"));
                pushed = true;
            }
            Expr::ForLoop(i) => {
                self.stack.push(format!("ForLoop"));
                pushed = true;
            }
            Expr::Loop(i) => {
                self.stack.push(format!("Loop"));
                pushed = true;
            }
            Expr::Return(i) => {
                self.stack.push(format!("Return"));
                pushed = true;
            }
            Expr::While(i) => {
                self.stack.push(format!("While"));
                pushed = true;
            }
            _ => {}
        }
        let i = self.replace_expr(i);
        let ret = fold::fold_expr(self, i);
        if pushed {
            self.stack.pop().unwrap();
        }
        ret
    }
}
