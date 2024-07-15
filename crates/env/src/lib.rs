pub use ground_env_derive::FromEnv;
use std::borrow::Cow;

#[cfg(test)]
mod tests;

pub struct Context {
    prefix: Vec<&'static str>,
    env: std::collections::HashMap<String, Result<String, std::ffi::OsString>>,
}

impl Context {
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
            prefix: vec![],
            env,
        }
    }

    pub fn empty() -> Self {
        Self {
            prefix: vec![],
            env: Default::default(),
        }
    }

    #[doc(hidden)]
    pub fn with_prefix<T: FromEnv>(&mut self, prefix: &'static str) -> Result<T> {
        self.prefix.push(prefix);

        let out = T::from_ctx(self);

        let old = self.prefix.pop();

        assert!(old.is_some(), "Any operation on the prefix should be self-contained. [Something being flattened removed an extra segment]");

        out
    }

    #[doc(hidden)]
    pub fn resolve(&self, key: &'static str) -> Result<Result<&str, String>> {
        let mut key_alloc;

        let key = if self.prefix.is_empty() {
            Cow::Borrowed(key)
        } else {
            let len = self.prefix.iter()
                .map(|item| item.len())
                .sum::<usize>()
                + key.len();
            key_alloc = String::with_capacity(len);
            for prefix in self.prefix.iter() {
                key_alloc.push_str(prefix);
            }
            key_alloc.push_str(key);
            Cow::Owned(key_alloc)
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
        Self::from_ctx(&mut Context::env())
    }

    fn from_ctx(ctx: &mut Context) -> Result<Self>;
}

pub trait Parse: Sized {
    fn parse(input: &str) -> Result<Self>;
}

impl<T, E> Parse for T
    where
        T: std::str::FromStr<Err = E>,
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
