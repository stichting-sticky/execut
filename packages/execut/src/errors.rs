use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum Error {
    DuplicateBadge,
    DuplicateToken,
    DuplicateScan,
    Internal,
    InvalidRequest,
    InvalidToken,
    MissingCredentials,
    SelfScan,
    Unauthorized,
    UnknownBadge,
    UnknownUser,
    WrongCredentials,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::DuplicateBadge => (StatusCode::UNPROCESSABLE_ENTITY, "duplicate badge"),
            Self::DuplicateToken => (StatusCode::UNPROCESSABLE_ENTITY, "duplicate token"),
            Self::DuplicateScan => (StatusCode::UNPROCESSABLE_ENTITY, "duplicate scan"),
            Self::InvalidRequest => (StatusCode::UNPROCESSABLE_ENTITY, "invalid request"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid token"),
            Self::MissingCredentials => (StatusCode::UNPROCESSABLE_ENTITY, "no credentials found"),
            Self::SelfScan => (StatusCode::UNPROCESSABLE_ENTITY, "self scan"),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "not authorized"),
            Self::UnknownBadge => (StatusCode::NOT_FOUND, "unknown badge"),
            Self::UnknownUser => (StatusCode::NOT_FOUND, "unknown user"),
            Self::WrongCredentials => (StatusCode::UNPROCESSABLE_ENTITY, "wrong credentials"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
        };

        let body = Json(json!({
            "detail": message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
