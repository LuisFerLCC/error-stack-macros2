use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Expr, LitStr,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

#[derive(Debug)]
pub(crate) struct FormatArgs {
    lit_str: LitStr,
    args: Punctuated<Expr, Comma>,
}

impl Parse for FormatArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit_str = input.parse()?;

        let comma: Option<Comma> = input.parse()?;

        if comma.is_none() && !input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "unexpected token after string literal",
            ));
        }

        let args = Punctuated::parse_terminated(input)?;

        Ok(Self { lit_str, args })
    }
}

impl ToTokens for FormatArgs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { lit_str, args } = self;

        tokens.extend(quote! {
            #lit_str, #args
        });
    }
}
