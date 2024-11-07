use axum::{http::StatusCode, Json};
use serde::Serialize;
use std::fmt::Write;
use types::api::{self};

pub trait IntoError: std::error::Error + Sized {
    fn into_error(self) -> api::Error {
        let mut msg = String::new();
        let mut error: Option<&dyn std::error::Error> = Some(&self);
        while let Some(err) = error {
            write!(msg, "{}", err).unwrap();
            if err.source().is_some() {
                write!(msg, ": ").unwrap();
            }
            error = err.source();
        }
        api::Error { msg }
    }
}

impl<T: std::error::Error> IntoError for T {}

pub type ApiOk<T> = (StatusCode, Json<T>);
pub type ApiErr = (StatusCode, Json<api::Error>);
pub type ApiResult<T> = Result<ApiOk<T>, ApiErr>;

pub fn api_ok<T: Serialize>(resp: T) -> ApiOk<T> {
    (StatusCode::OK, Json(resp))
}

pub fn api_err<E: IntoError>(err: E) -> ApiErr {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(err.into_error()))
}
