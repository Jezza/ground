use std::path::{Path, PathBuf};

use anyhow::Context as _;
use jrsonnet_evaluator::manifest::JsonFormat;
use jrsonnet_evaluator::trace::TraceFormat as _;

pub use crate::builder::BuilderOpts;
use crate::importers::{resolve_embed, resolve_from};

mod builder;
mod importers;

// This is used to name the types.
// Colloquially, this means that anything that it is generated. (Except externals)
pub const GROUND_NAME: &str = "x-ground-name";

// These types are known and managed by us.
// This means the types aren't generated, and need to be manually maintained.
pub const GROUND_EXTERNAL: &str = "x-ground-external";

// This is used to help organise the code during generation.
// Routes that have the same tag will be generated and placed in the same trait.
// (Note, this means that we're expecting only a single tag per route, although the spec says they can have any amount of tags)
pub const GROUND_TAGS: &str = "x-ground-tags";

// This is used to pass external types to the generator, so it can shove it into the file openapi doc.
//
// Because of how jsonnet works, we ended up passing around a lot of duplicate data.
// (eg, each response contains the complete schema for `ApiError`)
//
// The solution was to only expose references in the jsonnet world.
// The references, however, need to point to valid nodes in the document.
// So this key is used to shuffle the data into the generator.
// You can see this at work in `builder.rs`
pub const GROUND_EXTERNAL_TYPES: &str = "x-ground-external-types";

#[derive(rust_embed::Embed)]
#[folder = "api/"]
#[prefix = "api/"]
struct ApiFiles;

pub fn eval_paths(path: Vec<impl AsRef<Path>>) -> anyhow::Result<openapiv3::OpenAPI> {
    let workspace = PathBuf::from(".");
    eval_paths_with_opts(path, Default::default(), &workspace)
}

fn eval_paths_with_opts(
    path: Vec<impl AsRef<Path>>,
    opts: BuilderOpts,
    workspace: &Path,
) -> anyhow::Result<openapiv3::OpenAPI> {
    let paths = eval::<openapiv3::Paths>(workspace, path)?;
    Ok(builder::build(opts, paths)?)
}

pub fn eval<T: serde::de::DeserializeOwned>(
    workspace: &Path,
    path: Vec<impl AsRef<Path>>,
) -> anyhow::Result<Vec<T>> {
    let state = jrsonnet_evaluator::State::default();

    path.into_iter()
        .map(|path| {
            let path = path.as_ref();

            let text = eval_file(&state, workspace, &path)?;
            let paths = serde_json::from_str::<T>(&text)?;
            Ok(paths)
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

fn eval_file(
    state: &jrsonnet_evaluator::State,
    workspace: &Path,
    path: &Path,
) -> anyhow::Result<String> {
    let parent = path.parent().context("Unable to determine parent path.")?;

    let ctx = jrsonnet_stdlib::ContextInitializer::new(
        state.clone(),
        jrsonnet_evaluator::trace::PathResolver::FileName,
    );

    let tracer = importers::FileTracer::new({
        let folders = vec![parent.to_path_buf()];

        importers::from_fn(move |from, path| {
            if let Some(source) = resolve_from(&folders, path)? {
                return Ok(Some(source));
            }

            if let Some(source) = resolve_embed::<ApiFiles>(from, path)? {
                return Ok(Some(source));
            }

            Ok(None)
        })
    });

    state.set_context_initializer(ctx);
    state.set_import_resolver(tracer);

    fn format_error(workspace: PathBuf, error: &jrsonnet_evaluator::Error) -> anyhow::Error {
        let format = jrsonnet_evaluator::trace::CompactFormat {
            resolver: jrsonnet_evaluator::trace::PathResolver::Relative(workspace.to_path_buf()),
            max_trace: 20,
            padding: 4,
        };
        match format.format(error) {
            Ok(formatted) => anyhow::anyhow!("{}", formatted),
            Err(_) => anyhow::anyhow!("{}", error),
        }
    }

    let val = state
        .import(path)
        .map_err(|err| format_error(workspace.to_path_buf(), &err))?;

    let formatter = JsonFormat::minify(true);

    let text = val
        .manifest(formatter)
        .map_err(|err| format_error(workspace.to_path_buf(), &err))?;

    Ok(String::from(text.as_str()))
}
