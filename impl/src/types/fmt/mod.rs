use std::convert::Infallible;
#[cfg(test)]
use std::fmt::{self, Debug, Formatter};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{Attribute, Data, Fields, Ident, LitStr, spanned::Spanned};

mod input;
use input::{StructFormatInput, VariantFormatInput};

mod util;

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
        let default_display_attr = super::util::take_display_attr(attrs);

        match input_data {
            Data::Struct(_) => {
                drop(input_data);

                let display_attr = default_display_attr
                    .ok_or_else(|| syn::Error::new(ident_span, "missing `display` attribute for struct with `#[derive(Error)]`"))?;
                let display_input = util::get_format_input(display_attr)?;

                Ok(Self::Struct { display_input })
            }

            Data::Enum(data) => {
                let variants = data.variants;
                if variants.is_empty() {
                    drop(variants);
                    return Ok(Self::EmptyEnum);
                }

                let variant_display_inputs =
                    util::collect_valid_variant_states(variants)?;

                if let Some(attr) = default_display_attr {
                    let default_display_input =
                        Some(util::get_format_input(attr)?);

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
                    util::separate_existing_variant_states(
                        variant_display_inputs,
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
    use crate::ErrorStackDeriveInput;

    use super::*;

    use quote::quote;
    use syn::DeriveInput;

    #[test]
    fn struct_data_requires_display_attr() {
        let mut derive_input: DeriveInput =
            syn::parse2(quote! { struct CustomType; })
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
        let mut derive_input: DeriveInput =
            syn::parse2(quote! { #[display] struct CustomType; })
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
        let mut derive_input: DeriveInput =
            syn::parse2(quote! { enum CustomType { One, Two } })
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
        let mut derive_input: DeriveInput =
            syn::parse2(quote! { #[display] enum CustomType { One, Two } })
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
        let mut derive_input: DeriveInput = syn::parse2(quote! {
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
        let mut derive_input: DeriveInput =
            syn::parse2(quote! { union CustomType { f1: u32, f2: f32 } })
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

    #[test]
    fn variant_works_with_other_attrs() {
        let derive_input: ErrorStackDeriveInput = syn::parse2(quote! {
            #[display("custom type")]
            enum CustomType {
                #[cfg(true)]
                One { inner: u8 },

                #[cfg(true)]
                #[display("custom type two {0}.{1}.{2}.{3}")]
                Two(u8, u8, u8, u8),
            }
        })
        .expect("malformed test stream");

        let output = quote! { #derive_input };
        assert_eq!(
            output.to_string(),
            "# [allow (single_use_lifetimes)] impl :: core :: fmt :: Display for CustomType { fn fmt (& self , f : & mut :: core :: fmt :: Formatter < '_ >) -> :: core :: fmt :: Result { match & self { # [cfg (true)] Self :: Two (_field0 , _field1 , _field2 , _field3) => :: core :: write ! (f , \"custom type two {}.{}.{}.{}\" , _field0 , _field1 , _field2 , _field3) , _ => :: core :: write ! (f , \"custom type\") } } } # [allow (single_use_lifetimes)] impl :: core :: error :: Error for CustomType { }"
        );
    }
}
