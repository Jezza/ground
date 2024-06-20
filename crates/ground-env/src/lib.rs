//! `ground-env` is used to parse environment variables into structs
//!
//! Usage example:
//! ```
//! #[derive(FromEnv)]
//! struct Config {
//!     text: String,
//!     optional_text: Option<String>,
//!     // The default delimiter is set to ","
//!     list: Vec<String>,
//!     #[env(delimiter = " ")] // But you can customise it.
//!     names: Vec<String>,
//!     number: i64,
//!     // Supports renaming.
//!     #[env(rename = "EMAIL_ADDRESS")]
//!     email: String,
//!     #[env(default)] // Defaults to 0 when no explicit value is provided.
//!     count: i64,
//!     #[env(default = "64")] // Defaults to 64 when not provided.
//!     background_tasks: i64,
//!     #[env(flatten)] // You can flatten other structs
//!     admin_credentials: Credentials,
//!     #[env(flatten = "DB_")] // You can also provide a prefix.
//!     db_args: Credentials,
//! }
//!
//! #[derive(FromEnv)]
//! struct Credentials {
//!     username: String,
//!     password: String,
//! }
//!
//! fn main() -> anyhow::Result<()> {
//!     let t = Config::from_env()?;
//!     Ok(())
//! }
//! ```

/// re-export of the derive macro powering `ground-env`
pub use ground_env_derive::FromEnv;
use std::borrow::Cow;

#[cfg(test)]
mod tests;

/// context containing an optional prefix and a map of environment variables
pub struct Context<'a> {
    pub prefix: std::cell::Cell<Option<&'a str>>,
    pub env: std::collections::HashMap<String, Result<String, std::ffi::OsString>>,
}

impl<'prefix> Default for Context<'prefix> {
    fn default() -> Self {
        Self::env()
    }
}

impl<'prefix> Context<'prefix> {
    /// create a new context from current environment
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

    /// create an empty context
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

/// a [std::result::Result] returning the `ground-env` [Error]
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// enumeration of possible `ground-env` errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// environment variable cannot be found
    #[error("Unable to locate '{0}' within environment")]
    Missing(String),
    /// environment variable was found but does not contain valid unicode data
    #[error("Unable to convert '{0}' into UTF-8")]
    NotUnicode(String, std::ffi::OsString),
    #[error("Parsing '{input}' as '{ty}' failed: {err}")]
    /// failed to parse the environment variable's string value into an actual data model
    Parse {
        err: String,
        input: String,
        ty: &'static str,
    },
}

/// usualy you don't implement this trait, as it is automatically implemented by the derive macro
pub trait FromEnv: Sized {
    /// read [Self] from environment
    fn from_env() -> Result<Self> {
        Self::from_ctx(&Context::env())
    }

    /// read [Self] from a specific [Context]
    fn from_ctx(ctx: &Context<'_>) -> Result<Self>;
}

/// implement this trait for types that you can parse from an environment variable's [String] value.
///
/// this is implemented for types implementing [std::str::FromStr]
pub trait Parse: Sized {
    /// parse [Self] from `input`
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
