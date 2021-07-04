use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn;
use syn::fold::{self, Fold};
use syn::Expr;
use syn::ItemFn;

pub fn do_scope_impl(tokens: TokenStream1) -> TokenStream1 {
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

#[derive(Debug)]
struct DoMacro {
    func: syn::ExprCall,
    block: syn::ExprBlock,
}

impl syn::parse::Parse for DoMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        println!("{:?}", &input);
        let r = DoMacro {
            func: input.parse()?,
            block: input.parse()?,
        };
        println!("{:?}", &r);
        Ok(r)
    }
}

struct State {
    stack: Vec<StackEntry>,
}

enum StackEntry {
    Function,
    JumpTarget { label: Option<syn::Lifetime> },
}

impl std::fmt::Debug for StackEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            StackEntry::Function => write!(f, "function"),
            StackEntry::JumpTarget { label } => write!(f, "loop {:?}", label),
        }
    }
}

impl State {
    fn new() -> Self {
        Self {
            stack: vec![StackEntry::Function],
        }
    }

    fn parse_macro(&mut self, tokens: TokenStream1) -> TokenStream1 {
        let input = syn::parse_macro_input!(tokens as DoMacro);

        let expr = input.block;
        let expanded = quote! {
            #expr
        };

        TokenStream1::from(expanded)
    }

    fn replace_expr(&mut self, i: Expr) -> Expr {
        if let Expr::Macro(i) = &i {
            println!("{:#?}", self.stack);
            println!("{}", i.to_token_stream());
            println!();
            if i.mac.path.is_ident("do_") {
                let transformed = self.parse_macro(i.mac.tokens.clone().into());
                return Expr::Verbatim(transformed.into());
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
            Expr::Closure(i) => {
                /*
                    self.stack.push(format!("Closure"));
                    pushed = true;
                */
            }

            // jumps
            Expr::Break(i) => {
                /*
                    self.stack.push(format!("Break"));
                    pushed = true;
                */
            }
            Expr::Continue(i) => {
                /*
                    self.stack.push(format!("Continue"));
                    pushed = true;
                */
            }
            Expr::Return(i) => {
                /*
                    self.stack.push(format!("Return"));
                    pushed = true;
                */
            }

            // jump targets
            Expr::Block(i) => {
                self.stack.push(StackEntry::JumpTarget {
                    label: i.label.as_ref().map(|v| v.name.clone()),
                });
                pushed = true;
            }
            Expr::ForLoop(i) => {
                self.stack.push(StackEntry::JumpTarget {
                    label: i.label.as_ref().map(|v| v.name.clone()),
                });
                pushed = true;
            }
            Expr::Loop(i) => {
                self.stack.push(StackEntry::JumpTarget {
                    label: i.label.as_ref().map(|v| v.name.clone()),
                });
                pushed = true;
            }
            Expr::While(i) => {
                self.stack.push(StackEntry::JumpTarget {
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
