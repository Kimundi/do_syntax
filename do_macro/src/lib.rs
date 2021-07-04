extern crate proc_macro;

use proc_macro::TokenStream;
mod do_scope;

#[proc_macro_attribute]
pub fn do_scope(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    println!("/*");
    let ret = do_scope::do_scope_impl(tokens);
    println!("*/");
    ret
}
