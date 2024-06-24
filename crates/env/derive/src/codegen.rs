use darling::util::Override;
use heck::ToShoutySnakeCase;
use proc_macro2::TokenStream;

use crate::{ast, utils};

pub fn expand(root: TokenStream, input: ast::DeriveInput) -> TokenStream {
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
                    fn from_ctx(ctx: &mut #root::Context) -> #root::Result<Self> {
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
    root: &TokenStream,
    field: ast::EnvField,
    field_ident: &proc_macro2::Ident,
) -> TokenStream {
    let span = field_ident.span();

    if let Some(flatten) = field.flatten {
        if field.rename.is_some() {
            return quote::quote_spanned! { span=>
                compile_error!("#[env(rename = "...")] cannot be used with `flatten`")
            };
        }
        if field.delimiter.is_some() {
            return quote::quote_spanned! { span=>
                compile_error!("#[env(delimiter = "...")] cannot be used with `flatten`")
            };
        }
        if field.default.is_some() {
            return quote::quote_spanned! { span=>
                compile_error!("#[env(default = "...")] cannot be used with `flatten`")
            };
        }
        return match flatten {
            Override::Inherit => {
                quote::quote_spanned! { span=>
                    <_ as #root::FromEnv>::from_ctx(ctx)?
                }
            }
            Override::Explicit(prefix) => {
                quote::quote_spanned! { span=>
                    ctx.with_prefix::<_>(#prefix)?
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
            if utils::is_generic_ty(sub_ty, "Option") {
                quote::quote_spanned! { span=>
                    compile_error!("Option<Option<_>> is not supported")
                }
            } else if utils::is_generic_ty(sub_ty, "Vec") {
                quote::quote_spanned! { span=>
                    compile_error!("Option<Vec<_>> is not supported")
                }
            } else {
                expand_optional_field(root, span, value, field.default)
            }
        }
        None => match utils::subty_if_name(&field.ty, "Vec") {
            Some(sub_ty) => {
                if utils::is_generic_ty(sub_ty, "Option") {
                    quote::quote_spanned! { span=>
                        compile_error!("Vec<Option<_>> is not supported")
                    }
                } else if utils::is_generic_ty(sub_ty, "Vec") {
                    quote::quote_spanned! { span=>
                        compile_error!("Vec<Vec<_>> is not supported")
                    }
                } else {
                    expand_vec_field(
                        root,
                        span,
                        value,
                        field.default,
                        field.delimiter,
                    )
                }
            }
            None => expand_mandatory_field(root, span, value, field.default),
        },
    }
}

fn expand_optional_field(
    root: &TokenStream,
    span: proc_macro2::Span,
    value: TokenStream,
    default: Option<Override<syn::LitStr>>,
) -> TokenStream {
    match default {
        Some(Override::Explicit(default)) => quote::quote_spanned! { span=>
            Some(#root::flatten_err(#root::transpose_err(#value?.map(#root::Parse::parse))?
                .map_err(|_| #root::Parse::parse(#default)))?)
        },
        Some(Override::Inherit) => quote::quote_spanned! { span=>
            #root::transpose_err(#value?.map(#root::Parse::parse))?.ok()
        },
        None => quote::quote_spanned! { span=>
            #value?.ok().map(#root::Parse::parse).transpose()?
        },
    }
}

fn expand_mandatory_field(
    root: &TokenStream,
    span: proc_macro2::Span,
    value: TokenStream,
    default: Option<Override<syn::LitStr>>,
) -> TokenStream {
    match default {
        Some(Override::Explicit(default)) => quote::quote_spanned! { span=>
            #root::Parse::parse(#value?.unwrap_or(#default))?
        },
        Some(Override::Inherit) => quote::quote_spanned! { span=>
            #root::transpose_err(#value?.map(#root::Parse::parse))?.unwrap_or_default()
        },
        None => quote::quote_spanned! { span=>
            #value?.map(#root::Parse::parse).map_err(#root::Error::Missing)??
        },
    }
}

fn expand_vec_field(
    root: &TokenStream,
    span: proc_macro2::Span,
    value: TokenStream,
    default: Option<Override<syn::LitStr>>,
    delimiter: Option<syn::LitStr>,
) -> TokenStream {
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

    match default {
        Some(Override::Explicit(_)) => quote::quote_spanned! { span=>
            compile_error!("default is not supported on Vec<T>")
        },
        Some(Override::Inherit) => quote::quote_spanned! { span=>
            #value.unwrap_or_default()
        },
        None => quote::quote_spanned! { span=>
            #value.map_err(#root::Error::Missing)?
        },
    }
}
