use std::io::Read as _;
use std::path::Path;

use crate::importers::SourceEmbed;
use jrsonnet_evaluator::parser::{SourceDirectory, SourceFile, SourcePath, SourceVirtual};
use jrsonnet_evaluator::ImportResolver;
use jrsonnet_evaluator::{error::ErrorKind, Result};
use jrsonnet_gcmodule::Tracer;

#[derive(Debug)]
pub enum FromSource<'s> {
    Virtual(&'s str),
    #[allow(unused)]
    Physical(&'s Path),
}

#[derive(Default)]
pub struct FnImportResolver<F> {
    resolver: F,
}

impl<F> FnImportResolver<F> {
    pub fn new(func: F) -> Self {
        Self { resolver: func }
    }
}

impl<F: 'static> jrsonnet_gcmodule::Trace for FnImportResolver<F> {
    fn trace(&self, tracer: &mut Tracer) {
        let _tracer = tracer;
    }

    fn is_type_tracked() -> bool
    where
        Self: Sized,
    {
        false
    }
}

impl<F> ImportResolver for FnImportResolver<F>
where
    F: for<'s, 'n> Fn(FromSource<'s>, &'n str) -> Result<Option<SourcePath>> + 'static,
{
    fn resolve_from(&self, from: &SourcePath, path: &str) -> Result<SourcePath> {
        if let Some(source) = from.downcast_ref::<SourceEmbed>() {
            let source = FromSource::Virtual(&source.path);

            return if let Some(resolved) = (self.resolver)(source, path)? {
                Ok(resolved)
            } else {
                Err(ErrorKind::ImportFileNotFound(from.clone(), path.to_owned()))?
            };
        }

        let mut direct = if let Some(f) = from.downcast_ref::<SourceFile>() {
            let mut o = f.path().to_owned();
            o.pop();
            o
        } else if let Some(d) = from.downcast_ref::<SourceDirectory>() {
            d.path().to_owned()
        } else if from.is_default() {
            std::env::current_dir().map_err(|e| ErrorKind::ImportIo(e.to_string()))?
        } else {
            // Can't resolve it, as there's no physical link anymore.
            return Err(ErrorKind::ImportFileNotFound(from.clone(), path.to_owned()))?;
        };

        let source = FromSource::Physical(&direct);
        if let Some(resolved) = (self.resolver)(source, path)? {
            return Ok(resolved);
        }

        direct.push(path);

        if direct.is_file() {
            let path = direct
                .canonicalize()
                .map_err(|e| ErrorKind::ImportIo(e.to_string()))?;

            Ok(SourcePath::new(SourceFile::new(path)))
        } else {
            Err(ErrorKind::ImportFileNotFound(from.clone(), path.to_owned()))?
        }
    }

    fn resolve(&self, path: &Path) -> Result<SourcePath> {
        {
            let name = path.file_name().and_then(|path| path.to_str());
            if let Some(name) = name {
                let mut path = path.to_path_buf();
                path.pop();

                let source = FromSource::Physical(&path);

                if let Some(source) = (self.resolver)(source, name)? {
                    return Ok(source);
                }
            }
        }

        let meta = match std::fs::metadata(path) {
            Ok(v) => v,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Err(ErrorKind::AbsoluteImportFileNotFound(path.to_owned()))?
            }
            Err(e) => Err(ErrorKind::ImportIo(e.to_string()))?,
        };

        let path = path
            .canonicalize()
            .map_err(|e| ErrorKind::ImportIo(e.to_string()))?;

        if meta.is_file() {
            Ok(SourcePath::new(SourceFile::new(path)))
        } else if meta.is_dir() {
            Ok(SourcePath::new(SourceDirectory::new(path)))
        } else {
            unreachable!("this can't be a symlink")
        }
    }

    fn load_file_contents(&self, id: &SourcePath) -> Result<Vec<u8>> {
        if let Some(source) = id.downcast_ref::<SourceEmbed>() {
            return Ok(source.data.clone());
        }

        let path = if let Some(f) = id.downcast_ref::<SourceFile>() {
            f.path()
        } else if let Some(f) = id.downcast_ref::<SourceVirtual>() {
            return Ok(f.0.as_bytes().to_vec());
        } else if id.downcast_ref::<SourceDirectory>().is_some() || id.is_default() {
            Err(ErrorKind::ImportIsADirectory(id.clone()))?
        } else {
            unreachable!("other types are not supported in resolve");
        };
        let mut file =
            std::fs::File::open(path).map_err(|_e| ErrorKind::ResolvedFileNotFound(id.clone()))?;
        let mut out = Vec::new();
        file.read_to_end(&mut out)
            .map_err(|e| ErrorKind::ImportIo(e.to_string()))?;
        Ok(out)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
