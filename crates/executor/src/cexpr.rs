use std::{collections::HashSet, sync::Arc};

use types::{
    core::{Value, ValueType},
    lang::Ident,
};

use crate::function::Function;

/// Compiled Expr
#[derive(Clone)]
pub struct CExpr(pub Arc<CExprInner>);

#[derive(Clone)]
pub struct CExprInner {
    /// Set of all variables used inside of this CExpr
    pub vars: HashSet<Ident>,

    /// CExpr has some [return] type, that may NOT change
    pub value_type: ValueType,

    pub kind: CExprInnerKind,
}

#[derive(Clone)]
pub enum CExprInnerKind {
    Value(Value),
    Variable(Ident),
    FuncCall(FuncCallCExpr),
    If(IfCExpr),
}

#[derive(Clone)]
pub struct FuncCallCExpr {
    pub func: Function,
    pub args: Vec<CExpr>,
}

#[derive(Clone)]
pub struct IfCExpr {
    pub cases: Vec<IfCExprCase>,
    pub default_case_value: Option<CExpr>,
}

#[derive(Clone)]
pub struct IfCExprCase {
    pub cond: CExpr,
    pub value: CExpr,
}
