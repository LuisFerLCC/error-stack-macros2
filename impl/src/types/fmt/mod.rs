use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Data, DeriveInput, Fields, LitStr, Meta, Variant, parse::Parse,
    spanned::Spanned,
};

pub(crate) mod input;
use input::{EnumVariantFormatInput, StructFormatInput};

use crate::util::traits::IteratorExt;

#[derive(Debug)]
pub(crate) enum FormatData {
    Struct {
        display_input: StructFormatInput,
    },

    Enum {
        default_display_input: Option<LitStr>,
        variant_display_inputs: HashMap<Variant, EnumVariantFormatInput>,
    },

    EmptyEnum,
}

impl FormatData {
    pub(crate) fn new(derive_input: &DeriveInput) -> syn::Result<Self> {
        let ident = &derive_input.ident;

        match &derive_input.data {
            Data::Struct(_) => {
                let display_attr = Self::get_display_attr(&derive_input.attrs)
                    .ok_or_else(|| syn::Error::new(ident.span(), "missing `display` attribute for struct with `#[derive(Error)]`"))?;
                let display_input = Self::get_format_input(display_attr)?;

                Ok(Self::Struct { display_input })
            }

            Data::Enum(data) => {
                let variants = &data.variants;
                if variants.is_empty() {
                    return Ok(Self::EmptyEnum);
                }

                let default_display_attr =
                    Self::get_display_attr(&derive_input.attrs);

                let variant_display_attrs = variants.iter().map(|variant| {
                    (variant, Self::get_display_attr(&variant.attrs))
                });

                let variant_display_inputs_res = variant_display_attrs
                    .clone()
                    .filter_map(|(variant, attr)| Some((variant, attr?)))
                    .map(|(variant, attr)| {
                        Self::get_format_input(attr)
                            .map(|input| (variant.clone(), input))
                    })
                    .collect_hashmap_and_combine_syn_errors();

                if let Some(attr) = default_display_attr {
                    let default_display_input =
                        Some(Self::get_format_input(attr)?);

                    return Ok(Self::Enum {
                        default_display_input,
                        variant_display_inputs: variant_display_inputs_res?,
                    });
                }

                match variant_display_inputs_res {
                    Ok(inputs) => {
                        if inputs.is_empty() {
                            return Err(syn::Error::new(
                                ident.span(),
                                "missing `display` attribute for enum with `#[derive(Error)]`\nadd a `display` attribute to at least the whole enum or to all of its variants",
                            ));
                        }

                        let unformatted_variants_error = variant_display_attrs
                            .filter_map(|(variant, attr)| match attr {
                                Some(_) => None,
                                None => Some(syn::Error::new(
                                    variant.span(),
                                    "missing `display` attribute for variant in enum with `#[derive(Error)]`\nadd a `display` attribute either to the whole enum (as a default) or to the remaining variants"
                                )),
                            })
                            .reduce(|mut acc, next| {
                                acc.combine(next);
                                acc
                            });

                        if let Some(err) = unformatted_variants_error {
                            return Err(err);
                        }

                        Ok(Self::Enum {
                            default_display_input: None,
                            variant_display_inputs: inputs,
                        })
                    }

                    Err(err) => Err(err),
                }
            }

            _ => Err(syn::Error::new(
                ident.span(),
                "`#[derive(Error)]` only supports structs and enums",
            )),
        }
    }

    pub(crate) fn get_display_attr(attrs: &[Attribute]) -> Option<&Attribute> {
        attrs.iter().find(|attr| attr.path().is_ident("display"))
    }

    pub(crate) fn get_format_input<T>(
        display_attr: &Attribute,
    ) -> syn::Result<T>
    where
        T: Parse,
    {
        if let Meta::List(meta) = &display_attr.meta {
            return syn::parse(meta.tokens.clone().into()).map_err(|err| {
                if err.to_string()
                    == "unexpected end of input, expected string literal"
                {
                    syn::Error::new(meta.span(), "unexpected empty `display` attribute, expected string literal")
                } else {
                    err
                }
            });
        }

        Err(syn::Error::new(
            display_attr.span(),
            "expected `display` to be a list attribute: `#[display(\"template...\")]`",
        ))
    }
}

impl ToTokens for FormatData {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Struct { display_input } => {
                tokens.extend(quote! {
                    ::core::write!(f, #display_input)
                });
            }

            Self::Enum {
                default_display_input,
                variant_display_inputs,
            } => {
                let branches = variant_display_inputs
                    .iter()
                    .map(|(variant, format_input)| {
                        let ident = &variant.ident;

                        let fields = &variant.fields;
                        let field_idents = fields
                            .iter()
                            .enumerate()
                            .map(|(i, field)|
                                field.ident.clone().unwrap_or_else(||
                                    syn::Ident::new(
                                        &format!("_field{}", i),
                                        field.span())));

                        let field_tokens = match &variant.fields {
                            Fields::Named(_) => quote! { { #(#field_idents),* } },
                            Fields::Unnamed(_) => quote! { ( #(#field_idents),* ) },
                            Fields::Unit => TokenStream2::new()
                        };

                        quote! {
                            Self::#ident #field_tokens => ::core::write!(f, #format_input)
                        }
                    })
                    .chain(default_display_input.as_ref().map(|lit_str| {
                        quote! {
                           _ => ::core::write!(f, #lit_str)
                        }
                    }));

                tokens.extend(quote! {
                   match &self {
                       #(#branches),*
                   }
                });
            }

            Self::EmptyEnum => {
                tokens.extend(quote! {
                    unreachable!("attempted to format an empty enum")
                });
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    use quote::quote;

    #[test]
    fn struct_format_data_requires_display_attr() {
        let derive_input =
            syn::parse2::<DeriveInput>(quote! { struct CustomType; })
                .expect("malformed test stream");
        let err = FormatData::new(&derive_input).expect_err(
            "stream without display attr was parsed successfully as FormatData",
        );
        assert_eq!(
            err.to_string(),
            "missing `display` attribute for struct with `#[derive(Error)]`"
        );
    }

    #[test]
    fn struct_format_data_requires_list_form_for_display_attr() {
        let derive_input = syn::parse2::<DeriveInput>(
            quote! { #[display] struct CustomType; },
        )
        .expect("malformed test stream");
        let err = FormatData::new(&derive_input).expect_err(
            "stream with path display attr was parsed successfully as FormatData",
        );
        assert_eq!(
            err.to_string(),
            "expected `display` to be a list attribute: `#[display(\"template...\")]`"
        );
    }

    #[test]
    fn enum_format_data_requires_display_attr() {
        let derive_input =
            syn::parse2::<DeriveInput>(quote! { enum CustomType { One, Two } })
                .expect("malformed test stream");
        let err = FormatData::new(&derive_input).expect_err(
            "stream without display attr was parsed successfully as FormatData",
        );
        assert_eq!(
            err.to_string(),
            "missing `display` attribute for enum with `#[derive(Error)]`\nadd a `display` attribute to at least the whole enum or to all of its variants"
        );
    }

    #[test]
    fn enum_format_data_requires_list_form_for_display_attr() {
        let derive_input = syn::parse2::<DeriveInput>(
            quote! { #[display] enum CustomType { One, Two } },
        )
        .expect("malformed test stream");
        let err = FormatData::new(&derive_input).expect_err(
            "stream with path display attr was parsed successfully as FormatData",
        );
        assert_eq!(
            err.to_string(),
            "expected `display` to be a list attribute: `#[display(\"template...\")]`"
        );
    }

    #[test]
    fn enum_format_data_requires_list_form_for_display_attr_on_every_variant() {
        let derive_input = syn::parse2::<DeriveInput>(quote! {
            enum CustomType {
                #[display]
                One,
                #[display]
                Two
            }
        })
        .expect("malformed test stream");
        let err = FormatData::new(&derive_input).expect_err(
            "stream with path display attr was parsed successfully as FormatData",
        );
        assert_eq!(
            err.to_string(),
            "expected `display` to be a list attribute: `#[display(\"template...\")]`"
        );
    }

    #[test]
    fn union_type_is_rejected() {
        let derive_input = syn::parse2::<DeriveInput>(
            quote! { union CustomType { f1: u32, f2: f32 } },
        )
        .expect("malformed test stream");
        let err = FormatData::new(&derive_input).expect_err(
            "stream with union type was parsed successfully as FormatData",
        );
        assert_eq!(
            err.to_string(),
            "`#[derive(Error)]` only supports structs and enums"
        );
    }
}
