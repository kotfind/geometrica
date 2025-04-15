//! = Implementation Details
//! * All id's should be unique. Even for different types of objects.
//!     This assumption is used, when checking for dependency cycles.
//! * Arc's addresses are used as ids, when serializing.

use serde_with::serde_as;
use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use types::{
    core::{Ident, Value, ValueType},
    lang::FunctionSignature,
};

pub(super) type Id = u64;
pub(super) type StoredCExprId = Id;
pub(super) type StoredFunctionId = Id;
pub(super) type StoredNodeId = Id;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct StoredExecScope {
    pub(super) cexprs: HashMap<StoredCExprId, StoredCExpr>,
    pub(super) nodes: HashMap<StoredNodeId, StoredNode>,
    pub(super) funcs: HashMap<StoredFunctionId, StoredFunction>,

    #[serde_as(as = "Vec<(_, _)>")]
    pub(super) name_to_node: HashMap<Ident, StoredNodeId>,

    #[serde_as(as = "Vec<(_, _)>")]
    pub(super) sign_to_func: HashMap<FunctionSignature, StoredNodeId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct StoredCExpr {
    pub(super) required_vars: HashSet<Ident>,
    pub(super) value_type: ValueType,
    pub(super) kind: StoredCExprKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) enum StoredCExprKind {
    Value(Value),
    Variable(Ident),
    FuncCall {
        func: StoredFunctionId,
        args: Vec<StoredCExprId>,
    },
    If {
        cases: Vec<(
            StoredCExprId, /* cond */
            StoredCExprId, /* value */
        )>,
        default_case_value: Option<StoredCExprId>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) enum StoredFunction {
    Builtin(FunctionSignature),
    CExpr {
        arg_names: Vec<Ident>,
        body: StoredCExprId,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) enum StoredNode {
    Value(Value),
    CExpr {
        body: StoredCExprId,
        bindings: Vec<(Ident, StoredNodeId)>,
    },
}
