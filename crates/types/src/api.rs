use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Write};

use crate::{core::Value, lang::Ident};

// -------------------- ApiError --------------------
#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub msg: String,
}

impl<E: std::error::Error> From<E> for ApiError {
    fn from(error: E) -> Self {
        let mut msg = String::new();
        let mut error: Option<&dyn std::error::Error> = Some(&error);
        while let Some(err) = error {
            write!(msg, "{}", err).unwrap();
            if err.source().is_some() {
                write!(msg, ": ").unwrap();
            }
            error = err.source();
        }
        Self { msg }
    }
}

// -------------------- Eval --------------------

#[derive(Serialize, Deserialize)]
pub struct EvalRequest {
    pub exprs: Vec<EvalRequestExpr>,
}

#[derive(Serialize, Deserialize)]
pub struct EvalRequestExpr {
    pub expr: String,

    #[serde(default)]
    pub vars: HashMap<Ident, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct EvalResponse {
    pub values: Vec<Result<Value, ApiError>>,
}

// -------------------- Exec --------------------

#[derive(Serialize, Deserialize)]
pub struct ExecRequest {
    pub script: String,
    // TODO: bindings
}

#[derive(Serialize, Deserialize)]
pub struct ExecResponse;

// -------------------- Get All Items --------------------

#[derive(Serialize, Deserialize)]
pub struct GetAllItemsRequest;

#[derive(Serialize, Deserialize)]
pub struct GetAllItemsResponse {
    pub items: HashMap<Ident, Value>,
}
