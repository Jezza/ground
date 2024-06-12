use darling::util::Override;
use proc_macro2::Ident;

pub(crate) type EnvData = darling::ast::Data<(), EnvField>;

#[derive(Debug, darling::FromDeriveInput)]
#[darling(attributes(env), supports(struct_named))]
pub(crate) struct DeriveInput {
    pub ident: Ident,
    pub data: EnvData,
    pub root: Option<syn::Path>,
}

#[derive(Clone, Debug, darling::FromField)]
#[darling(attributes(env))]
pub(crate) struct EnvField {
    pub ident: Option<Ident>,
    pub ty: syn::Type,

    /// use this environment variable name instead of field ident
    pub rename: Option<syn::LitStr>,

    /// parse string literal into value
    pub default: Option<Override<syn::LitStr>>,

    /// parse string literal into value, or Default::default() when no explicit value provided
    pub flatten: Option<Override<syn::LitStr>>,

    pub delimiter: Option<syn::LitStr>,
}
