use std::collections::HashMap;

use types::lang::{FunctionSignature, Ident};

use crate::{function::Function, node::Node};

struct Scope {
    items: HashMap<Ident, Node>,

    functions: HashMap<FunctionSignature, Function>,
}
