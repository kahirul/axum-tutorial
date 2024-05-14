use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFailed,

    AuthCtxNotFound,
    AuthTokenCookieNoFound,
    AuthTokenInvalid,

    TicketDeleteIdNotFound { id: u64 },
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFailed => (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL),
            Self::AuthCtxNotFound | Self::AuthTokenCookieNoFound | Self::AuthTokenInvalid => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            Self::TicketDeleteIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<16} - {self:?}", "into_response");

        let mut response =
            (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response();

        response.extensions_mut().insert(self);

        response
    }
}
