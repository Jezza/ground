use axum::response::IntoResponse;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::ops::Not as _;

pub type ApiResult<T = axum::response::Response, E = ApiError> = Result<T, E>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiError {
    pub location: Option<String>,
    pub message: String,
    pub reason: ApiErrorReason,
}

impl ApiError {
    pub fn with_location(self, location: String) -> Self {
        Self {
            location: location.is_empty().not().then_some(location),
            ..self
        }
    }

    pub fn missing_field(message: impl std::fmt::Display, ty: &'static str, field: String) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::MissingField {
                obj_type: ty.into(),
                field,
            },
        }
    }

    pub fn unknown_field(message: impl std::fmt::Display, ty: String, field: String) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::UnknownField {
                field_type: ty,
                field,
            },
        }
    }

    pub fn unknown_variant(
        message: impl std::fmt::Display,
        ty: &'static str,
        variant: String,
    ) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::UnknownVariant {
                variant_type: ty.into(),
                variant,
            },
        }
    }

    pub fn invalid_type(
        message: impl std::fmt::Display,
        expected: &'static str,
        got: String,
    ) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::InvalidType {
                expected: expected.into(),
                value: got,
            },
        }
    }

    pub fn invalid_value(message: impl std::fmt::Display, ty: &'static str, value: String) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::InvalidValue {
                value_type: ty.into(),
                value,
            },
        }
    }

    pub fn missing_query_argument(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::MissingQueryArgument,
        }
    }

    pub fn invalid_query_argument(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::InvalidQueryArgument,
        }
    }

    pub fn missing_header(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::MissingHeader,
        }
    }

    pub fn invalid_header(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::InvalidHeader,
        }
    }

    pub fn missing_path_argument(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::MissingPathArgument,
        }
    }

    pub fn invalid_path_argument(message: impl std::fmt::Display, key: Option<String>) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::InvalidPathArgument { key },
        }
    }

    pub fn not_found(
        message: impl std::fmt::Display,
        entity: String,
        value: Option<String>,
    ) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::NotFound { entity, value },
        }
    }

    pub fn internal(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::Internal,
        }
    }

    pub fn unauthorized(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::Unauthorized,
        }
    }

    pub fn failed_precondition(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::FailedPrecondition,
        }
    }

    pub fn bad_gateway(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::BadGateway,
        }
    }

    pub fn bad_request(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::BadRequest,
        }
    }

    pub fn service_unavailable(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::ServiceUnavailable,
        }
    }

    pub fn unprocessable_entity(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::UnprocessableEntity,
        }
    }

    pub fn forbidden(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::Forbidden,
        }
    }

    pub fn conflict(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::Conflict,
        }
    }

    pub fn not_implemented(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::NotImplemented,
        }
    }

    pub fn misdirected_request(message: impl std::fmt::Display) -> Self {
        Self {
            location: None,
            message: message.to_string(),
            reason: ApiErrorReason::MisdirectedRequest,
        }
    }
}

/// Reason for the API error
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiErrorReason {
    MissingField {
        obj_type: String,
        field: String,
    },
    UnknownField {
        field_type: String,
        field: String,
    },
    UnknownVariant {
        variant_type: String,
        variant: String,
    },
    InvalidType {
        expected: String,
        value: String,
    },
    InvalidValue {
        value_type: String,
        value: String,
    },
    MissingQueryArgument,
    InvalidQueryArgument,
    MissingHeader,
    InvalidHeader,
    MissingPathArgument,
    InvalidPathArgument {
        key: Option<String>,
    },
    NotFound {
        entity: String,
        value: Option<String>,
    },
    Internal,
    Unauthorized,
    FailedPrecondition,
    BadGateway,
    ServiceUnavailable,
    UnprocessableEntity,
    Forbidden,
    Conflict,
    NotImplemented,
    BadRequest,
    MisdirectedRequest,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match &self.reason {
            ApiErrorReason::MissingField { .. }
            | ApiErrorReason::UnknownField { .. }
            | ApiErrorReason::UnknownVariant { .. }
            | ApiErrorReason::InvalidType { .. }
            | ApiErrorReason::InvalidValue { .. }
            | ApiErrorReason::MissingQueryArgument
            | ApiErrorReason::InvalidQueryArgument
            | ApiErrorReason::MissingHeader
            | ApiErrorReason::InvalidHeader
            | ApiErrorReason::BadRequest
            | ApiErrorReason::MissingPathArgument
            | ApiErrorReason::InvalidPathArgument { .. } => {
                (StatusCode::BAD_REQUEST, axum::Json(self)).into_response()
            }
            ApiErrorReason::NotFound {
                entity: _,
                value: _,
            } => (StatusCode::NOT_FOUND, axum::Json(self)).into_response(),
            ApiErrorReason::Unauthorized => {
                (StatusCode::UNAUTHORIZED, axum::Json(self)).into_response()
            }
            ApiErrorReason::UnprocessableEntity => {
                (StatusCode::UNPROCESSABLE_ENTITY, axum::Json(self)).into_response()
            }
            ApiErrorReason::Forbidden => (StatusCode::FORBIDDEN, axum::Json(self)).into_response(),
            ApiErrorReason::FailedPrecondition => {
                tracing::warn!("{}", self.message);
                (StatusCode::PRECONDITION_FAILED, axum::Json(self)).into_response()
            }
            ApiErrorReason::Internal => {
                tracing::warn!("{}", self.message);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ApiErrorReason::BadGateway => {
                tracing::warn!("{}", self.message);
                StatusCode::BAD_GATEWAY.into_response()
            }
            ApiErrorReason::MisdirectedRequest => {
                tracing::warn!("{}", self.message);
                (StatusCode::MISDIRECTED_REQUEST, axum::Json(self.message)).into_response()
            }
            ApiErrorReason::ServiceUnavailable => {
                tracing::warn!("{}", self.message);
                StatusCode::SERVICE_UNAVAILABLE.into_response()
            }
            ApiErrorReason::Conflict => (StatusCode::CONFLICT, axum::Json(self)).into_response(),
            ApiErrorReason::NotImplemented => {
                tracing::warn!("{}", self.message);
                StatusCode::NOT_IMPLEMENTED.into_response()
            }
        }
    }
}
