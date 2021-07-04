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
}

impl State {
    fn new() -> Self {
        Self {
            stack: vec![StackEntry::Function],
        }
    }
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
