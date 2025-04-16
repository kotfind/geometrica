use std::{collections::HashSet, sync::Arc};

use types::{
    core::Ident,
    core::{Value, ValueType},
};

use crate::function::Function;

pub mod compile;
pub mod eval;

/// Compiled Expr
#[derive(Clone, Debug)]
pub(crate) struct CExpr(Arc<CExprInner>);

impl CExpr {
    pub(crate) fn address(&self) -> usize {
        Arc::as_ptr(&self.0) as usize
    }

    pub(crate) fn inner(&self) -> &CExprInner {
        &self.0
    }

    pub(crate) fn required_vars(&self) -> &HashSet<Ident> {
        &self.0.required_vars
    }

    pub(crate) fn value_type(&self) -> ValueType {
        self.0.value_type.clone()
    }
}

impl From<CExprInner> for CExpr {
    fn from(value: CExprInner) -> Self {
        CExpr(Arc::new(value))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CExprInner {
    /// Set of all variables used inside of this CExpr
    pub(crate) required_vars: HashSet<Ident>,

    /// CExpr has some [return] type, that may NOT change
    pub(crate) value_type: ValueType,

    pub(crate) kind: CExprInnerKind,
}

#[derive(Clone, Debug)]
pub(crate) enum CExprInnerKind {
    Value(Value),
    Variable(Ident),
    FuncCall(FuncCallCExpr),
    If(IfCExpr),
}

#[derive(Clone, Debug)]
pub(crate) struct FuncCallCExpr {
    pub(crate) func: Function,
    pub(crate) args: Vec<CExpr>,
}

#[derive(Clone, Debug)]
pub(crate) struct IfCExpr {
    pub(crate) cases: Vec<IfCExprCase>,
    pub(crate) default_case_value: Option<CExpr>,
}

#[derive(Clone, Debug)]
pub(crate) struct IfCExprCase {
    pub(crate) cond: CExpr,
    pub(crate) value: CExpr,
}
