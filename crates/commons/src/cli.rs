//! Common functionality related to CLI tools

#[cfg(all(
    feature = "cli-postgres",
    any(
        feature = "env",
        feature = "clap",
    )
))]
pub mod postgres;

#[cfg(all(
    feature = "cli-service",
    any(
        feature = "env",
        feature = "clap",
    )
))]
pub mod service;
