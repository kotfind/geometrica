use crate::{core::Ident, core::Value};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Write};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Request {
        pub exprs: Vec<RequestExpr>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct RequestExpr {
        pub expr: String,

        #[serde(default)]
        pub vars: HashMap<Ident, Value>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Response {
        pub values: Vec<Result<Value, Error>>,
    }
}

pub mod exec {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Request {
        pub script: String,
        // TODO: bindings
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Response;
}

pub mod items {
    use super::*;

    pub mod get_all {
        use super::*;

        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        pub struct Request;

        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        pub struct Response {
            pub items: HashMap<Ident, Value>,
        }
    }
}
