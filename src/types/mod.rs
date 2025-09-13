use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    DeriveInput,
    parse::{Parse, ParseStream},
};

#[derive(Debug)]
pub(crate) struct ErrorStackType(DeriveInput);

impl Parse for ErrorStackType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for ErrorStackType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.0.ident;

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
