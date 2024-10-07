use std::collections::HashMap;

use openapiv3::{ParameterSchemaOrContent, PathItem, ReferenceOr};

pub use ground_openapi_utils::*;

#[derive(Debug)]
pub enum ClassifiedPathItem {
    Item {
        url: String,
        item: PathItem,
    },
    Alias {
        url: String,
        // operation_id, security_model
        items: Vec<(String, ())>,
    },
}

impl ClassifiedPathItem {
    pub fn url(&self) -> &str {
        match self {
            ClassifiedPathItem::Item { url, .. } => url,
            ClassifiedPathItem::Alias { url, .. } => url,
        }
    }

    /// This is just the url, without the first segment (if it looks like a version (v\d\d)). (Which is typically the version: "v0", "inc", etc)
    pub fn url_without_version(&self) -> &str {
        let url = self.url();

        let raw_url = match url.as_bytes() {
            // Check if it starts with "/v"
            [b'/', b'v', rest @ ..] => {
                let Some(next_slash) = rest.iter().position(|c| *c == b'/') else {
                    eprintln!("Lacks a first segment: {}", url);
                    return url;
                };

                // That first segment isn't a version, so just return the whole thing.
                let version = (&rest[..next_slash]).iter().all(u8::is_ascii_digit);
                if !version {
                    eprintln!("First segment not v\\d\\d: {}", url);
                    return url;
                }
                // Safety: We're not splitting bytes in half. (we're checking against ascii)
                &rest[(next_slash + 1)..]
            }
            // Check if it starts with "/inc/"
            [b'/', b'i', b'n', b'c', b'/', url @ ..] => url,
            _ => {
                return url;
            }
        };

        std::str::from_utf8(raw_url)
            .expect("Internal error. [Bytes taken from a string don't reconstruct into a string]")
    }
}

// @TODO jezza - 07 Oct 2024: Add support for non-unique items
pub fn group_path_items(paths: Obj<PathItem>) -> HashMap<String, Vec<ClassifiedPathItem>> {
    const DEFAULT_GROUP: &str = "default";

    let grouped_paths = group_by(paths, |(url, mut item): (_, ReferenceOr<PathItem>)| {
        let ext_prefix = match &mut item {
            ReferenceOr::Item(item) => {
                // While we're here, we can also fix the operation_id
                // Here, we do the same thing as with the url.

                // operation_id _must_ be unique, but during generation, we could want to generate multiple variations of the same api.
                // Because of this, they look identical as they just reuse the same ops, meaning the operation ids aren't unique.
                // To solve that, we prefix the operation id with the api name during the merge phase (the builder step).
                // Here, we have to remove it during code generation, because we want the original names.

                visit_op_mut(item, |_, op| {
                    if let Some(id) = op.operation_id.as_mut() {
                        let (_, raw_id) = id.split_once("__").unwrap_or(("", &**id));
                        *id = String::from(raw_id);
                    }
                });

                ext_str(&item.extensions, "x-ground-name")
            }
            ReferenceOr::Reference { reference } => {
                // This is what json pointers look like:
                // reference = #/paths/~1v0~1segment~1thing

                // target = ~1v0~1segment~1thing
                let (_, target) = reference
                    .rsplit_once('/')
                    .expect("Path reference should have a root");

                // Rust, y u no have in-place replace. :(
                // target = /v0/segment/thing
                *reference = target.replace("~1", "/").replace("~0", "~");

                None
            }
        };

        // To avoid collisions, we prefix the url with the api name.
        // Here, we remove that prefix, so we have the original url
        // For more information, take a look at `builder::Opts`
        let (prefix, url): (&str, &str) = url.split_once("__").unwrap_or((DEFAULT_GROUP, &url));
        let prefix = prefix.strip_prefix('/').unwrap_or(prefix);

        let prefix = ext_prefix.unwrap_or(prefix);

        (String::from(prefix), (String::from(url), item))
    });

    grouped_paths
        .into_iter()
        .map(|(group, path_items)| {
            let mut classified = vec![];

            // We want to separate the references first because we need to go through the current items to find the aliased item.
            let (references, items): (Vec<_>, Vec<_>) = path_items
                .into_iter()
                .partition(|(_, item)| matches!(item, ReferenceOr::Reference { .. }));

            for (url, reference) in references {
                let ReferenceOr::Reference { reference: target } = reference else {
                    panic!("Only references should be in this list.");
                };

                let (_, item) = items
                    .iter()
                    .find(|(url, _)| *url == target)
                    .expect("Unknown alias");
                let item = item.as_item().expect("Only items should be in this list");

                let items = item
                    .iter()
                    .map(|(_, op)| {
                        let id = op
                            .operation_id
                            .as_deref()
                            .expect("operation_id should always be present");
                        // let claims_type = ClaimsType::from_op(op);
                        let security_model = ();

                        (String::from(id), security_model)
                    })
                    .collect::<Vec<_>>();

                classified.push(ClassifiedPathItem::Alias { url, items });
            }

            for (url, item) in items {
                let item = item.into_item().expect("Only items should be in this list");

                classified.push(ClassifiedPathItem::Item { url, item });
            }

            // This is just a nice touch.
            // All endpoints related to each other will be grouped together.
            classified.sort_unstable_by(|left, right| {
                let left = left.url_without_version();
                let right = right.url_without_version();

                left.cmp(right)
            });

            (group, classified)
        })
        .collect()
}

pub fn group_by<T, GK: Eq + std::hash::Hash, GV>(
    items: impl IntoIterator<Item = T>,
    group: impl Fn(T) -> (GK, GV),
) -> HashMap<GK, Vec<GV>> {
    let mut groups: HashMap<GK, Vec<GV>> = HashMap::new();
    for item in items {
        let (key, value) = group(item);
        groups.entry(key).or_default().push(value);
    }
    groups
}

pub fn name(schema: &ReferenceOr<openapiv3::Schema>) -> &str {
    match &schema {
        ReferenceOr::Item(item) => {
            // @TODO jezza - 01 Oct 2024: Fix this shit
            let name = item
                .schema_data
                .extensions
                .get("x-ground-name")
                .expect("ResponseBody must have a name");
            match name {
                serde_json::Value::String(name) => name,
                _ => {
                    panic!("Expected string");
                }
            }
        }
        ReferenceOr::Reference { reference } => reference,
    }
}

fn segments(path: &str) -> Vec<proc_macro2::Ident> {
    path.split('.')
        .map(|ident| quote::format_ident!("{}", ident))
        .collect()
}

pub fn write_ref(
    region: &'static str,
    schema: &ReferenceOr<openapiv3::Schema>,
) -> proc_macro2::TokenStream {
    // At this point, we're expecting all types to be "writeable in-place".
    // Nothing should need specific handling.

    match schema {
        ReferenceOr::Item(schema) => {
            use openapiv3::SchemaKind as Kind;
            use openapiv3::Type as Ty;
            match &schema.schema_kind {
                Kind::Type(Ty::String(_ty)) => quote::quote! {
                    String
                },
                Kind::Type(Ty::Boolean(_ty)) => quote::quote! {
                    bool
                },
                Kind::Type(Ty::Integer(_ty)) => quote::quote! {
                    i32
                },
                Kind::Type(Ty::Number(_ty)) => quote::quote! {
                    i32
                },
                _ => {
                    panic!(
                        "Only references or simple types are expected here. [{}]",
                        region
                    );
                }
            }
        }
        ReferenceOr::Reference { reference } => Ref::new(reference).write_path(None),
    }
}

pub fn write_param_ref(
    region: &'static str,
    schema: &ParameterSchemaOrContent,
) -> proc_macro2::TokenStream {
    let ParameterSchemaOrContent::Schema(schema) = schema else {
        panic!("Only schemas are supported. [{}]", region);
    };
    write_ref(region, &schema)
}

pub struct Ref {
    pub external: bool,
    pub reference: String,
}

impl Ref {
    pub fn new(reference: impl Into<String>) -> Self {
        let mut reference = reference.into();

        let external = reference.starts_with("external:");

        let index = reference
            .rfind('/')
            .unwrap_or_else(|| panic!("Unknown reference: {}", reference));

        reference.replace_range(0..index + 1, "");

        Self {
            external,
            reference,
        }
    }

    pub fn model_name(&self) -> Option<(&str, &str)> {
        self.reference.rsplit_once('.')
    }

    pub fn write_path(&self, prefix: Option<&str>) -> proc_macro2::TokenStream {
        let model_name = self.model_name();
        match (prefix, model_name) {
            (Some(prefix), Some((path, model_name))) if prefix == path => {
                let name = quote::format_ident!("{}", model_name);
                quote::quote! {
                    #name
                }
            }
            _ => {
                if self.external {
                    write_external(&self.reference)
                } else {
                    let name: Vec<_> = segments(&self.reference);

                    quote::quote! {
                        crate::models::#(#name)::*
                    }
                }
            }
        }
    }
}

fn write_external(name: &str) -> proc_macro2::TokenStream {
    let segments = segments(name);
    quote::quote! {
        ::ground_openapi::external::#(#segments)::*
    }
}

// fn handle_external(ext: &utils::Ext) -> Option<TokenStream> {
//     if utils::is_external(ext) {
//         let name = ext
//             .get(super::GROUND_NAME)
//             .expect("External types require a name");
//         let name = match name {
//             serde_json::Value::String(value) => &**value,
//             _ => panic!("External name should be a string"),
//         };
//         Some(write_external(name))
//     } else {
//         None
//     }
// }
