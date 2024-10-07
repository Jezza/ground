use crate::utils;
use ground_codegen_utils::module::Module;
use ground_openapi_utils as api_utils;
use openapiv3::ReferenceOr;

use to_schema::ToSchema;

mod to_schema;

pub fn generate_models(
    root: &mut Module,
    components: &openapiv3::Components,
) -> anyhow::Result<()> {
    let schemas = components.schemas.iter().filter_map(|(_key, schema)| {
        let ReferenceOr::Item(schema) = schema else {
            // We don't write references, as they're just that...
            return None;
        };
        let ext = &schema.schema_data.extensions;

        if api_utils::check_ext(ext, "x-ground-external") {
            // We don't write externals, as they're externally managed.
            return None;
        }

        let name = api_utils::ext_str(ext, "x-ground-name")?;

        let schema = schema.to_schema();

        Some((name, schema))
    });

    let grouped_schemas = utils::group_by(schemas, |(name, schema)| {
        let (group, name) = name.split_once('.').unwrap_or(("default", name));

        (group, (name, schema))
    });

    let settings = typify_impl::TypeSpaceSettings::default();

    for (group, schemas) in grouped_schemas {
        let mut ts = typify_impl::TypeSpace::new(&settings);

        ts.add_ref_types(schemas).expect("Unable to add types");

        let module = root.create_child(group);
        module.extend(quote::quote! {
            use ::ground_openapi::export::*;

            #ts
        });
    }

    Ok(())
}
