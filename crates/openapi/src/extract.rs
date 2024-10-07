use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts},
    http::request::Parts,
};
use bytes::{BufMut, Bytes, BytesMut};
use http::{header, HeaderMap, HeaderValue, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::error::ApiError;

#[derive(Debug, Clone, Default)]
pub struct QueryMap(pub HashMap<String, Vec<String>>);

#[async_trait]
impl<S> FromRequestParts<S> for QueryMap {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();

        let query_params =
            serde_html_form::from_str::<Vec<(String, String)>>(query).map_err(|err| {
                tracing::warn!("{}", err);
                ApiError::internal("Unable to process query parameters.")
            })?;

        let mut map = HashMap::<String, Vec<String>>::new();

        for (key, value) in query_params {
            map.entry(key).or_default().push(value);
        }

        Ok(Self(map))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Json<T>(pub T);

impl<T> std::ops::Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JsonRejectionError {
    #[error("'content-type' header missing")]
    MissingContentType,
    #[error("'content-type' header invalid")]
    InvalidContentType,
    #[error("'{0}' is not a valid mime type")]
    InvalidMimeType(String),
    #[error("Expected mime subtype '{0}', but found '{1}'")]
    UnexpectedMimeType(&'static str, String),
    #[error(transparent)]
    Bytes(#[from] axum::extract::rejection::BytesRejection),
    #[error(transparent)]
    Json(#[from] axum::extract::rejection::JsonRejection),
}

impl From<JsonRejectionError> for ApiError {
    fn from(value: JsonRejectionError) -> Self {
        match value {
            JsonRejectionError::MissingContentType => ApiError::missing_header(value.to_string()),
            JsonRejectionError::InvalidContentType => ApiError::invalid_header(value.to_string()),
            JsonRejectionError::InvalidMimeType(_)
            | JsonRejectionError::UnexpectedMimeType(_, _) => {
                ApiError::bad_request(value.to_string())
            }
            JsonRejectionError::Bytes(_) => ApiError::internal("Unable to read body bytes"),
            JsonRejectionError::Json(_) => {
                // @FIXME jezza - 21 Nov 2022: We should probably try to fill out a bit more information here.
                let received = String::from("n/a");
                ApiError::invalid_type("A valid json value is expected here.", "json", received)
            }
        }
    }
}

impl IntoResponse for JsonRejectionError {
    fn into_response(self) -> Response {
        let api_error: ApiError = self.into();
        api_error.into_response()
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = JsonRejectionError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        check_json_content_type(req.headers())?;

        let bytes = Bytes::from_request(req, state).await?;
        let value = axum::extract::Json::<T>::from_bytes(&bytes)?;

        tracing::info!("JSON OK");
        Ok(Json(value.0))
    }
}

impl<T> IntoResponse for Json<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        let mut buf = BytesMut::new().writer();
        match serde_json::to_writer(&mut buf, &self.0) {
            Ok(()) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                buf.into_inner().freeze(),
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                err.to_string(),
            )
                .into_response(),
        }
    }
}

fn check_json_content_type(headers: &HeaderMap) -> Result<(), JsonRejectionError> {
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .ok_or_else(|| {
            tracing::warn!("`content-type` header not found");
            JsonRejectionError::MissingContentType
        })?
        .to_str()
        .map_err(|err| {
            tracing::warn!("non-ASCII `content-type` header: {err}");
            JsonRejectionError::InvalidContentType
        })?;

    let mime = content_type.parse::<mime::Mime>().map_err(|err| {
        tracing::warn!("cannot parse `content-type`: {err}");
        JsonRejectionError::InvalidMimeType(content_type.into())
    })?;

    if mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().map_or(false, |name| name == "json"))
    {
        Ok(())
    } else {
        tracing::warn!("MIME != JSON");
        Err(JsonRejectionError::UnexpectedMimeType(
            "application/json",
            content_type.into(),
        ))
    }
}
