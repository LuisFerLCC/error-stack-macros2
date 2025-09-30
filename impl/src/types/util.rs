use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Attribute, GenericParam, Ident, Lifetime, Path, TraitBound,
    TraitBoundModifier, TypeParamBound,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Colon, Comma},
};

pub(crate) enum ReducedGenericParam {
    ConstOrType(Ident),
    Lifetime(Lifetime),
}

impl ToTokens for ReducedGenericParam {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        use ReducedGenericParam as RGP;
        match self {
            RGP::ConstOrType(ident) => tokens.extend(quote! { #ident }),
            RGP::Lifetime(lifetime) => tokens.extend(quote! { #lifetime }),
        }
    }
}

pub(crate) struct ReducedGenerics {
    params: Punctuated<ReducedGenericParam, Comma>,
}

impl FromIterator<ReducedGenericParam> for ReducedGenerics {
    fn from_iter<T: IntoIterator<Item = ReducedGenericParam>>(iter: T) -> Self {
        Self {
            params: iter.into_iter().collect(),
        }
    }
}

impl ToTokens for ReducedGenerics {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if self.params.is_empty() {
            return;
        }

        let params = self.params.iter();
        tokens.extend(quote! { < #(#params),* > });
    }
}

pub(crate) fn take_display_attr(
    attrs: &mut Vec<Attribute>,
) -> Option<Attribute> {
    let index = attrs
        .iter_mut()
        .position(|attr| attr.path().is_ident("display"))?;
    Some(attrs.remove(index))
}

pub(crate) fn remove_generic_default(param: &mut GenericParam) {
    use GenericParam as GP;
    match param {
        GP::Const(const_p) => {
            const_p.eq_token = None;
            const_p.default = None;
        }

        GP::Type(type_p) => {
            type_p.eq_token = None;
            type_p.default = None;
        }

        _ => {}
    }
}

pub(crate) fn generic_reduced_to_ident(
    param: GenericParam,
) -> ReducedGenericParam {
    use GenericParam as GP;
    use ReducedGenericParam as RGP;
    match param {
        GP::Const(const_p) => {
            drop(const_p.attrs);
            drop(const_p.default);
            drop(const_p.ty);

            RGP::ConstOrType(const_p.ident)
        }

        GP::Type(type_p) => {
            drop(type_p.attrs);
            drop(type_p.bounds);
            drop(type_p.default);

            RGP::ConstOrType(type_p.ident)
        }

        GP::Lifetime(lifetime_p) => {
            drop(lifetime_p.attrs);
            drop(lifetime_p.bounds);

            RGP::Lifetime(lifetime_p.lifetime)
        }
    }
}

pub(crate) fn add_debug_trait_bound(param: &mut GenericParam) {
    use GenericParam as GP;
    if let GP::Type(type_p) = param {
        #[allow(clippy::unwrap_used)]
        let trait_path: Path =
            syn::parse2(quote! { ::core::fmt::Debug }).unwrap();

        type_p.colon_token = Some(Colon(type_p.span()));
        type_p.bounds.push(TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None,
            lifetimes: None,
            path: trait_path,
        }));
    }
}
