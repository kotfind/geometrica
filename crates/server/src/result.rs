use axum::{http::StatusCode, Json};
use serde::Serialize;
use std::fmt::Write;
use types::api::{self};

pub trait IntoError: Sized {
    fn into_error(self) -> api::Error;
}

impl<T: std::error::Error> IntoError for T {
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

pub type ApiOk<T> = (StatusCode, Json<T>);
pub type ApiErr = (StatusCode, Json<api::Error>);
pub type ApiResult<T> = Result<ApiOk<T>, ApiErr>;

pub fn api_ok<T: Serialize>(resp: T) -> ApiResult<T> {
    Ok(api_ok_no_result(resp))
}

pub fn api_ok_no_result<T: Serialize>(resp: T) -> ApiOk<T> {
    (StatusCode::OK, Json(resp))
}

pub fn api_err<T>(err: impl IntoError) -> ApiResult<T> {
    Err(api_err_no_result(err))
}

pub fn api_err_no_result(err: impl IntoError) -> ApiErr {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(err.into_error()))
}
