use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

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

impl crate::State {
    fn parse_macro(&mut self, tokens: TokenStream1) -> TokenStream1 {
        let input = syn::parse_macro_input!(tokens as DoMacro);

        let expr = input.block;
        let expanded = quote! {
            #expr
        };

        TokenStream1::from(expanded)
    }

    pub(crate) fn replace_macro(&mut self, tokens: TokenStream) -> TokenStream {
        let transformed = self.parse_macro(tokens.into());
        transformed.into()
    }
}
