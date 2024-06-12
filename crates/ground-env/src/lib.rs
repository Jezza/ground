pub use ground_env_derive::FromEnv;
use std::borrow::Cow;

#[cfg(test)]
mod tests;

pub struct Context<'a> {
    pub prefix: std::cell::Cell<Option<&'a str>>,
    pub env: std::collections::HashMap<String, Result<String, std::ffi::OsString>>,
}

impl<'prefix> Context<'prefix> {
    pub fn env() -> Self {
        let env = std::env::vars_os()
            .filter_map(|(key, value)| {
                // Invalid key => missing key? (Hard to debug if you've messed up the key)
                let key = key.into_string().ok()?;
                // Invalid value => we can store the result.
                let value = value.into_string();
                Some((key, value))
            })
            .collect::<std::collections::HashMap<_, _>>();

        Self {
            prefix: std::cell::Cell::new(None),
            env,
        }
    }

    pub fn empty() -> Self {
        Self {
            prefix: std::cell::Cell::new(None),
            env: Default::default(),
        }
    }

    #[doc(hidden)]
    pub fn with_prefix<'a: 'prefix, T: FromEnv>(&self, prefix: Option<&'a str>) -> Result<T> {
        let old_prefix = self.prefix.replace(prefix);
        let out = T::from_ctx(self);
        self.prefix.replace(old_prefix);
        out
    }

    #[doc(hidden)]
    pub fn resolve(&self, key: &'static str) -> Result<Result<&str, String>> {
        let key_alloc;
        let key = if let Some(prefix) = self.prefix.get() {
            key_alloc = format!("{}{}", prefix, key);
            Cow::Owned(key_alloc)
        } else {
            Cow::Borrowed(key)
        };

        match self.env.get(key.as_ref()) {
            Some(t) => match t {
                Ok(va) => Ok(Ok(va.as_ref())),
                Err(o) => Err(Error::NotUnicode(key.into_owned(), o.clone())),
            },
            None => Ok(Err(key.into_owned())),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to locate '{0}' within environment")]
    Missing(String),
    #[error("Unable to convert '{0}' into UTF-8")]
    NotUnicode(String, std::ffi::OsString),
    #[error("Parsing '{input}' as '{ty}' failed: {err}")]
    Parse {
        err: String,
        input: String,
        ty: &'static str,
    },
}

pub trait FromEnv: Sized {
    fn from_env() -> Result<Self> {
        Self::from_ctx(&Context::env())
    }

    fn from_ctx(ctx: &Context<'_>) -> Result<Self>;
}

pub trait Parse: Sized {
    fn parse(input: &str) -> Result<Self>;
}

impl<T, E> Parse for T
    where
        T: std::str::FromStr<Err=E>,
        E: std::error::Error,
{
    fn parse(value: &str) -> Result<Self> {
        std::str::FromStr::from_str(value).map_err(|err: E| Error::Parse {
            err: err.to_string(),
            input: value.to_string(),
            ty: std::any::type_name::<Self>(),
        })
    }
}

#[doc(hidden)]
pub fn transpose_err<T, E, U>(result: Result<Result<T, U>, E>) -> Result<Result<T, E>, U> {
    match result {
        Ok(result) => match result {
            Ok(value) => Ok(Ok(value)),
            Err(err) => Err(err),
        },
        Err(err) => Ok(Err(err)),
    }
}

// Result<T, Result<T, E>> -> Result<T, E>
#[doc(hidden)]
pub fn flatten_err<T, E>(result: Result<T, Result<T, E>>) -> Result<T, E> {
    match result {
        Ok(value) => Ok(value),
        Err(result) => match result {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        },
    }
}
