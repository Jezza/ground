use ground_codegen_utils::case;
use ground_codegen_utils::module::Module;
use openapiv3::{Parameter, ParameterSchemaOrContent, PathStyle};
use proc_macro2::TokenStream;

use crate::utils;
use crate::utils::ClassifiedPathItem;

pub fn generate_routes(
    root: &mut Module,
    components: &openapiv3::Components,
    paths: openapiv3::Paths,
) -> anyhow::Result<()> {
    let openapiv3::Paths {
        paths,
        extensions: _path_ext,
    } = paths;

    // Group all the paths by their prefix, then generate routes.
    for (raw_api_name, api_routes) in utils::group_path_items(paths) {
        generate_route_module(root, &components, raw_api_name, api_routes)?;
    }

    Ok(())
}

fn generate_route_module(
    root: &mut Module,
    components: &openapiv3::Components,
    raw_api_name: String,
    api_routes: Vec<ClassifiedPathItem>,
) -> anyhow::Result<()> {
    let api_name = case::convert(&raw_api_name, case::Case::Snake);
    let ty_name = case::convert(&raw_api_name, case::Case::Pascal);
    let trait_name = quote::format_ident!("{}Api", ty_name);

    let api_module = root.create_child_from_path(vec![api_name.as_ref()]);

    let mut methods = quote::quote!();
    let mut routes = quote::quote!();

    let mut compile_errors = quote::quote!();
    macro_rules! comp_error {
        ($($tt:tt)*) => {{
            let msg = format!($($tt)*);
            compile_errors.extend(quote::quote! {
                compile_error!(#msg);
            });
        }};
    }

    for item in api_routes {
        let (url, mut item) = match item {
            ClassifiedPathItem::Item { url, item } => (url, item),
            ClassifiedPathItem::Alias { url, items } => {
                // @TODO jezza - 07 Oct 2024: We could probably do this in the pre-processing stage.
                let url = url.replace('{', ":");
                let url = url.replace('}', "");

                for (operation_id, _security) in items {
                    let raw_route_name = case::convert(&operation_id, case::Case::Snake);
                    let route_name = quote::format_ident!("{}", raw_route_name);

                    routes.extend(quote::quote! {
                        .route(#url, #route_name::method_router())
                    });
                }
                continue;
            }
        };

        // Segments of the route that specifically contain an id.
        // Think something like: /v0/users/{id}
        // This vec will contain ["id"].
        // /v0/users/{id} => ["id"]
        // /v0/users/{id}/{status} => ["id", "status"]
        let mut path_segments: Vec<_> = url
            .split('/')
            .filter_map(|item| item.strip_prefix('{'))
            .filter_map(|item| item.strip_suffix('}'))
            .collect();

        // @TODO jezza - 01 Oct 2024: Do this in-place
        // @TODO jezza - 07 Oct 2024: We could probably do this in the pre-processing stage.
        let url = url.replace('{', ":");
        let url = url.replace('}', "");

        // Common parameters
        let item_parameters = std::mem::take(&mut item.parameters);

        for (method, op) in item.into_iter() {
            let id = &op.operation_id.expect("Route doesn't have an id");

            let raw_route_name = case::convert(id, case::Case::Snake);
            let route_name = quote::format_ident!("{}", raw_route_name);

            let request_name =
                quote::format_ident!("{}Request", case::convert(id, case::Case::Pascal));
            let response_name =
                quote::format_ident!("{}Response", case::convert(id, case::Case::Pascal));

            // let mut endpoint_module = quote::quote!();

            let http_method = {
                let method = method
                    .to_ascii_uppercase()
                    .parse::<http::Method>()
                    .unwrap_or_else(|_| panic!("Must be a valid http route: {}", method));

                // @TODO jezza - 11 Apr 2023: Validate method and request body

                // We're relying on the fact that the constants in axum's MethodFilter are the same as the http's Method as_str.
                // Laziness ftw
                let method = quote::format_ident!("{}", method.as_str());
                quote::quote!(#method)
            };

            let mut handler_params = quote::quote!();
            let mut handler_body = quote::quote!();
            let mut request_fields = quote::quote!();
            let mut request_building = quote::quote!();

            let mut queries = vec![];
            let mut headers = vec![];
            let mut segments = vec![];
            let mut cookies = vec![];

            let parameters = item_parameters
                .clone()
                .into_iter()
                .chain(op.parameters.into_iter());

            for param in parameters {
                let param = utils::resolve_ref(components, &param).to_owned();

                match param {
                    Parameter::Query {
                        parameter_data: ref data,
                        ..
                    } => {
                        match data.format {
                            ParameterSchemaOrContent::Schema(_) => (),
                            ParameterSchemaOrContent::Content(_) => {
                                comp_error!(
                                    "Parameter ({}) `content` field not supported.",
                                    data.name
                                );
                                continue;
                            }
                        }

                        queries.push(param)
                    }
                    Parameter::Header {
                        parameter_data: ref data,
                        ..
                    } => {
                        match data.format {
                            ParameterSchemaOrContent::Schema(_) => (),
                            ParameterSchemaOrContent::Content(_) => {
                                comp_error!(
                                    "Parameter ({}) `content` field not supported.",
                                    data.name
                                );
                                continue;
                            }
                        }

                        headers.push(param)
                    }
                    Parameter::Path { .. } => segments.push(param),
                    Parameter::Cookie { .. } => cookies.push(param),
                }
            }

            if !queries.is_empty() {
                handler_params.extend(quote::quote! {
                    ::ground_openapi::export::QueryMap(mut query_map): ::ground_openapi::export::QueryMap,
                });

                for query in queries {
                    let (query_properties, query_fields) = generate_query_parameter(query);

                    request_fields.extend(quote::quote! {
                        #query_properties
                    });
                    request_building.extend(quote::quote! {
                        #query_fields
                    });
                }
            }

            if !headers.is_empty() {
                handler_params.extend(quote::quote! {
                    mut headers: ::ground_openapi::export::http::HeaderMap,
                });

                for header in headers {
                    let Parameter::Header {
                        parameter_data,
                        style: _,
                    } = header
                    else {
                        panic!("This list should only contain elements of type `Header`");
                    };

                    let id = parameter_data.name;
                    let param_name =
                        quote::format_ident!("header_{}", case::convert(&id, case::Case::Snake));
                    let name = quote::format_ident!("{}", case::convert(&id, case::Case::Snake));

                    let ty = utils::write_param_ref("request headers", &parameter_data.format);

                    let missing_msg = format!("Missing header: `{}`", id);

                    let required_header = if parameter_data.required {
                        quote::quote! {
                            .ok_or_else(|| {
                                ::ground_openapi::export::ApiError::missing_header(#missing_msg)
                            })?
                        }
                    } else {
                        quote::quote!()
                    };

                    let ty = if parameter_data.required {
                        ty
                    } else {
                        quote::quote! {
                            Option<#ty>
                        }
                    };

                    handler_body.extend(quote::quote! {
                        let #param_name: #ty = ::ground_openapi::handlers::parse_header(&mut headers, #id)?
                            #required_header;
                    });

                    request_building.extend(quote::quote! {
                        #name: #param_name,
                    });
                    request_fields.extend(quote::quote! {
                        pub #name: #ty,
                    });
                }

                handler_body.extend(quote::quote! {
                    ::ground_openapi::export::check_unused_headers(&headers)?;
                });
            }
            if !segments.is_empty() {
                if path_segments.len() != segments.len() {
                    comp_error!("Path segment mismatch between segments and uri template.");
                }

                handler_params.extend(quote::quote! {
                    path_params: ::ground_openapi::export::RawPathResult,
                });

                handler_body.extend(quote::quote! {
                    let path_params = path_params.map_err(::ground_openapi::export::handle_raw_path_error)?;
                });

                let mut path_arms = quote::quote!();

                for segment in segments {
                    let Parameter::Path {
                        parameter_data,
                        style,
                    } = segment
                    else {
                        panic!("This list should only contain elements of type path");
                    };
                    // simple – (default) comma-separated values. Corresponds to the {param_name} URI template.
                    // label – dot-prefixed values, also known as label expansion. Corresponds to the {.param_name} URI template.
                    // matrix – semicolon-prefixed values, also known as path-style expansion. Corresponds to the {;param_name} URI template.
                    match style {
                        PathStyle::Simple => {}
                        PathStyle::Matrix | PathStyle::Label => {
                            comp_error!("We only support simple style path segments");
                        }
                    }

                    let id = parameter_data.name;

                    let Some(pos) = path_segments.iter().position(|item| *item == &*id) else {
                        comp_error!("Unknown path segment: {}", id);
                        continue;
                    };
                    // Remove it so other segments can't use it.
                    let _ = path_segments.swap_remove(pos);

                    let name = quote::format_ident!("path_{}", id);

                    let ty = utils::write_param_ref("segment", &parameter_data.format);

                    let missing_msg = format!("Missing path argument: `{}`", id);
                    let duplicate_msg = format!("Duplicate path argument: `{}`", id);

                    handler_body.extend(quote::quote! {
                        let mut #name: Option<#ty> = None;
                    });
                    path_arms.extend(quote::quote! {
                        #id => if #name.is_some() {
                            let err = ::ground_openapi::export::ApiError::invalid_path_argument(
                                #duplicate_msg,
                                Some(value.into()),
                            );
                            return Err(err);
                        } else {
                            let value = ::ground_openapi::export::parse_segment(value)?;
                            #name = Some(value);
                        }
                    });
                    request_fields.extend(quote::quote! {
                        pub #name: #ty,
                    });
                    request_building.extend(quote::quote! {
                        #name: #name.ok_or_else(|| {
                            ::ground_openapi::export::ApiError::missing_path_argument(#missing_msg)
                        })?,
                    });
                }

                handler_body.extend(quote::quote! {
                    for (key, value) in path_params.into_iter() {
                        match key {
                            #path_arms
                            key => {
                                let msg = format!("Unexpected path argument found: {}", key);
                                let err = ::ground_openapi::export::ApiError::invalid_path_argument(msg, Some(value.into()));
                                return Err(err);
                            }
                        }
                    }
                });
            } else if !path_segments.is_empty() {
                // Template didn't have any patterns, but ids were declared.
                let ids = path_segments.join(",");

                comp_error!("Operation (url = {:?}, params = [{}]) declared with segments, but no segment pieces were defined. [You should use: `op.segment(..)`]", url, ids);
            }
            if !cookies.is_empty() {
                comp_error!("Cookies are not yet supported");
            }

            if let Some(request_body) = op.request_body {
                let body = utils::resolve_ref(components, &request_body);

                let media_type = body
                    .content
                    .get("application/json")
                    .expect("Only application/json is supported");
                let schema = media_type
                    .schema
                    .as_ref()
                    .expect("No schema defined for request body.");

                let ty = utils::write_ref("request_body", schema);

                request_fields.extend(quote::quote! {
                    pub body: #ty,
                });
                handler_params.extend(quote::quote! {
                    body: ::ground_openapi::export::RawBody<#ty>,
                });
                handler_body.extend(quote::quote! {
                    let ::ground_openapi::export::Json(body) = body.map_err(::ground_openapi::export::handle_json_rejection_error)?;
                });
                request_building.extend(quote::quote! {
                    body,
                });
            }

            let mut response_variants = quote::quote!();
            let mut response_arms = quote::quote!();
            let mut status_codes = vec![];

            for (status_code, response) in op.responses.responses {
                let openapiv3::Response {
                    description: _,
                    headers,
                    mut content,
                    links: _,
                    extensions: _,
                } = response.into_item().expect("We don't support ref elements");

                let code = match status_code {
                    openapiv3::StatusCode::Code(code) => {
                        status_codes.push(code);

                        code
                    }
                    openapiv3::StatusCode::Range(_) => {
                        // @TODO jezza - 11 Apr 2023: Support ranges
                        comp_error!("StatusCode ranges aren't supported");
                        continue;
                    }
                };

                let reason = http::StatusCode::from_u16(code)
                    .expect("already checked within the jsonnet lib")
                    .canonical_reason()
                    // @TODO jezza - 20 Mar 2023: Impl a way for the definition to provide a custom reason.
                    .expect("Non-standard return code.");
                let variant_name =
                    quote::format_ident!("{}", case::convert(reason, case::Case::Pascal));

                let mut fields = vec![];
                let mut into_response = vec![];
                let mut arm_body = quote::quote!();

                // Only json is supported
                if let Some(media_type) = content.swap_remove("application/json") {
                    let response_body = media_type.schema.expect("Response should have a schema");

                    let ty = utils::write_ref("response_body", &response_body);

                    let body = quote::format_ident!("body");
                    fields.push((body.clone(), ty));

                    arm_body.extend(quote::quote! {
                        let #body = ::ground_openapi::export::Json(#body);
                    });
                    into_response.push(body);
                } else if !content.is_empty() {
                    panic!("Response has an unsupported content type.");
                }

                if !headers.is_empty() {
                    let mut headers: Vec<_> = headers.into_iter().collect();
                    headers.sort_unstable_by(|left, right| left.0.cmp(&right.0));

                    let mut response_building = quote::quote!();

                    for (key, header) in headers {
                        let header = header
                            .into_item()
                            .expect("We don't support referenced parameters");

                        let ty = utils::write_param_ref("response headers", &header.format);

                        let ty = if header.required {
                            ty
                        } else {
                            quote::quote! {
                                Option<#ty>
                            }
                        };

                        let field_name =
                            quote::format_ident!("{}", case::convert(&key, case::Case::Snake));

                        fields.push((field_name.clone(), ty));

                        let constructor = quote::quote! {
                            headers.insert(#key, ::ground_openapi::export::into_header_value(#key, #field_name)?);
                        };

                        if header.required {
                            response_building.extend(constructor);
                        } else {
                            response_building.extend(quote::quote! {
                                if let Some(#field_name) = #field_name {
                                    #constructor
                                }
                            });
                        }
                    }

                    let headers = quote::format_ident!("headers");
                    arm_body.extend(quote::quote! {
                        let #headers = {
                            let mut headers = ::ground_openapi::export::http::HeaderMap::new();
                            #response_building
                            headers
                        };
                    });
                    into_response.push(headers);
                }

                let mut destructuring = quote::quote!();
                match fields.len() {
                    0 => {
                        response_variants.extend(quote::quote! {
                            #variant_name,
                        });

                        destructuring.extend(quote::quote! {
                            #variant_name
                        });
                    }
                    1 => {
                        let (name, ty) = fields.swap_remove(0);

                        response_variants.extend(quote::quote! {
                            #variant_name(#ty),
                        });

                        destructuring.extend(quote::quote! {
                            #variant_name {
                                0: #name
                            }
                        });
                    }
                    _ => {
                        let decls = fields
                            .iter()
                            .map(|(field, ty)| {
                                quote::quote! {
                                    #field: #ty,
                                }
                            })
                            .collect::<TokenStream>();

                        response_variants.extend(quote::quote! {
                            #variant_name {
                                #decls
                            },
                        });

                        let names = fields
                            .iter()
                            .map(|(field, _ty)| {
                                quote::quote! {
                                    #field,
                                }
                            })
                            .collect::<TokenStream>();

                        destructuring.extend(quote::quote! {
                            #variant_name {
                                #names
                            }
                        });
                    }
                }

                // We reverse it because we want the `body` field last (axum requirements). :D
                into_response.reverse();

                response_arms.extend(quote::quote! {
                    Response::#destructuring => {
                        let status_code = ::ground_openapi::export::http::StatusCode::from_u16(#code).expect("Status code has already been checked");

                        #arm_body

                        #[allow(unused_parens)]
                        let response = (
                            status_code
                            #(,#into_response)*
                        );

                        ::ground_openapi::export::axum::response::IntoResponse::into_response(response)
                    }
                });
            }

            if let Some(response) = op.responses.default {
                let openapiv3::Response {
                    description: _,
                    headers: _,
                    mut content,
                    links: _,
                    extensions: _,
                } = response
                    .into_item()
                    .expect("We don't support ref responses");

                // Only json is supported
                let response = content
                    .swap_remove("application/json")
                    .expect("Expected application/json response body");
                let response_body = response.schema.expect("Response should have a schema");

                // If the default was swapped out, then we assume that people know what they're doing.
                // If it turns out they don't, then we should add an else branch to this and log stuff out.
                if utils::name(&response_body).ends_with("ApiError") {
                    // These are the codes that ApiError can return.
                    status_codes.extend([400, 401, 403, 404, 409, 412, 422, 500, 502, 503]);
                }
            }

            methods.extend(quote::quote! {
                async fn #route_name(
                    self,
                    req: #route_name::#request_name,
                ) -> ::ground_openapi::export::ApiResult<#route_name::#response_name>;
            });

            routes.extend(quote::quote! {
                .route(#route_name::PATH, #route_name::method_router())
            });

            let ts = quote::quote! {
                use ::ground_openapi::export::axum::extract::State as AxumState;

                use super::Ctx;

                #[allow(dead_code)]
                pub type #request_name = Request;
                #[allow(dead_code)]
                pub struct Request {
                    #request_fields
                }

                #[allow(dead_code)]
                pub type #response_name = Response;
                #[allow(dead_code)]
                pub enum Response {
                    #response_variants
                }

                // #endpoint_module

                pub(super) const PATH: &str = #url;

                pub(super) fn method_router<S>() -> ::ground_openapi::export::axum::routing::MethodRouter<super::Ctx<S>>
                    where
                        S: super::#trait_name + Sized + Clone + Send + Sync + 'static,
                {
                    use ::ground_openapi::export::axum::routing::{on, MethodFilter};

                    on(
                        MethodFilter::#http_method,
                        #route_name::<S>,
                    )
                }

                pub(super) async fn #route_name<S>(
                    AxumState(ctx): AxumState<Ctx<S>>,
                    #handler_params
                ) -> ::ground_openapi::export::ApiResult
                where
                    S: super::#trait_name + Sized + Clone + Send + Sync + 'static,
                {
                    let Ctx { api } = ctx;
                    #handler_body
                    let req = Request {
                        #request_building
                    };
                    let response = api.#route_name(req).await?;

                    let response = match response {
                        #response_arms
                    };

                    if !matches!(response.status().as_u16(), #(#status_codes)|*) {
                        // Undefined status code. (We don't stop the response, but we log a "maybe define this stuff")
                        ::ground_openapi::export::tracing::error!("[api] '{}' returned an undeclared status code '{}'", PATH, response.status().as_u16());
                    }
                    Ok(response)
                }
            };

            api_module
                .create_child_from_path([raw_route_name.as_ref()])
                .extend(ts);
        }
    }

    api_module.extend(quote::quote! {
        #compile_errors

        #[::ground_openapi::export::async_trait]
        pub trait #trait_name {
            #methods
        }

        #[derive(Clone)]
        struct Ctx<S>
            where
                S: #trait_name + Sized + Clone + Send + Sync + 'static,
        {
            api: S,
        }

        #[allow(clippy::redundant_clone)]
        pub fn into_router<S>(
            api: S,
        ) -> ::ground_openapi::export::axum::Router
        where
            S: #trait_name + Sized + Clone + Send + Sync + 'static,
        {
            let router = ::ground_openapi::export::axum::Router::new()
                #routes
                ;

            router.with_state(Ctx {
                api,
            })
        }
    });

    Ok(())
}

fn generate_query_parameter(query: Parameter) -> (TokenStream, TokenStream) {
    let Parameter::Query {
        parameter_data: data,
        allow_reserved: _,
        style: _,
        allow_empty_value: _,
    } = query
    else {
        panic!("This list should only contain elements of type `Query`");
    };

    let raw_name = &data.name;
    let name = if raw_name == "type" {
        quote::format_ident!("r#type")
    } else {
        quote::format_ident!("{}", case::convert(raw_name, case::Case::Snake))
    };

    let parameter_ty = utils::write_param_ref("query parameters", &data.format);

    // If the type can handle multiple values, that determines how we interact with query parameters.
    // We determine this by extracting the information during definition time.
    // @TODO jezza - 07 Oct 2024: This should be provided by the input source.
    let expecting_array = false;

    let parameter_ty = match (data.required, expecting_array) {
        (true, _) | (false, true) => {
            // If the data is required OR we're expecting an array (as an empty array is good enough)
            quote::quote!(#parameter_ty)
        }
        (false, false) => {
            quote::quote!(Option<#parameter_ty>)
        }
    };

    // @CLEANUP jezza - 27 Apr 2023: This is yucky...
    let func = match (data.required, expecting_array) {
        (false, true) => quote::quote!(parse_query_array_opt),
        (false, false) => quote::quote!(parse_query_opt),
        (true, true) => quote::quote!(parse_query_array),
        (true, false) => quote::quote!(parse_query),
    };

    let parser = quote::quote! {
        ::ground_openapi::export::#func(&mut query_map, #raw_name)?
    };

    (
        quote::quote! {
            pub #name: #parameter_ty,
        },
        quote::quote! {
            #name: #parser,
        },
    )
}
