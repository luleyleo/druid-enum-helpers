#![allow(dead_code)]
#![allow(unused_variables)]
extern crate proc_macro;

use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse_macro_input;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, Path, Result, Token, Type};

struct WidgetMatch {
    subject: Path,
    branches: Vec<MatchBranch>,
}

struct MatchBranch {
    variant: Path,
    params: Vec<Type>,
    expr: Expr,
}

impl Parse for WidgetMatch {
    fn parse(input: ParseStream) -> Result<Self> {
        let subject = input.parse()?;
        input.parse::<Token![,]>()?;

        let branches = input
            .parse_terminated::<MatchBranch, Token![,]>(MatchBranch::parse)?
            .into_iter()
            .collect();

        Ok(WidgetMatch { subject, branches })
    }
}

impl Parse for MatchBranch {
    fn parse(input: ParseStream) -> Result<Self> {
        let variant = input.parse()?;

        let params = if input.peek(syn::token::Paren) {
            let types;
            syn::parenthesized!(types in input);
            Punctuated::<Type, Token![,]>::parse_separated_nonempty(&types)?
                .into_iter()
                .collect()
        } else {
            Vec::new()
        };

        input.parse::<Token![=>]>()?;

        let expr = input.parse()?;

        Ok(MatchBranch {
            variant,
            params,
            expr,
        })
    }
}

#[proc_macro_hack]
pub fn match_widget(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let wm: WidgetMatch = parse_macro_input!(input as WidgetMatch);

    let target = wm.subject;

    let branches = wm.branches.into_iter().map(|branch: MatchBranch| {
        let variant = branch.variant;
        let expr = branch.expr;
        let result = quote! {
            {
                let widget = #expr;
                let boxed: Box<dyn druid::Widget<#target>> = Box::new(widget);
                boxed
            }
        };
        if branch.params.is_empty() {
            quote! {
                #variant => #result
            }
        } else {
            let pattern = branch.params.iter().map(|_| quote!(_));
            quote! {
                #variant(#(#pattern),*) => #result
            }
        }
    });

    let output = quote! {
        match_macro::WidgetMatcher::new(|target: &#target| match target {
            #(#branches,)*
        })
    };

    proc_macro::TokenStream::from(output)
}
