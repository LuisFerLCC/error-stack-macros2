use syn::Attribute;

pub(crate) fn take_display_attr(
    attrs: &mut Vec<Attribute>,
) -> Option<Attribute> {
    let index = attrs
        .iter_mut()
        .position(|attr| attr.path().is_ident("display"))?;
    Some(attrs.remove(index))
}
