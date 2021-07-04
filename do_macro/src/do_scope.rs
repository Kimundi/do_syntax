use crate::StackEntry;
use crate::State;
use proc_macro::TokenStream as TokenStream1;
use quote::ToTokens;
use syn;
use syn::fold::{self, Fold};
use syn::Expr;
use syn::ItemFn;

pub fn do_scope_impl(tokens: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(tokens as Function);
    let function_name = input.item.sig.ident.clone();

    let new_item = State::new(function_name).fold_item_fn(input.item);
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

impl State {
    fn replace_expr(&mut self, i: Expr) -> Expr {
        if let Expr::Macro(i) = &i {
            // println!("{:#?}", self.stack);
            // println!("{}", i.to_token_stream());
            // println!();
            if i.mac.path.is_ident("do_") {
                return Expr::Verbatim(self.replace_macro(i.mac.tokens.clone()));
            }
        }
        i
    }
}

impl Fold for State {
    fn fold_expr(&mut self, i: Expr) -> Expr {
        let mut pushed = false;
        match &i {
            // reset of stack
            // TODO: ItemFN, ItemMethod
            Expr::Closure(_i) => {
                /*
                    self.stack.push(format!("Closure"));
                    pushed = true;
                */
            }

            // jumps
            Expr::Break(_i) => {
                /*
                    self.stack.push(format!("Break"));
                    pushed = true;
                */
            }
            Expr::Continue(_i) => {
                /*
                    self.stack.push(format!("Continue"));
                    pushed = true;
                */
            }
            Expr::Return(_i) => {
                /*
                    self.stack.push(format!("Return"));
                    pushed = true;
                */
            }

            // jump targets
            Expr::Block(i) => {
                self.stack.push(StackEntry::Block {
                    label: i.label.as_ref().map(|v| v.name.clone()),
                });
                pushed = true;
            }
            Expr::ForLoop(i) => {
                self.stack.push(StackEntry::ForOrWhile {
                    label: i.label.as_ref().map(|v| v.name.clone()),
                });
                pushed = true;
            }
            Expr::Loop(i) => {
                self.stack.push(StackEntry::Loop {
                    label: i.label.as_ref().map(|v| v.name.clone()),
                });
                pushed = true;
            }
            Expr::While(i) => {
                self.stack.push(StackEntry::ForOrWhile {
                    label: i.label.as_ref().map(|v| v.name.clone()),
                });
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

    fn fold_item_fn(&mut self, i: ItemFn) -> ItemFn {
        fold::fold_item_fn(self, i)
    }
}
