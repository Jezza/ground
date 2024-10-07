use anyhow::Context;
use ground_openapi_utils::visit_op_mut;
use ground_openapi_utils::visitor::type_visit_mut::TypeVisitMut;
use ground_openapi_utils::visitor::visit_mut::VisitMut;
use openapiv3::{Paths, ReferenceOr, Schema};
use serde_json::Value;

pub struct OpenApiSettings {
    /// Remove all fields that contain `x-ground`. (Some generators don't ignore extensions)
    pub remove_ground_extensions: bool,
}

impl Default for OpenApiSettings {
    fn default() -> Self {
        Self {
            remove_ground_extensions: true,
        }
    }
}

pub fn write_openapi(
    mut api: openapiv3::OpenAPI,
    target: impl AsRef<std::path::Path>,
) -> anyhow::Result<()> {
    VisitMut::visit_openapi_mut(
        &mut OpenApiCleaner {
            opts: Default::default(),
        },
        &mut api,
    );

    let text = serde_json::to_string_pretty(&api).context("Unable to represent OpenApi as json")?;

    let target = target.as_ref();

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).context("Unable to create parent directories.")?;
    }

    std::fs::write(target, text.as_bytes()).context("Unable to write openapi doc")?;

    Ok(())
}

struct OpenApiCleaner {
    opts: OpenApiSettings,
}

impl<'a> TypeVisitMut<'a> for OpenApiCleaner {
    type Outcome = ();

    fn visit_reference_or_schema_mut(&mut self, node: &'a mut ReferenceOr<Schema>) {
        let item = match node {
            ReferenceOr::Item(item) => item,
            ReferenceOr::Reference { reference } => {
                if let Some(stripped) = reference.strip_prefix("external:") {
                    *reference = String::from(stripped);
                }
                return;
            }
        };

        self.visit_schema_mut(item);
    }

    fn visit_reference_or_box_schema_mut(&mut self, node: &'a mut ReferenceOr<Box<Schema>>) {
        let item = match node {
            ReferenceOr::Item(item) => item.as_mut(),
            ReferenceOr::Reference { reference } => {
                if let Some(stripped) = reference.strip_prefix("external:") {
                    *reference = String::from(stripped);
                }
                return;
            }
        };

        self.visit_schema_mut(item);
    }

    fn visit_extensions_mut(&mut self, node: &'a mut indexmap::IndexMap<String, Value>) {
        if self.opts.remove_ground_extensions {
            node.retain(|key, _| !key.starts_with("x-ground"));
        }
    }
}

impl<'a> VisitMut<'a> for OpenApiCleaner {
    fn visit_paths_mut(&mut self, node: &'a mut Paths) {
        let mut map = indexmap::IndexMap::new();
        std::mem::swap(&mut map, &mut node.paths);

        // Each path item contains the name of the originating api it came from.
        // We do this so we can differentiate differing routes and operation ids.
        // API routes that come from the same api will collide.
        for (mut url, mut item) in map.into_iter() {
            // if !self.opts.include_hidden_routes && path.starts_with("/inc/") {
            //     continue;
            // }

            if let ReferenceOr::Item(item) = &mut item {
                visit_op_mut(item, |_, op| {
                    if let Some(id) = op.operation_id.as_mut() {
                        let (_group, raw_id) = id.split_once("__").unwrap_or(("", &**id));
                        *id = String::from(raw_id);
                    }
                });
            }

            let url = if let Some(pos) = url.find("__") {
                url.replace_range(0..(pos + 2), "");
                url
            } else {
                url
            };

            if let Some(previous) = node.paths.insert(url.clone(), item) {
                panic!("URL collision: {}: {:#?}", url, previous);
            }
        }
    }
}
