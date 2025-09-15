use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    DeriveInput, Ident,
    parse::{Parse, ParseStream},
};

#[derive(Debug)]
pub(crate) struct ErrorStackDeriveInput {
    ident: Ident,
}

impl ErrorStackDeriveInput {
    pub(crate) fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl Parse for ErrorStackDeriveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let derive_input: DeriveInput = input.parse()?;

        Ok(Self {
            ident: derive_input.ident,
        })
    }
}

impl ToTokens for ErrorStackDeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = self.ident();

        tokens.extend(quote! {
            impl ::core::fmt::Display for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::write!(f, "test")
                }
            }

            impl ::core::error::Error for #ident {}
        });
    }
}
