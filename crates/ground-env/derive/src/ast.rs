use darling::util::Override;
use proc_macro2::Ident;

#[derive(Clone, Debug, darling::FromField)]
#[darling(attributes(env))]
pub(crate) struct EnvField {
    pub ident: Option<Ident>,
    pub ty: syn::Type,

    /// use std::Default::default() as default value
    pub default: darling::util::Flag,
    /// parse string literal into value
    pub default_value: Option<syn::LitStr>,

    /// use this environment variable name instead of field ident
    pub rename: Option<syn::LitStr>,

    pub flatten: Option<Override<syn::LitStr>>,

    pub delimiter: Option<syn::LitStr>,
}

pub(crate) type EnvData = darling::ast::Data<(), EnvField>;

#[derive(Debug, darling::FromDeriveInput)]
#[darling(attributes(env), supports(struct_named))]
pub(crate) struct DeriveInput {
    pub ident: Ident,
    pub data: EnvData,
    pub root: Option<syn::Path>,
}
