use std::{collections::HashSet, sync::Arc};

use types::{
    core::{Value, ValueType},
    lang::Ident,
};

use crate::function::Function;

/// Compiled Expr
#[derive(Clone, Debug)]
pub struct CExpr(pub Arc<CExprInner>);

#[derive(Clone, Debug)]
pub struct CExprInner {
    /// Set of all variables used inside of this CExpr
    pub required_vars: HashSet<Ident>,

    /// CExpr has some [return] type, that may NOT change
    pub value_type: ValueType,

    pub kind: CExprInnerKind,
}

#[derive(Clone, Debug)]
pub enum CExprInnerKind {
    Value(Value),
    Variable(Ident),
    FuncCall(FuncCallCExpr),
    If(IfCExpr),
}

#[derive(Clone, Debug)]
pub struct FuncCallCExpr {
    pub func: Function,
    pub args: Vec<CExpr>,
}

#[derive(Clone, Debug)]
pub struct IfCExpr {
    pub cases: Vec<IfCExprCase>,
    pub default_case_value: Option<CExpr>,
}

#[derive(Clone, Debug)]
pub struct IfCExprCase {
    pub cond: CExpr,
    pub value: CExpr,
}
