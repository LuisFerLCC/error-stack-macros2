use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    DeriveInput, Ident,
    parse::{Parse, ParseStream},
};

mod fmt;
use fmt::TypeData;

pub(crate) struct ErrorStackDeriveInput {
    ident: Ident,
    display_data: TypeData,
}

impl Parse for ErrorStackDeriveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let derive_input: DeriveInput = input.parse()?;

        let display_data = TypeData::new(
            derive_input.data,
            derive_input.attrs,
            derive_input.ident.span(),
        )?;

        let ident = derive_input.ident;

        Ok(Self {
            ident,
            display_data,
        })
    }
}

impl ToTokens for ErrorStackDeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            ident,
            display_data,
        } = self;

        tokens.extend(quote! {
            impl ::core::fmt::Display for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #display_data
                }
            }

            impl ::core::error::Error for #ident {}
        });
    }
}
