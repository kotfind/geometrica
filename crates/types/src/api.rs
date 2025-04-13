use crate::{
    core::{Ident, Value},
    lang::{Definition, Expr},
};
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

macro_rules! route {
    (
        ROUTE $route:literal
        REQUEST {
            $($req_field_name:ident: $req_field_type:ty),*
            $(,)?
        }
        RESPONSE {
            $($resp_field_name:ident: $resp_field_type:ty),*
            $(,)?
        }
    ) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
        pub struct Request {
            $(pub $req_field_name: $req_field_type),*
        }

        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
        pub struct Response {
            $(pub $resp_field_name: $resp_field_type),*
        }

        impl crate::api::Request for Request {
            type Response = Response;
            const ROUTE: &str = $route;
        }

        impl crate::api::Response for Response {
            type Request = Request;
            const ROUTE: &str = $route;
        }
    };
}

pub mod eval {
    use super::*;

    route! {
        ROUTE "/eval"
        REQUEST {
            exprs: Vec<Expr>,
        }
        RESPONSE {
            values: Vec<Result<Value, Error>>,
        }
    }
}

pub mod exec {
    use super::*;

    route! {
        ROUTE "/exec"
        REQUEST {
            defs: Vec<Definition>,
        }
        RESPONSE {}
    }
}

pub mod items {
    use super::*;

    pub mod get_all {
        use super::*;

        route! {
            ROUTE "/items/get_all"
            REQUEST {}
            RESPONSE {
                items: HashMap<Ident, Value>,
            }
        }
    }

    pub mod get {
        use super::*;

        route! {
            ROUTE "/items/get"
            REQUEST {
                name: Ident
            }
            RESPONSE {
                value: Value
            }
        }
    }
}

pub mod set {
    use super::*;

    route! {
        ROUTE "/set"
        REQUEST {
            name: Ident,
            expr: Expr,
        }
        RESPONSE {}
    }
}

pub mod rm {
    use super::*;

    route! {
        ROUTE "/rm"
        REQUEST {
            name: Ident,
        }
        RESPONSE {}
    }
}
