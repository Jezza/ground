pub mod error;
pub mod extract;
pub mod handlers;

#[doc(hidden)]
pub mod export {
    pub use axum;
    pub use headers_core;
    pub use http;
    pub use http_body;
    pub use mime;
    // pub use regex;
    pub use serde;
    pub use serde_json;
    pub use tracing;

    pub use crate::error::{ApiError, ApiResult};
    pub use crate::extract::{Json, JsonRejectionError, QueryMap};
    pub use async_trait::async_trait;

    pub use crate::handlers::*;

    pub type RawPathResult = Result<
        axum::extract::path::RawPathParams,
        axum::extract::rejection::RawPathParamsRejection,
    >;

    pub type RawBody<T> = Result<Json<T>, JsonRejectionError>;
}
