use crate::utils::is_generic_ty;
use darling::util::Override;
use heck::ToShoutySnakeCase;

mod ast;
mod utils;

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

    expand(root, input).into()
}

fn expand(root: proc_macro2::TokenStream, input: ast::DeriveInput) -> proc_macro2::TokenStream {
    let ident = input.ident;
    let span = ident.span();

    match input.data {
        ast::EnvData::Enum(_) => {
            quote::quote_spanned! {span=>
                compile_error!("enum not supported");
            }
        }
        ast::EnvData::Struct(fields) => {
            let mut init = quote::quote! {};

            for field in fields {
                let field_ident = field.ident.clone().unwrap();

                let field_init = expand_field(&root, field, &field_ident);

                init.extend(quote::quote_spanned! {span=> #field_ident: { #field_init },})
            }

            quote::quote_spanned! { span=>
                impl #root::FromEnv for #ident {
                    #[allow(unused_variables)]
                    fn from_ctx(ctx: &#root::Context<'_>) -> #root::Result<Self> {
                        let prefix = ctx.prefix.get().unwrap_or_default();

                        Ok(Self {
                            #init
                        })
                    }
                }
            }
        }
    }
}

fn expand_field(
    root: &proc_macro2::TokenStream,
    field: ast::EnvField,
    field_ident: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let span = field_ident.span();

    if let Some(flatten) = field.flatten {
        return match flatten {
            Override::Inherit => {
                quote::quote_spanned! { span =>
                    ctx.with_prefix::<_>(None)?
                }
            }
            Override::Explicit(prefix) => {
                quote::quote_spanned! { span =>
                    ctx.with_prefix::<_>(Some(#prefix))?
                }
            }
        };
    }

    let value = {
        let key = match field.rename {
            Some(rename) => rename.value(),
            None => field_ident.to_string().to_shouty_snake_case(),
        };

        quote::quote_spanned! {span=>
            ctx.resolve(#key)
        }
    };

    match utils::subty_if_name(&field.ty, "Option") {
        Some(sub_ty) => {
            if is_generic_ty(sub_ty, "Option") {
                quote::quote_spanned! { span =>
                    compile_error!("Option<Option<_>> is not supported")
                }
            } else if is_generic_ty(sub_ty, "Vec") {
                quote::quote_spanned! { span =>
                    compile_error!("Option<Vec<_>> is not supported")
                }
            } else {
                expand_optional_field(root, span, value, field.default, field.default_value)
            }
        }
        None => match utils::subty_if_name(&field.ty, "Vec") {
            Some(sub_ty) => {
                if is_generic_ty(sub_ty, "Option") {
                    quote::quote_spanned! { span =>
                        compile_error!("Vec<Option<_>> is not supported")
                    }
                } else if is_generic_ty(sub_ty, "Vec") {
                    quote::quote_spanned! { span =>
                        compile_error!("Vec<Vec<_>> is not supported")
                    }
                } else {
                    expand_vec_field(
                        root,
                        span,
                        value,
                        field.default,
                        field.default_value,
                        field.delimiter,
                    )
                }
            }
            None => expand_mandatory_field(root, span, value, field.default, field.default_value),
        },
    }
}

fn expand_optional_field(
    root: &proc_macro2::TokenStream,
    span: proc_macro2::Span,
    value: proc_macro2::TokenStream,
    default: darling::util::Flag,
    default_value: Option<syn::LitStr>,
) -> proc_macro2::TokenStream {
    if default.is_present() {
        quote::quote_spanned! { span =>
            #root::transpose_err(#value?.map(#root::Parse::parse))?.ok()
        }
    } else if default_value.is_some() {
        quote::quote_spanned! { span =>
            compile_error!("default_value is not supported on Option<T>")
        }
    } else {
        quote::quote_spanned! { span =>
            #value?.ok().map(#root::Parse::parse).transpose()?
        }
    }
}

fn expand_mandatory_field(
    root: &proc_macro2::TokenStream,
    span: proc_macro2::Span,
    value: proc_macro2::TokenStream,
    default: darling::util::Flag,
    default_value: Option<syn::LitStr>,
) -> proc_macro2::TokenStream {
    if default.is_present() {
        quote::quote_spanned! { span =>
            #root::transpose_err(#value?.map(#root::Parse::parse))?.unwrap_or_default()
        }
    } else if let Some(default_value) = default_value {
        quote::quote_spanned! { span =>
            {
                let value = #value?.unwrap_or(#default_value);
                #root::Parse::parse(value)?
            }
        }
    } else {
        quote::quote_spanned! { span =>
            #value?.map(Parse::parse).map_err(Error::Missing)??
        }
    }
}

fn expand_vec_field(
    root: &proc_macro2::TokenStream,
    span: proc_macro2::Span,
    value: proc_macro2::TokenStream,
    default: darling::util::Flag,
    default_value: Option<syn::LitStr>,
    delimiter: Option<syn::LitStr>,
) -> proc_macro2::TokenStream {
    let delimiter = delimiter
        .map(|delimiter| delimiter.value())
        .unwrap_or(String::from(","));

    // This returns a `Result<T, String>`, where `Err` means it was missing.
    let value = quote::quote_spanned! {span=> {
        let result = #value?.map(|value| {
            #[allow(clippy::single_char_pattern)]
            value.split(#delimiter)
                .map(#root::Parse::parse)
                .collect::<std::result::Result<std::vec::Vec<_>, _>>()
        });
        transpose_err(result)?
    }};

    if default.is_present() {
        quote::quote_spanned! { span =>
            #value.unwrap_or_default()
        }
    } else if default_value.is_some() {
        quote::quote_spanned! { span =>
            compile_error!("default_value is not supported on Vec<T>")
        }
    } else {
        quote::quote_spanned! { span =>
            #value.map_err(#root::Error::Missing)?
        }
    }
}
