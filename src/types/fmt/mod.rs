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
pub(crate) struct FormatInput {
    lit_str: LitStr,
    args: Punctuated<Expr, Comma>,
}

impl Parse for FormatInput {
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

            let arg_tokens = match inline_arg_str.parse::<usize>() {
                Ok(_) => {
                    let lit_int =
                        LitInt::new(inline_arg_str, Span::call_site());
                    quote! { &self.#lit_int }
                }

                Err(_) => {
                    let ident = Ident::new(inline_arg_str, Span::call_site());
                    quote! { &self.#ident }
                }
            };

            let arg_expr = syn::parse(arg_tokens.into())?;
            args.push(arg_expr);

            fmt_string.replace_range(group.range(), "");
        }

        lit_str = LitStr::new(&fmt_string, lit_str.span());

        Ok(Self { lit_str, args })
    }
}

impl ToTokens for FormatInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { lit_str, args } = self;

        tokens.extend(quote! {
            #lit_str, #args
        });
    }
}
