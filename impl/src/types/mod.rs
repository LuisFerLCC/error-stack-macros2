use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Attribute, DeriveInput, Ident,
    parse::{Parse, ParseStream},
};

mod fmt;
use fmt::TypeData;

mod util;

pub(crate) struct ErrorStackDeriveInput {
    other_attrs: Vec<Attribute>,
    ident: Ident,
    display_data: TypeData,
}

impl Parse for ErrorStackDeriveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let derive_input: DeriveInput = input.parse()?;

        drop(derive_input.generics);
        drop(derive_input.vis);

        let mut attrs = derive_input.attrs;

        let display_data = TypeData::new(
            derive_input.data,
            &mut attrs,
            derive_input.ident.span(),
        )?;

        let ident = derive_input.ident;

        Ok(Self {
            other_attrs: attrs,
            ident,
            display_data,
        })
    }
}

impl ToTokens for ErrorStackDeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            other_attrs,
            ident,
            display_data,
        } = self;

        tokens.extend(quote! {
            #(#other_attrs)*
            impl ::core::fmt::Display for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #display_data
                }
            }

            impl ::core::error::Error for #ident {}
        });
    }
}
