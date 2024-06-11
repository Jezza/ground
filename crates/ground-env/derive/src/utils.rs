pub(crate) fn subty_if_name<'a>(ty: &'a syn::Type, name: &str) -> Option<&'a syn::Type> {
    subty_if(ty, |seg| seg.ident == name)
}

pub(crate) fn is_generic_ty(ty: &syn::Type, name: &str) -> bool {
    subty_if_name(ty, name).is_some()
}

fn subty_if<F>(ty: &syn::Type, f: F) -> Option<&syn::Type>
where
    F: FnOnce(&syn::PathSegment) -> bool,
{
    let ty = strip_group(ty);

    last(ty).filter(|segment| f(segment)).and_then(|segment| {
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            only_one(args.args.iter()).and_then(|arg| {
                if let syn::GenericArgument::Type(ty) = arg {
                    Some(ty)
                } else {
                    None
                }
            })
        } else {
            None
        }
    })
}

fn strip_group(mut ty: &syn::Type) -> &syn::Type {
    while let syn::Type::Group(group) = ty {
        ty = &*group.elem;
    }

    ty
}

fn last(ty: &syn::Type) -> Option<&syn::PathSegment> {
    match ty {
        syn::Type::Path(syn::TypePath {
            qself: None,
            path:
                syn::Path {
                    leading_colon: None,
                    segments,
                },
        }) => only_one(segments.iter()),

        _ => None,
    }
}

fn only_one<I: Iterator<Item = T>, T>(mut it: I) -> Option<T> {
    it.next().filter(|_| it.next().is_none())
}
