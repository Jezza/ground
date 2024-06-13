use regex::Regex;
use std::borrow::Cow;
use std::{
    fmt::Display,
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};

// `ServiceName` always contains a valid SQL identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceName(String);

impl ServiceName {
    pub fn random() -> Self {
        uuid::Uuid::new_v4()
            .hyphenated()
            .to_string()
            .parse()
            .expect("converting to string and back always works")
    }

    pub fn utils() -> Self {
        "utils".parse().expect("utils is a vaild service name")
    }

    pub fn as_db_name(&self) -> Cow<str> {
        if self.0.contains('-') {
            let name = format!(r#""{}""#, self.0);
            Cow::Owned(name)
        } else {
            Cow::Borrowed(&self.0)
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Display for ServiceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<Path> for ServiceName {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl FromStr for ServiceName {
    type Err = ServiceNameError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        // @TODO jeremy.barrow - 13 June 2024: Replace this with a `.bytes().all(...)`
        let allowed_chars =
            Regex::new(r"^[a-zA-Z0-9_\-]+$").expect("const regex will always compile");

        let name = name.trim();

        if name.is_empty() || !allowed_chars.is_match(name) {
            return Err(ServiceNameError::InvalidFormat);
        }

        Ok(Self(name.into()))
    }
}

impl Deref for ServiceName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceNameError {
    #[error("Invalid format. Names can only contain letters, numbers, and underscores.")]
    InvalidFormat,
}
