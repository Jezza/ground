use std::path::Path;

use jrsonnet_evaluator::parser::SourcePath;
use jrsonnet_gcmodule::Tracer;
use jrsonnet_parser::SourceFile;

pub struct FileTracer<T> {
    original: T,
}

impl<T> FileTracer<T> {
    pub fn new(value: T) -> Self {
        Self { original: value }
    }
}

impl<T: jrsonnet_gcmodule::Trace> jrsonnet_gcmodule::Trace for FileTracer<T> {
    fn trace(&self, tracer: &mut Tracer) {
        self.original.trace(tracer)
    }

    fn is_type_tracked() -> bool
    where
        Self: Sized,
    {
        T::is_type_tracked()
    }
}

impl<T: jrsonnet_evaluator::ImportResolver> jrsonnet_evaluator::ImportResolver for FileTracer<T> {
    fn resolve_from(
        &self,
        from: &SourcePath,
        path: &str,
    ) -> jrsonnet_evaluator::Result<SourcePath> {
        self.original.resolve_from(from, path)
    }

    fn resolve_from_default(&self, path: &str) -> jrsonnet_evaluator::Result<SourcePath> {
        self.original.resolve_from_default(path)
    }

    fn resolve(&self, path: &Path) -> jrsonnet_evaluator::Result<SourcePath> {
        self.original.resolve(path)
    }

    fn load_file_contents(&self, resolved: &SourcePath) -> jrsonnet_evaluator::Result<Vec<u8>> {
        if let Some(source) = resolved.downcast_ref::<SourceFile>() {
            println!("cargo:rerun-if-changed={}", source.path().display());
        }
        self.original.load_file_contents(resolved)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
