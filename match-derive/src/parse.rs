use heck::SnakeCase;
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Data, DataStruct, DataUnion, DeriveInput, Error, Field, Fields, FieldsUnnamed,
    Generics, Ident, Path, Result, Token,
};

#[derive(Debug)]
pub struct MatcherDerive {
    pub name: Ident,
    pub generics: Generics,
    pub variants: Vec<MatcherVariant>,
}

#[derive(Debug)]
pub struct MatcherVariant {
    pub builder_name: Option<Ident>,
    pub name: Ident,
    pub field: Field,
}

impl MatcherVariant {
    pub fn resolve_builder_name(&self) -> Ident {
        self.builder_name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| snakify(&self.name))
    }
}

impl Parse for MatcherDerive {
    fn parse(input: ParseStream) -> Result<Self> {
        let input: DeriveInput = input.parse()?;
        let name = input.ident;
        let generics = input.generics;
        let data = match input.data {
            Data::Enum(data) => Ok(data),
            Data::Struct(DataStruct { struct_token, .. }) => enum_error(struct_token.span),
            Data::Union(DataUnion { union_token, .. }) => enum_error(union_token.span),
        }?;
        let mut variants = Vec::new();
        for variant in data.variants {
            let variant_name = variant.ident;
            let name_span = variant_name.span();
            let field = match variant.fields {
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => match iter_get_one(unnamed) {
                    Some(f) => f,
                    None => return variant_error(name_span),
                },
                _ => return variant_error(name_span),
            };
            let attrs = MatcherAttrs::parse(variant.attrs)?;
            variants.push(MatcherVariant {
                builder_name: attrs.builder_name,
                name: variant_name,
                field,
            });
        }
        Ok(MatcherDerive {
            name,
            generics,
            variants,
        })
    }
}

fn enum_error<T>(span: Span) -> Result<T> {
    Err(Error::new(span, "only `enum`s can implement `Matcher`"))
}

fn variant_error<T>(span: Span) -> Result<T> {
    Err(Error::new(
        span,
        "the variant's data must be a single unnamed field",
    ))
}

#[derive(Debug, Default)]
struct MatcherAttrs {
    /// The name of the function call to build the corresponding widget.
    builder_name: Option<Ident>,
}

impl MatcherAttrs {
    fn parse(attrs: Vec<Attribute>) -> Result<Self> {
        let mut matcher_attrs = MatcherAttrs::default();
        for attr in attrs {
            // filter out attributes not called `matches`
            if !matches_path(attr.path.clone()) {
                continue;
            }
            let attrs: Punctuated<MatcherAttr, Token![,]> =
                attr.parse_args_with(Punctuated::parse_terminated)?;

            for attr in attrs {
                match attr {
                    MatcherAttr::BuilderName(builder_name) => {
                        matcher_attrs.builder_name = Some(builder_name)
                    }
                }
            }
        }
        Ok(matcher_attrs)
    }
}

#[derive(Debug)]
enum MatcherAttr {
    BuilderName(Ident),
}

impl Parse for MatcherAttr {
    fn parse(s: ParseStream) -> Result<Self> {
        let attr_name = s.parse::<Ident>()?;
        match attr_name.to_string().as_str() {
            "builder_name" => {
                s.parse::<Token![=]>()?;
                s.parse().map(MatcherAttr::BuilderName)
            }
            other => Err(Error::new(
                attr_name.span(),
                format!("expected `builder_name`, found `{}`", other),
            )),
        }
    }
}

/// True if the path is exactly `matches`.
fn matches_path(p: Path) -> bool {
    if p.leading_colon.is_some() {
        return false;
    }
    let segment = match iter_get_one(p.segments) {
        Some(s) => s,
        None => return false,
    };
    segment.ident.to_string() == "matcher" && segment.arguments.is_empty()
}

fn iter_get_one<T, I: IntoIterator<Item = T>>(iter: I) -> Option<T> {
    let mut iter = iter.into_iter();
    let result = iter.next()?;
    if iter.next().is_some() {
        return None;
    }
    Some(result)
}

fn snakify(input: &Ident) -> Ident {
    Ident::new(&input.to_string().to_snake_case(), input.span())
}
