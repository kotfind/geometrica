use crate::{core::Value, core::Ident};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Write};

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub msg: String,
}

impl<E: std::error::Error> From<E> for Error {
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

pub mod eval {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub exprs: Vec<RequestExpr>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RequestExpr {
        pub expr: String,

        #[serde(default)]
        pub vars: HashMap<Ident, Value>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        pub values: Vec<Result<Value, Error>>,
    }
}

pub mod exec {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub script: String,
        // TODO: bindings
    }

    #[derive(Serialize, Deserialize)]
    pub struct Response;
}

pub mod items {
    use super::*;

    pub mod get_all {
        use super::*;

        #[derive(Serialize, Deserialize)]
        pub struct Request;

        #[derive(Serialize, Deserialize)]
        pub struct Response {
            pub items: HashMap<Ident, Value>,
        }
    }
}
