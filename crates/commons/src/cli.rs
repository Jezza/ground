//! Common functionality related to CLI tools

#[cfg(feature = "cli-postgres")]
pub mod postgres;

#[cfg(feature = "cli-service")]
pub mod service;

#[cfg(feature = "cli-tracing")]
pub mod tracing;

#[cfg(feature = "cli-otel")]
pub mod otel;
