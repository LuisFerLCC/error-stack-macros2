use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Attribute, DeriveInput, Generics, Ident,
    parse::{Parse, ParseStream},
};

mod fmt;
use fmt::TypeData;

mod util;
use util::ReducedGenerics;

pub(crate) struct ErrorStackDeriveInput {
    other_attrs: Vec<Attribute>,
    ident: Ident,
    generics: Generics,
    display_data: TypeData,
}

impl Parse for ErrorStackDeriveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let derive_input: DeriveInput = input.parse()?;

        drop(derive_input.vis);

        let mut attrs = derive_input.attrs;

        let display_data = TypeData::new(
            derive_input.data,
            &mut attrs,
            derive_input.ident.span(),
        )?;

        let ident = derive_input.ident;

        let mut generics = derive_input.generics;
        generics
            .params
            .iter_mut()
            .for_each(util::remove_generic_default);

        Ok(Self {
            other_attrs: attrs,
            ident,
            generics,
            display_data,
        })
    }
}

impl ToTokens for ErrorStackDeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            other_attrs,
            ident,
            generics,
            display_data,
        } = self;

        let mut error_trait_generics = generics.clone();
        error_trait_generics
            .params
            .iter_mut()
            .for_each(util::add_debug_trait_bound);

        let type_generics: ReducedGenerics = generics
            .params
            .iter()
            .cloned()
            .map(util::generic_reduced_to_ident)
            .collect();

        tokens.extend(quote! {
            #(#other_attrs)*
            impl #generics ::core::fmt::Display for #ident #type_generics {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #display_data
                }
            }

            impl #error_trait_generics ::core::error::Error for #ident #type_generics {}
        });
    }
}
