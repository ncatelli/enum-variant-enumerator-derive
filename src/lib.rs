use std::str::FromStr;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DataEnum, DeriveInput, Fields, Ident};

struct VariantMetadata {
    _span: Span,
    ident: Ident,
}

impl VariantMetadata {
    fn new(span: Span, ident: Ident) -> Self {
        Self { _span: span, ident }
    }
}

impl From<VariantMetadata> for Ident {
    fn from(value: VariantMetadata) -> Self {
        value.ident
    }
}

struct Variants {
    _span: Span,
    /// Represents the Identifier for the enum.
    enum_ident: Ident,
    // Contains all metadata around each enum variant.
    variant_metadata: Vec<VariantMetadata>,
}

impl Variants {
    fn new(span: Span, enum_ident: Ident, variant_metadata: Vec<VariantMetadata>) -> Self {
        Self {
            _span: span,
            enum_ident,
            variant_metadata,
        }
    }
}

fn parse(input: DeriveInput) -> Result<Variants, syn::Error> {
    let input_span = input.span();
    let tok_enum_name = input.ident;
    let enum_variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => {
            return Err(syn::Error::new(
                input_span,
                "derive macro only works on enums",
            ))
        }
    };

    enum_variants
        .into_iter()
        .map(|variant| {
            let variant_span = variant.span();
            let variant_ident = variant.ident;
            let variant_fields = variant.fields;

            match variant_fields {
                // an empty filed
                Fields::Unit => Ok(VariantMetadata::new(variant_span, variant_ident)),
                l => Err(syn::Error::new(
                    l.span(),
                    format!(
                        "variant({}) expects exactly 0 fields, got {}",
                        &variant_ident,
                        l.len()
                    ),
                )),
            }
        })
        .collect::<Result<_, _>>()
        .map(|enriched_token_variants| {
            Variants::new(input_span, tok_enum_name, enriched_token_variants)
        })
}

/// Generates an iterator over all variants for an enum.
fn codegen(variants: Variants) -> syn::Result<TokenStream> {
    let enum_ident = &variants.enum_ident;
    let variant_len = variants.variant_metadata.len();

    let variant_strs = variants
        .variant_metadata
        .iter()
        .map(|var| format!("{}::{}", enum_ident, &var.ident))
        .collect::<Vec<_>>();

    let joined_variants = TokenStream::from_str(&variant_strs.join(", "))?;

    let enumerator_stream = quote! {
        impl #enum_ident {
            pub fn enumerate_variants() -> std::array::IntoIter<#enum_ident, #variant_len> {
                [#joined_variants].into_iter()
            }
        }
    };

    Ok(enumerator_stream)
}

/// Parses and generates an ordered iterator over all variants of an enum in
/// the order that they are defined..
#[proc_macro_derive(VariantEnumerator)]
pub fn generate_variant_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    parse(input)
        .and_then(codegen)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
