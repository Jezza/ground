mod ast;
mod utils;
mod codegen;

#[proc_macro_derive(FromEnv, attributes(env))]
pub fn env_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let input: ast::DeriveInput = match darling::FromDeriveInput::from_derive_input(&input) {
        Ok(parsed) => parsed,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let root = input
        .root
        .as_ref()
        .map(|path| quote::quote!(#path))
        .unwrap_or(quote::quote! {
            ::ground_env
        });

    codegen::expand(root, input).into()
}
