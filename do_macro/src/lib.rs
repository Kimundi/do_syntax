extern crate proc_macro;

use proc_macro::TokenStream;
mod do_block;
mod do_scope;

#[proc_macro_attribute]
pub fn do_scope(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    println!("/*");
    let ret = do_scope::do_scope_impl(tokens);
    println!("*/");
    ret
}

struct State {
    stack: Vec<StackEntry>,
    function_name: syn::Ident,
}

impl State {
    fn new(function_name: syn::Ident) -> Self {
        Self {
            stack: vec![StackEntry::Function],
            function_name,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
enum StackEntry {
    Function,
    Block { label: Option<syn::Lifetime> },
    ForOrWhile { label: Option<syn::Lifetime> },
    Loop { label: Option<syn::Lifetime> },
}
