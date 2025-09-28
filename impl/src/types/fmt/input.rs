#[cfg(test)]
use std::fmt::{self, Debug, Formatter};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use regex::Regex;
use syn::{
    Expr, Ident, LitInt, LitStr,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

pub(crate) struct StructFormatInput {
    lit_str: LitStr,
    args: Punctuated<Expr, Comma>,
}

#[cfg(test)]
impl Debug for StructFormatInput {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl Parse for StructFormatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit_str: LitStr = input.parse()?;

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

        let lit_str_span = lit_str.span();
        drop(lit_str);

        while let Some(captures) = regex.captures(&fmt_string) {
            #[allow(clippy::unwrap_used)]
            let group = captures.get(1).unwrap();
            drop(captures);

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

        drop(regex);

        let lit_str = LitStr::new(&fmt_string, lit_str_span);
        drop(fmt_string);

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

pub(crate) struct VariantFormatInput {
    lit_str: LitStr,
    args: Punctuated<Ident, Comma>,
}

#[cfg(test)]
impl Debug for VariantFormatInput {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl Parse for VariantFormatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit_str: LitStr = input.parse()?;

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

        let lit_str_span = lit_str.span();
        drop(lit_str);

        while let Some(captures) = regex.captures(&fmt_string) {
            #[allow(clippy::unwrap_used)]
            let group = captures.get(1).unwrap();
            drop(captures);

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

        drop(regex);

        let lit_str = LitStr::new(&fmt_string, lit_str_span);
        drop(fmt_string);

        Ok(Self { lit_str, args })
    }
}

impl ToTokens for VariantFormatInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { lit_str, args } = self;

        tokens.extend(quote! {
            #lit_str, #args
        });
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    use quote::quote;

    #[test]
    fn struct_format_input_requires_initial_lit_str() {
        let empty_stream_res = syn::parse2::<StructFormatInput>(quote! {});
        let err = empty_stream_res.expect_err(
            "empty stream was parsed successfully as StructFormatInput",
        );
        assert_eq!(
            err.to_string(),
            "unexpected end of input, expected string literal"
        );
    }

    #[test]
    fn struct_format_input_requires_initial_arg_to_be_lit_str() {
        let empty_stream_res =
            syn::parse2::<StructFormatInput>(quote! { true });
        let err = empty_stream_res.expect_err(
            "stream `true` was parsed successfully as StructFormatInput",
        );
        assert_eq!(err.to_string(), "expected string literal");
    }

    #[test]
    fn struct_format_input_rejects_unexpected_token_after_lit_str() {
        let empty_stream_res =
            syn::parse2::<StructFormatInput>(quote! { "format string" 5 });
        let err = empty_stream_res.expect_err(
            "stream `\"format string\" 5` was parsed successfully as StructFormatInput",
        );
        assert_eq!(err.to_string(), "unexpected token after string literal");
    }

    #[test]
    fn struct_format_input_parses_lit_str_with_trailing_comma() {
        let empty_stream_res =
            syn::parse2::<StructFormatInput>(quote! { "format string", });
        let format_input = empty_stream_res.expect(
            "stream `\"format string\",` could not be parsed as StructFormatInput",
        );
        assert_eq!(format_input.lit_str.value(), "format string");
    }

    #[test]
    fn enum_variant_format_input_requires_initial_lit_str() {
        let empty_stream_res = syn::parse2::<VariantFormatInput>(quote! {});
        let err = empty_stream_res.expect_err(
            "empty stream was parsed successfully as VariantFormatInput",
        );
        assert_eq!(
            err.to_string(),
            "unexpected end of input, expected string literal"
        );
    }

    #[test]
    fn enum_variant_format_input_requires_initial_arg_to_be_lit_str() {
        let empty_stream_res =
            syn::parse2::<VariantFormatInput>(quote! { true });
        let err = empty_stream_res.expect_err(
            "stream `true` was parsed successfully as VariantFormatInput",
        );
        assert_eq!(err.to_string(), "expected string literal");
    }

    #[test]
    fn enum_variant_format_input_rejects_unexpected_token_after_lit_str() {
        let empty_stream_res =
            syn::parse2::<VariantFormatInput>(quote! { "format string" 5 });
        let err = empty_stream_res.expect_err(
            "stream `\"format string\" 5` was parsed successfully as VariantFormatInput",
        );
        assert_eq!(err.to_string(), "unexpected token after string literal");
    }

    #[test]
    fn enum_variant_format_input_parses_lit_str_with_trailing_comma() {
        let empty_stream_res =
            syn::parse2::<VariantFormatInput>(quote! { "format string", });
        let format_input = empty_stream_res.expect(
            "stream `\"format string\",` could not be parsed as VariantFormatInput",
        );
        assert_eq!(format_input.lit_str.value(), "format string");
    }
}
