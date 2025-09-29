use std::convert::Infallible;
#[cfg(test)]
use std::fmt::{self, Debug, Formatter};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{
    Attribute, Data, Fields, Ident, LitStr, Meta, Variant, parse::Parse,
    punctuated::Punctuated, spanned::Spanned, token::Comma,
};

mod input;
use input::{StructFormatInput, VariantFormatInput};

use super::util;

pub(crate) enum TypeData {
    Struct {
        display_input: StructFormatInput,
    },

    Enum {
        default_display_input: Option<LitStr>,
        variant_display_inputs: Vec<VariantData>,
    },

    EmptyEnum,
}

impl TypeData {
    pub(crate) fn new(
        input_data: Data,
        attrs: &mut Vec<Attribute>,
        ident_span: Span,
    ) -> syn::Result<Self> {
        let default_display_attr = util::take_display_attr(attrs);

        match input_data {
            Data::Struct(_) => {
                drop(input_data);

                let display_attr = default_display_attr
                    .ok_or_else(|| syn::Error::new(ident_span, "missing `display` attribute for struct with `#[derive(Error)]`"))?;
                let display_input = Self::get_format_input(display_attr)?;

                Ok(Self::Struct { display_input })
            }

            Data::Enum(data) => {
                let variants = data.variants;
                if variants.is_empty() {
                    drop(variants);
                    return Ok(Self::EmptyEnum);
                }

                let variant_display_inputs =
                    Self::collect_valid_variant_states(variants)?;

                if let Some(attr) = default_display_attr {
                    let default_display_input =
                        Some(Self::get_format_input(attr)?);

                    return Ok(Self::Enum {
                        default_display_input,
                        variant_display_inputs: variant_display_inputs
                            .into_iter()
                            .filter_map(|state| state.data())
                            .collect(),
                    });
                };

                drop(default_display_attr);

                let (valid_variants, none_spans) =
                    Self::separate_existing_variant_states(
                        variant_display_inputs.into_iter(),
                    );

                if valid_variants.is_empty() {
                    drop(valid_variants);
                    drop(none_spans);
                    return Err(syn::Error::new(
                        ident_span,
                        "missing `display` attribute for enum with `#[derive(Error)]`\nadd a `display` attribute to at least the whole enum or to all of its variants",
                    ));
                }

                if !none_spans.is_empty() {
                    drop(valid_variants);

                    #[allow(clippy::unwrap_used)]
                    return Err(none_spans
                        .into_iter()
                        .map(|span| {
                            syn::Error::new(
                                span,
                                "missing `display` attribute for variant in enum with `#[derive(Error)]`\nadd a `display` attribute either to the whole enum (as a default) or to the remaining variants"
                            )
                        }).reduce(|mut err, err2| {
                            err.combine(err2);
                            err
                        }).unwrap());
                }

                drop(none_spans);

                Ok(Self::Enum {
                    default_display_input: None,
                    variant_display_inputs: valid_variants,
                })
            }

            _ => {
                drop(input_data);
                drop(default_display_attr);

                Err(syn::Error::new(
                    ident_span,
                    "`#[derive(Error)]` only supports structs and enums",
                ))
            }
        }
    }

    fn get_format_input<T>(display_attr: Attribute) -> syn::Result<T>
    where
        T: Parse,
    {
        let attr_span = display_attr.span();

        if let Meta::List(meta) = display_attr.meta {
            let meta_span = meta.span();
            drop(meta.path);

            let parse_res = syn::parse2::<T>(meta.tokens);

            match parse_res {
                Ok(input) => return Ok(input),
                Err(err) => {
                    return Err(
                        if err.to_string()
                            == "unexpected end of input, expected string literal"
                        {
                            drop(err);

                            syn::Error::new(
                                meta_span,
                                "unexpected empty `display` attribute, expected string literal",
                            )
                        } else {
                            err
                        },
                    );
                }
            }
        }

        drop(display_attr);

        Err(syn::Error::new(
            attr_span,
            "expected `display` to be a list attribute: `#[display(\"template...\")]`",
        ))
    }

    fn collect_valid_variant_states(
        variants: Punctuated<Variant, Comma>,
    ) -> Result<Vec<ValidVariantState>, syn::Error> {
        let mut variant_states_iter = variants.into_iter().map(|variant| {
            let variant_span = variant.span();

            let mut attrs = variant.attrs;
            let display_attr = util::take_display_attr(&mut attrs);

            use VariantState as VS;
            match display_attr {
                None => VS::None(variant_span),
                Some(attr) => match Self::get_format_input(attr) {
                    Ok(input) => VS::Valid(VariantData {
                        other_attrs: attrs,
                        ident: variant.ident,
                        fields: variant.fields,
                        display_input: input,
                    }),
                    Err(err) => VS::Invalid(err),
                },
            }
        });

        let mut vec = Vec::new();

        while let Some(state) = variant_states_iter.next() {
            use VariantState as VS;
            match state {
                VS::None(span) => vec.push(VS::None(span)),
                VS::Valid(data) => vec.push(VS::Valid(data)),
                VS::Invalid(mut err) => {
                    while let Some(VS::Invalid(err2)) =
                        variant_states_iter.next()
                    {
                        err.combine(err2);
                    }

                    return Err(err);
                }
            }
        }

        drop(variant_states_iter);

        Ok(vec)
    }

    fn separate_existing_variant_states<I>(
        states_iter: I,
    ) -> (Vec<VariantData>, Vec<Span>)
    where
        I: Iterator<Item = ValidVariantState>,
    {
        let mut valid_variants = Vec::new();
        let mut none_spans = Vec::new();

        for state in states_iter {
            use VariantState as VS;
            match state {
                VS::Valid(data) => valid_variants.push(data),
                VS::None(span) => none_spans.push(span),
            }
        }

        (valid_variants, none_spans)
    }
}

#[cfg(test)]
impl Debug for TypeData {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl ToTokens for TypeData {
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
                    .map(|variant| {
                        quote! { #variant }
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

enum VariantState<E> {
    Valid(VariantData),
    Invalid(E),
    None(Span),
}

impl<E> VariantState<E> {
    fn data(self) -> Option<VariantData> {
        match self {
            Self::Valid(data) => Some(data),
            _ => {
                drop(self);
                None
            }
        }
    }
}

type ValidVariantState = VariantState<Infallible>;

pub(crate) struct VariantData {
    other_attrs: Vec<Attribute>,
    ident: Ident,
    fields: Fields,
    display_input: VariantFormatInput,
}

impl ToTokens for VariantData {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            other_attrs,
            ident,
            fields,
            display_input,
        } = self;

        let field_idents = fields.iter().enumerate().map(|(i, field)| {
            field.ident.clone().unwrap_or_else(|| {
                Ident::new(&format!("_field{}", i), field.span())
            })
        });

        let field_tokens = match fields {
            Fields::Named(_) => quote! { { #(#field_idents),* } },
            Fields::Unnamed(_) => quote! { ( #(#field_idents),* ) },
            Fields::Unit => {
                drop(field_idents);
                TokenStream2::new()
            }
        };

        tokens.extend(quote! {
            #(#other_attrs)*
            Self::#ident #field_tokens => ::core::write!(f, #display_input)
        })
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    use quote::quote;
    use syn::DeriveInput;

    #[test]
    fn struct_data_requires_display_attr() {
        let mut derive_input =
            syn::parse2::<DeriveInput>(quote! { struct CustomType; })
                .expect("malformed test stream");
        let err = TypeData::new(
            derive_input.data,
            &mut derive_input.attrs,
            derive_input.ident.span(),
        )
        .expect_err(
            "stream without display attr was parsed successfully as TypeData",
        );
        assert_eq!(
            err.to_string(),
            "missing `display` attribute for struct with `#[derive(Error)]`"
        );
    }

    #[test]
    fn struct_data_requires_list_form_for_display_attr() {
        let mut derive_input = syn::parse2::<DeriveInput>(
            quote! { #[display] struct CustomType; },
        )
        .expect("malformed test stream");
        let err = TypeData::new(
            derive_input.data,
            &mut derive_input.attrs,
            derive_input.ident.span(),
        )
        .expect_err(
            "stream with path display attr was parsed successfully as TypeData",
        );
        assert_eq!(
            err.to_string(),
            "expected `display` to be a list attribute: `#[display(\"template...\")]`"
        );
    }

    #[test]
    fn enum_data_requires_display_attr() {
        let mut derive_input =
            syn::parse2::<DeriveInput>(quote! { enum CustomType { One, Two } })
                .expect("malformed test stream");
        let err = TypeData::new(
            derive_input.data,
            &mut derive_input.attrs,
            derive_input.ident.span(),
        )
        .expect_err(
            "stream without display attr was parsed successfully as TypeData",
        );
        assert_eq!(
            err.to_string(),
            "missing `display` attribute for enum with `#[derive(Error)]`\nadd a `display` attribute to at least the whole enum or to all of its variants"
        );
    }

    #[test]
    fn enum_data_requires_list_form_for_display_attr() {
        let mut derive_input = syn::parse2::<DeriveInput>(
            quote! { #[display] enum CustomType { One, Two } },
        )
        .expect("malformed test stream");
        let err = TypeData::new(
            derive_input.data,
            &mut derive_input.attrs,
            derive_input.ident.span(),
        )
        .expect_err(
            "stream with path display attr was parsed successfully as TypeData",
        );
        assert_eq!(
            err.to_string(),
            "expected `display` to be a list attribute: `#[display(\"template...\")]`"
        );
    }

    #[test]
    fn enum_data_requires_list_form_for_display_attr_on_every_variant() {
        let mut derive_input = syn::parse2::<DeriveInput>(quote! {
            enum CustomType {
                #[display]
                One,
                #[display]
                Two
            }
        })
        .expect("malformed test stream");
        let err = TypeData::new(
            derive_input.data,
            &mut derive_input.attrs,
            derive_input.ident.span(),
        )
        .expect_err(
            "stream with path display attr was parsed successfully as TypeData",
        );
        assert_eq!(
            err.to_string(),
            "expected `display` to be a list attribute: `#[display(\"template...\")]`"
        );
    }

    #[test]
    fn union_type_is_rejected() {
        let mut derive_input = syn::parse2::<DeriveInput>(
            quote! { union CustomType { f1: u32, f2: f32 } },
        )
        .expect("malformed test stream");
        let err = TypeData::new(
            derive_input.data,
            &mut derive_input.attrs,
            derive_input.ident.span(),
        )
        .expect_err(
            "stream with union type was parsed successfully as TypeData",
        );
        assert_eq!(
            err.to_string(),
            "`#[derive(Error)]` only supports structs and enums"
        );
    }
}
