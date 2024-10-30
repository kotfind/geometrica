use std::{
    collections::{hash_map, HashMap, HashSet},
    sync::Arc,
};

use thiserror::Error;
use types::{
    core::{Value, ValueType},
    lang::{
        Expr, ExprInner, FuncCallExpr, FunctionSignature, Ident, IfExpr, LetExpr, LetExprDefinition,
    },
};

use crate::{
    cexpr::{CExpr, CExprInner, CExprInnerKind, FuncCallCExpr, IfCExpr, IfCExprCase},
    function::{FuncMap, Function},
};

/// Scope for compiling Expr into CExpr
pub struct CScope<'a> {
    funcs: FuncMap,
    var_types: HashMap<Ident, ValueType>,
    bindings: HashMap<Ident, CExpr>,
    parent: Option<&'a CScope<'a>>,
}

impl<'a> CScope<'a> {
    pub fn new() -> Self {
        Self {
            funcs: FuncMap::new(),
            var_types: HashMap::new(),
            bindings: HashMap::new(),
            parent: None,
        }
    }

    fn push(&'a self) -> Self {
        Self {
            funcs: FuncMap::new(),
            var_types: HashMap::new(),
            bindings: HashMap::new(),
            parent: Some(self),
        }
    }

    fn bind(&mut self, name: Ident, value: CExpr) -> CResult<()> {
        match self.bindings.entry(name.clone()) {
            hash_map::Entry::Occupied(_) => return Err(CError::VariableRedefinition(name)),
            hash_map::Entry::Vacant(entry) => {
                entry.insert(value);
            }
        }
        Ok(())
    }

    fn get_binding(&self, name: &Ident) -> Option<CExpr> {
        let mut scope_ = Some(self);
        while let Some(scope) = scope_ {
            let ans = scope.bindings.get(name);
            if ans.is_some() {
                return ans.cloned();
            }
            scope_ = scope.parent;
        }
        None
    }

    fn get_var_type(&self, name: &Ident) -> Option<ValueType> {
        let mut scope_ = Some(self);
        while let Some(scope) = scope_ {
            let ans = scope.var_types.get(name);
            if ans.is_some() {
                return ans.cloned();
            }
            scope_ = scope.parent;
        }
        None
    }

    fn get_func(&self, sign: &FunctionSignature) -> Option<Function> {
        let ans = Function::get_builtin(sign);
        if ans.is_some() {
            return ans;
        }

        let mut scope_ = Some(self);
        while let Some(scope) = scope_ {
            let ans = scope.funcs.get(sign);
            if ans.is_some() {
                return ans.cloned();
            }
            scope_ = scope.parent;
        }
        None
    }
}

/// Compile error
#[derive(Debug, Error)]
pub enum CError {
    #[error("variable undefined: {0}")]
    UndefinedVariable(Ident),

    #[error("function undefined: {0:?}")]
    UndefinedFunction(FunctionSignature),

    #[error("different branches of if has different types: {0} and {1}")]
    IfDifferentTypes(ValueType, ValueType),

    #[error("unexpected type in let definition: expected {expected}, got {got}")]
    UnexpectedLetDefinitionType { expected: ValueType, got: ValueType },

    #[error("variable redefinition: {0}")]
    VariableRedefinition(Ident),

    #[error("if condition should be bool")]
    IfConditionNotBool,
}

/// Compile Result
type CResult<T> = Result<T, CError>;

impl CExpr {
    fn from_inner(inner: CExprInner) -> Self {
        Self(Arc::new(inner))
    }

    pub fn from(expr: Expr, cscope: &CScope) -> CResult<Self> {
        match &expr.0 as &ExprInner {
            ExprInner::Value(value) => Ok(Self::from_value(value.clone())),
            ExprInner::Variable(var) => Self::from_var(var.clone(), cscope),
            ExprInner::FuncCall(func_call) => Self::from_func_call(func_call.clone(), cscope),
            ExprInner::If(if_expr) => Self::from_if(if_expr.clone(), cscope),
            ExprInner::Let(let_expr) => Self::from_let(let_expr.clone(), cscope),
        }
    }

    fn from_value(value: Value) -> Self {
        Self::from_inner(CExprInner {
            required_vars: HashSet::new(),
            value_type: value.value_type(),
            kind: CExprInnerKind::Value(value.clone()),
        })
    }

    fn from_var(var: Ident, cscope: &CScope) -> CResult<Self> {
        if let Some(cexpr) = cscope.get_binding(&var) {
            return Ok(cexpr);
        }

        let Some(value_type) = cscope.get_var_type(&var) else {
            return Err(CError::UndefinedVariable(var.clone()));
        };

        Ok(Self::from_inner(CExprInner {
            required_vars: HashSet::from([var.clone()]),
            value_type: value_type.clone(),
            kind: CExprInnerKind::Variable(var),
        }))
    }

    fn from_func_call(FuncCallExpr { name, args }: FuncCallExpr, cscope: &CScope) -> CResult<Self> {
        let args = args
            .clone()
            .into_iter()
            .map(|arg| CExpr::from(arg, cscope))
            .collect::<CResult<Vec<_>>>()?;

        let sign = FunctionSignature {
            name: name.clone(),
            arg_types: args.iter().map(|arg| arg.0.value_type.clone()).collect(),
        };

        let Some(func) = cscope.get_func(&sign) else {
            return Err(CError::UndefinedFunction(sign));
        };

        Ok(Self::from_inner(CExprInner {
            required_vars: args
                .iter()
                .map(|arg| arg.0.required_vars.clone().into_iter())
                .flatten()
                .collect(),
            value_type: func.0.return_type.clone(),
            kind: CExprInnerKind::FuncCall(FuncCallCExpr { func, args }),
        }))
    }

    fn from_if(
        IfExpr {
            cases,
            default_case_value,
        }: IfExpr,
        cscope: &CScope,
    ) -> CResult<Self> {
        let cases = cases
            .into_iter()
            .map(|case| {
                Ok(IfCExprCase {
                    cond: CExpr::from(case.condition, &cscope)?,
                    value: CExpr::from(case.value, &cscope)?,
                })
            })
            .collect::<CResult<Vec<_>>>()?;

        assert!(!cases.is_empty());
        let value_type = cases[0].value.0.value_type.clone();
        let mut vars = HashSet::new();
        for case in &cases {
            if case.value.0.value_type != value_type {
                return Err(CError::IfDifferentTypes(
                    value_type.clone(),
                    case.value.0.value_type.clone(),
                ));
            }
            if case.cond.0.value_type != ValueType::Bool {
                return Err(CError::IfConditionNotBool);
            }
            vars.extend(case.cond.0.required_vars.clone());
            vars.extend(case.value.0.required_vars.clone());
        }

        let default_case_value = if let Some(default_value) = default_case_value.clone() {
            let ans = CExpr::from(default_value, &cscope)?;
            if ans.0.value_type != value_type {
                return Err(CError::IfDifferentTypes(
                    value_type.clone(),
                    ans.0.value_type.clone(),
                ));
            }
            Some(ans)
        } else {
            None
        };

        Ok(Self::from_inner(CExprInner {
            required_vars: vars,
            value_type,
            kind: CExprInnerKind::If(IfCExpr {
                cases,
                default_case_value,
            }),
        }))
    }

    fn from_let(LetExpr { definitions, body }: LetExpr, cscope: &CScope) -> CResult<Self> {
        let mut new_cscope = cscope.push();
        for LetExprDefinition {
            name,
            value_type,
            body,
        } in definitions
        {
            let body = CExpr::from(body, &new_cscope)?;
            if let Some(value_type) = value_type {
                if body.0.value_type != value_type {
                    return Err(CError::UnexpectedLetDefinitionType {
                        expected: value_type,
                        got: body.0.value_type.clone(),
                    });
                }
            }
            new_cscope.bind(name, body)?;
        }
        CExpr::from(body, &new_cscope)
    }
}
