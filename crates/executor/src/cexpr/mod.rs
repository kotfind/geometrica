use std::{collections::HashSet, sync::Arc};

use types::{
    core::{Value, ValueType},
    core::Ident,
};

use crate::function::Function;

pub mod compile;
pub mod eval;

/// Compiled Expr
#[derive(Clone, Debug)]
pub(crate) struct CExpr(Arc<CExprInner>);

impl CExpr {
    pub(crate) fn required_vars(&self) -> &HashSet<Ident> {
        &self.0.required_vars
    }

    pub(crate) fn value_type(&self) -> ValueType {
        self.0.value_type.clone()
    }
}

#[derive(Clone, Debug)]
struct CExprInner {
    /// Set of all variables used inside of this CExpr
    required_vars: HashSet<Ident>,

    /// CExpr has some [return] type, that may NOT change
    value_type: ValueType,

    kind: CExprInnerKind,
}

#[derive(Clone, Debug)]
enum CExprInnerKind {
    Value(Value),
    Variable(Ident),
    FuncCall(FuncCallCExpr),
    If(IfCExpr),
}

#[derive(Clone, Debug)]
struct FuncCallCExpr {
    func: Function,
    args: Vec<CExpr>,
}

#[derive(Clone, Debug)]
struct IfCExpr {
    cases: Vec<IfCExprCase>,
    default_case_value: Option<CExpr>,
}

#[derive(Clone, Debug)]
struct IfCExprCase {
    cond: CExpr,
    value: CExpr,
}
