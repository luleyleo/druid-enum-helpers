use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse_macro_input;

mod parse;

use parse::MatcherDerive;

#[proc_macro_derive(Matcher, attributes(matcher))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MatcherDerive);

    let name = &input.name;
    let matcher_name = Ident::new(&format!("{}Matcher", name), Span::call_site());

    let struct_fields = input.variants.iter().map(|variant| {
        let builder_name = variant.resolve_builder_name();
        let variant_ty = &variant.field.ty;
        quote!(#builder_name: Option<Box<dyn ::druid::Widget<#variant_ty>>>)
    });

    let struct_defaults = input.variants.iter().map(|variant| {
        let builder_name = variant.resolve_builder_name();
        quote!(#builder_name: None)
    });

    let builder_fns = input.variants.iter().map(|variant| {
        let builder_name = variant.resolve_builder_name();
        let variant_ty = &variant.field.ty;
        quote! {
            fn #builder_name(mut self, widget: impl ::druid::Widget<#variant_ty> + 'static) -> Self {
                self.#builder_name = Some(Box::new(widget));
                self
            }
        }
    });

    let event_match = input.variants.iter().map(|variant| {
        let builder_name = variant.resolve_builder_name();
        let variant_name = &variant.name;
        quote! {
            #name::#variant_name(inner) => match &mut self.#builder_name {
                Some(widget) => widget.event(ctx, event, inner, env),
                None => (),
            }
        }
    });

    let lifecycle_match = input.variants.iter().map(|variant| {
        let builder_name = variant.resolve_builder_name();
        let variant_name = &variant.name;
        quote! {
            #name::#variant_name(inner) => match &mut self.#builder_name {
                Some(widget) => widget.lifecycle(ctx, event, inner, env),
                None => (),
            }
        }
    });

    let update_match = input.variants.iter().map(|variant| {
        let builder_name = variant.resolve_builder_name();
        let variant_name = &variant.name;
        todo!()
    });

    let layout_match = input.variants.iter().map(|variant| {
        let builder_name = variant.resolve_builder_name();
        let variant_name = &variant.name;
        quote! {
            #name::#variant_name(inner) => match &mut self.#builder_name {
                Some(widget) => widget.layout(ctx, bc, inner, env),
                None => bc.min(),
            }
        }
    });

    let paint_match = input.variants.iter().map(|variant| {
        let builder_name = variant.resolve_builder_name();
        let variant_name = &variant.name;
        quote! {
            #name::#variant_name(inner) => match &mut self.#builder_name {
                Some(widget) => widget.paint(ctx, inner, env),
                None => (),
            }
        }
    });

    let output = quote! {
        impl #name {
            pub fn matcher() -> #matcher_name {
                #matcher_name::new()
            }
        }

        struct #matcher_name {
            #(#struct_fields,)*
        }

        impl #matcher_name {
            fn new() -> Self {
                Self {
                    #(#struct_defaults,)*
                }
            }
            #(#builder_fns)*
        }

        impl ::druid::Widget<#name> for #matcher_name {
            fn event(
                &mut self,
                ctx: &mut ::druid::EventCtx,
                event: &::druid::Event,
                data: &mut #name,
                env: &::druid::Env
            ) {
                match data {
                    #(#event_match)*
                }
            }
            fn lifecycle(
                &mut self,
                ctx: &mut ::druid::LifeCycleCtx,
                event: &::druid::LifeCycle,
                data: &#name,
                env: &::druid::Env
            ) {
                match data {
                    #(#lifecycle_match)*
                }
            }
            fn update(&mut self,
                ctx: &mut ::druid::UpdateCtx,
                old_data: &#name,
                data: &#name,
                env: &::druid::Env
            ) {
                todo!()
            }
            fn layout(
                &mut self,
                ctx: &mut ::druid::LayoutCtx,
                bc: &::druid::BoxConstraints,
                data: &#name,
                env: &::druid::Env
            ) -> ::druid::Size {
                match data {
                    #(#layout_match)*
                }
            }
            fn paint(&mut self, ctx: &mut ::druid::PaintCtx, data: &#name, env: &::druid::Env) {
                match data {
                    #(#paint_match)*
                }
            }
        }
    };
    output.into()
}
