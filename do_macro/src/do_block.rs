use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Lifetime};

use crate::StackEntry;

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

        let enum_def = self
            .compute_enum(self.function_name.clone())
            .make_jump_target_enum();

        let expr = input.block;
        let expanded = quote! {
            #enum_def
            #expr
        };

        TokenStream1::from(expanded)
    }

    pub(crate) fn replace_macro(&mut self, tokens: TokenStream) -> TokenStream {
        let transformed = self.parse_macro(tokens.into());
        transformed.into()
    }

    fn compute_enum(&self, function_name: Ident) -> JumpEnum {
        let pos = self.stack.iter().rposition(|v| *v == StackEntry::Function);

        let pos = match pos {
            Some(pos) => pos,
            None => return JumpEnum::new(function_name),
        };

        let stack = &self.stack[pos..];
        println!("{:#?}", stack);

        let mut targets = Vec::new();
        let mut last_break = None;
        let mut last_continue = None;
        let mut id = 0;
        for e in stack.iter() {
            match e {
                StackEntry::Function => {
                    targets.push(DispatchTargets::Return(id));
                }
                StackEntry::Loop { label } => {
                    last_break = Some(DispatchTargets::BreakValue(id));
                    last_continue = Some(DispatchTargets::Continue(id));
                    if let Some(label) = label {
                        targets.extend([
                            DispatchTargets::BreakValueLabel(id, label.clone()),
                            DispatchTargets::ContinueLabel(id, label.clone()),
                        ]);
                    }
                }
                StackEntry::ForOrWhile { label } => {
                    last_break = Some(DispatchTargets::Break(id));
                    last_continue = Some(DispatchTargets::Continue(id));
                    if let Some(label) = label {
                        targets.extend([
                            DispatchTargets::BreakLabel(id, label.clone()),
                            DispatchTargets::ContinueLabel(id, label.clone()),
                        ]);
                    }
                }
                StackEntry::Block { label } => {
                    if let Some(label) = label {
                        targets.push(DispatchTargets::BreakLabel(id, label.clone()));
                    }
                }
            }
            id += 1;
        }

        if let Some(target) = last_break {
            targets.push(target);
        }
        if let Some(target) = last_continue {
            targets.push(target);
        }

        JumpEnum {
            targets,
            type_count: id,
            function_name,
        }
    }
}

type TypeId = usize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
enum DispatchTargets {
    Return(TypeId),

    Break(TypeId),
    BreakLabel(TypeId, Lifetime),

    BreakValue(TypeId),
    BreakValueLabel(TypeId, Lifetime),

    Continue(TypeId),
    ContinueLabel(TypeId, Lifetime),
}

struct JumpEnum {
    targets: Vec<DispatchTargets>,
    type_count: usize,
    function_name: Ident,
}

impl JumpEnum {
    fn new(function_name: Ident) -> Self {
        Self {
            targets: Vec::new(),
            type_count: 0,
            function_name,
        }
    }
    fn make_jump_target_enum(&self) -> TokenStream {
        println!("{:#?}", self.targets);

        let mut type_args = Vec::new();
        for i in 0..self.type_count {
            type_args.push(format_ident!("T_{}", i));
        }
        println!("{:#?}", type_args);

        let mut variants = Vec::new();
        for target in &self.targets {
            let s = match target {
                DispatchTargets::Return(id) => {
                    let variant = format_ident!("Return");
                    let ty = &type_args[*id];
                    quote! {
                        #variant(#ty)
                    }
                }
                DispatchTargets::Break(_id) => {
                    let variant = format_ident!("Break");
                    quote! {
                        #variant
                    }
                }
                DispatchTargets::BreakValue(id) => {
                    let variant = format_ident!("BreakValue");
                    let ty = &type_args[*id];
                    quote! {
                        #variant(#ty)
                    }
                }
                DispatchTargets::BreakLabel(_id, l) => {
                    let variant = format_ident!("Break_{}", l.ident);
                    quote! {
                        #variant
                    }
                }
                DispatchTargets::BreakValueLabel(id, l) => {
                    let variant = format_ident!("BreakValue_{}", l.ident);
                    let ty = &type_args[*id];
                    quote! {
                        #variant(#ty)
                    }
                }
                DispatchTargets::Continue(_id) => {
                    let variant = format_ident!("Continue");
                    quote! {
                        #variant
                    }
                }
                DispatchTargets::ContinueLabel(_id, l) => {
                    let variant = format_ident!("Continue_{}", l.ident);
                    quote! {
                        #variant
                    }
                }
            };
            variants.push(s);
        }
        println!("{:#?}", variants);

        let enum_name = format_ident!("JumpTarget_{}", self.function_name);
        println!("{:?}", enum_name);

        let enum_def = quote! {
            #[allow(non_camel_case_types)]
            enum #enum_name<#(#type_args),*> {
                #(#variants,)*
            }
        };
        println!("{:#?}", enum_def);
        enum_def
    }
}
