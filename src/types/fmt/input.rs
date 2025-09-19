use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use regex::Regex;
use syn::{
    Expr, Ident, LitInt, LitStr,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

#[derive(Debug)]
pub(crate) struct StructFormatInput {
    lit_str: LitStr,
    args: Punctuated<Expr, Comma>,
}

impl Parse for StructFormatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut lit_str: LitStr = input.parse()?;

        let comma: Option<Comma> = input.parse()?;
        if comma.is_none() && !input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "unexpected token after string literal",
            ));
        }

        #[allow(clippy::unwrap_used)]
        let regex = Regex::new(r"\{(\w+)(?::.+?)?\}").unwrap();
        let mut fmt_string = lit_str.value();
        let mut args = Punctuated::new();

        while let Some(captures) = regex.captures(&fmt_string) {
            #[allow(clippy::unwrap_used)]
            let group = captures.get(1).unwrap();
            let inline_arg_str = group.as_str();

            let arg_tokens = if inline_arg_str.parse::<usize>().is_ok() {
                let lit_int = LitInt::new(inline_arg_str, Span::call_site());
                quote! { &self.#lit_int }
            } else {
                let ident = Ident::new(inline_arg_str, Span::call_site());
                quote! { &self.#ident }
            };

            let arg_expr = syn::parse(arg_tokens.into())?;
            args.push(arg_expr);

            fmt_string.replace_range(group.range(), "");
        }

        lit_str = LitStr::new(&fmt_string, lit_str.span());

        Ok(Self { lit_str, args })
    }
}

impl ToTokens for StructFormatInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { lit_str, args } = self;

        tokens.extend(quote! {
            #lit_str, #args
        });
    }
}

#[derive(Debug)]
pub(crate) struct EnumVariantFormatInput {
    lit_str: LitStr,
    args: Punctuated<Ident, Comma>,
}

impl Parse for EnumVariantFormatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut lit_str: LitStr = input.parse()?;

        let comma: Option<Comma> = input.parse()?;
        if comma.is_none() && !input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "unexpected token after string literal",
            ));
        }

        #[allow(clippy::unwrap_used)]
        let regex = Regex::new(r"\{(\w+)(?::.+?)?\}").unwrap();
        let mut fmt_string = lit_str.value();
        let mut args = Punctuated::new();

        while let Some(captures) = regex.captures(&fmt_string) {
            #[allow(clippy::unwrap_used)]
            let group = captures.get(1).unwrap();
            let inline_arg_str = group.as_str();

            let ident_str = if inline_arg_str.parse::<usize>().is_ok() {
                &format!("_field{}", inline_arg_str)
            } else {
                inline_arg_str
            };

            let ident = Ident::new(ident_str, Span::call_site());
            args.push(ident);

            fmt_string.replace_range(group.range(), "");
        }

        lit_str = LitStr::new(&fmt_string, lit_str.span());

        Ok(Self { lit_str, args })
    }
}

impl ToTokens for EnumVariantFormatInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { lit_str, args } = self;

        tokens.extend(quote! {
            #lit_str, #args
        });
    }
}
