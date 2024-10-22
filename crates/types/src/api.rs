use std::collections::HashMap;

use crate::{
    core::Value,
    lang::{FunctionSignature, Ident, Statement},
};

#[derive(Debug)]
pub struct ClientMessage {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct ServerMessage {
    pub warnings: Vec<Warning>,

    pub errors: Vec<Error>,

    pub values: HashMap<Ident, ServerMessageValue>,

    pub new_functions: Vec<FunctionSignature>,
}

#[derive(Debug)]
pub struct ServerMessageValue {
    pub names: Vec<Ident>,

    // Real (untransformed) value of object. Is used to print info about object.
    pub original: Value,

    // Transformed value (with transformation applied). Is used to display in
    // GUI.
    pub transformed: Value,
}

#[derive(Debug)]
pub struct Warning(pub String);

#[derive(Debug)]
pub struct Error(pub String);
