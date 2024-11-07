use crate::{core::Ident, core::Value};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Error {
    pub msg: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}

pub trait Request: Serialize + DeserializeOwned {
    const ROUTE: &str;
    type Response: Response;
}

pub trait Response: Serialize + DeserializeOwned {
    const ROUTE: &str;
    type Request: Request;
}

macro_rules! query {
    ($route:literal, $req:ident, $resp:ident) => {
        impl crate::api::Request for $req {
            type Response = $resp;
            const ROUTE: &str = $route;
        }

        impl crate::api::Response for $resp {
            type Request = $req;
            const ROUTE: &str = $route;
        }
    };
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

    query!("/eval", Request, Response);
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

    query!("/exec", Request, Response);
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

        query!("/items/get_all", Request, Response);
    }
}
