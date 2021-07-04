extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn do_scope(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    do_macro_impl::do_scope_impl(tokens)
}
