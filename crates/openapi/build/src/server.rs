use crate::{models, routes};
use ground_codegen_utils::module::Module;

pub fn generate_server(api: openapiv3::OpenAPI) -> anyhow::Result<()> {
    generate_server_with_opts(api, "./src/", "crate")
}

pub fn generate_server_with_opts(
    api: openapiv3::OpenAPI,
    target: impl AsRef<std::path::Path>,
    module: impl AsRef<str>,
) -> anyhow::Result<()> {
    let module = module.as_ref();
    let (path, module) = module.rsplit_once("::").unwrap_or(("", module));
    let module_path = path.split("::").filter(|s| !s.is_empty());

    let mut base = if path.starts_with("crate") {
        Module::new_with_path(module, module_path)
    } else {
        let path = std::iter::once("crate").chain(module_path);
        Module::new_with_path(module, path)
    };

    let openapiv3::OpenAPI {
        openapi: _,
        info: _,
        servers: _,
        paths,
        components,
        security: _,
        tags: _,
        external_docs: _,
        extensions: _,
    } = api;

    let components = components.unwrap_or_default();

    // @TODO jezza - 07 Oct 2024:
    //   1. Hoist all schemas to the components.
    //   2. Generate a "reasonable" name for each root component
    //   3. Strip all prefixes for isolated processing.

    {
        let api = base.create_child("api");

        routes::generate_routes(api, &components, paths)?;
    }

    {
        let models = base.create_child("models");

        models::generate_models(models, &components)?;
    }

    let target = target.as_ref();
    for child in &base.children {
        child.write(target)?;
    }

    Ok(())
}
