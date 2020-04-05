#![allow(dead_code)]
#![allow(unused_variables)]
extern crate proc_macro;

use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse_macro_input;

use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Result, Token};

struct WidgetMatch {
    subject: Ident,
    branches: Vec<MatchBranch>,
}

struct MatchBranch {
    variant: Ident,
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
        input.parse::<Token![=>]>()?;
        let expr = input.parse()?;
        Ok(MatchBranch { variant, expr })
    }
}

#[proc_macro_hack]
pub fn match_widget(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let wm = parse_macro_input!(input as WidgetMatch);

    let output = quote! {
        ()
    };

    proc_macro::TokenStream::from(output)
}
