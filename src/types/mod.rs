use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    DeriveInput, Ident, Meta,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

mod fmt;
use fmt::FormatInput;

#[derive(Debug)]
pub(crate) struct ErrorStackDeriveInput {
    ident: Ident,
    display_input: FormatInput,
}

impl Parse for ErrorStackDeriveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let derive_input: DeriveInput = input.parse()?;

        let ident = derive_input.ident;

        let display_attr = derive_input
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("display"))
            .ok_or_else(|| {
                syn::Error::new(ident.span(), "missing `display` attribute for type annotated with `#[derive(Error)]`")
            })?;

        match &display_attr.meta {
            Meta::List(meta) => {
                let display_input = syn::parse(meta.tokens.clone().into()).map_err(|err| {
                    if err.to_string() == "unexpected end of input, expected string literal" {
                        syn::Error::new(meta.span(), "unexpected empty `display` attribute, expected string literal")
                    } else {
                        err
                    }
                })?;

                Ok(Self {
                    ident,
                    display_input,
                })
            }

            _ => Err(syn::Error::new(
                display_attr.span(),
                "expected `display` to be a list attribute: `#[display(\"template...\")]`",
            )),
        }
    }
}

impl ToTokens for ErrorStackDeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            ident,
            display_input,
        } = self;

        tokens.extend(quote! {
            impl ::core::fmt::Display for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::write!(f, #display_input)
                }
            }

            impl ::core::error::Error for #ident {}
        });
    }
}
