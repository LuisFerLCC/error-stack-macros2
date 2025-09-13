use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
    DeriveInput,
    parse::{Parse, ParseStream},
};

#[derive(Debug)]
pub struct ErrorStackType(DeriveInput);

impl Parse for ErrorStackType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for ErrorStackType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {}
}
