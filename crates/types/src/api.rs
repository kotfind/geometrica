use crate::{core::Ident, core::Value};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Error {
    pub msg: String,
}

pub trait IntoError: std::error::Error + Sized {
    fn into_error(error: Self) -> Error {
        let mut msg = String::new();
        let mut error: Option<&dyn std::error::Error> = Some(&error);
        while let Some(err) = error {
            write!(msg, "{}", err).unwrap();
            if err.source().is_some() {
                write!(msg, ": ").unwrap();
            }
            error = err.source();
        }
        Error { msg }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}
impl<T: std::error::Error> IntoError for T {}

pub trait Request: Serialize + DeserializeOwned {
    type Response: Serialize + DeserializeOwned;
    const PATH: &str;
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

    impl super::Request for Request {
        type Response = Response;
        const PATH: &str = "/eval";
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

    impl super::Request for Request {
        type Response = Response;
        const PATH: &str = "/exec";
    }
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

        impl super::Request for Request {
            type Response = Response;
            const PATH: &str = "/items/get_all";
        }
    }
}
