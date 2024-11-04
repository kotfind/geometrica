use std::{
    collections::{hash_map, HashMap, HashSet},
    sync::Arc,
};

use thiserror::Error;
use types::{
    core::{Value, ValueType},
    lang::{Expr, FuncCallExpr, FunctionSignature, Ident, IfExpr, LetExpr, LetExprDefinition},
};

use crate::{
    cexpr::{CExpr, CExprInner, CExprInnerKind, FuncCallCExpr, IfCExpr, IfCExprCase},
    exec::ExecScope,
    function::Function,
};

/// Scope for compiling Expr into CExpr
pub(crate) struct CScope<'a, 'b> {
    exec_scope: &'a ExecScope,
    bindings: HashMap<Ident, CExpr>,
    var_types: HashMap<Ident, ValueType>,
    parent: Option<&'b CScope<'a, 'b>>,
}

impl<'a, 'b> CScope<'a, 'b> {
    pub(crate) fn new(exec_scope: &'a ExecScope) -> Self {
        Self {
            exec_scope,
            bindings: HashMap::new(),
            var_types: HashMap::new(),
            parent: None,
        }
    }

    fn push(&'a self) -> Self {
        Self {
            exec_scope: self.exec_scope,
            bindings: HashMap::new(),
            var_types: HashMap::new(),
            parent: Some(self),
        }
    }

    fn insert_binding(&mut self, name: Ident, value: CExpr) -> Result<(), CError> {
        match self.bindings.entry(name.clone()) {
            hash_map::Entry::Occupied(_) => return Err(CError::VariableRedefinition(name)),
            hash_map::Entry::Vacant(entry) => {
                entry.insert(value);
            }
        }
        Ok(())
    }

    pub(crate) fn insert_var_type(
        &mut self,
        name: Ident,
        var_type: ValueType,
    ) -> Result<(), CError> {
        match self.var_types.entry(name.clone()) {
            hash_map::Entry::Occupied(_) => return Err(CError::VariableRedefinition(name)),
            hash_map::Entry::Vacant(entry) => {
                entry.insert(var_type);
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
        self.exec_scope.get_node(name).map(|node| node.value_type())
    }

    fn get_func(&self, sign: &FunctionSignature) -> Option<Function> {
        self.exec_scope.get_func(sign)
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
pub type CResult = Result<CExpr, CError>;

pub(crate) trait Compile {
    fn compile(self, cscope: &CScope) -> CResult;
}

impl CExpr {
    fn from_inner(inner: CExprInner) -> Self {
        Self(Arc::new(inner))
    }
}

impl Compile for Expr {
    fn compile(self, cscope: &CScope) -> CResult {
        match self {
            Expr::Value(value) => value.compile(cscope),
            Expr::Variable(var) => var.compile(cscope),
            Expr::FuncCall(func_call) => func_call.compile(cscope),
            Expr::If(if_expr) => if_expr.compile(cscope),
            Expr::Let(let_expr) => let_expr.compile(cscope),
        }
    }
}

impl Compile for Value {
    fn compile(self, _cscope: &CScope) -> CResult {
        Ok(CExpr::from_inner(CExprInner {
            required_vars: HashSet::new(),
            value_type: self.value_type(),
            kind: CExprInnerKind::Value(self),
        }))
    }
}

impl Compile for Ident {
    fn compile(self, cscope: &CScope) -> CResult {
        if let Some(cexpr) = cscope.get_binding(&self) {
            return Ok(cexpr);
        }

        let Some(value_type) = cscope.get_var_type(&self) else {
            return Err(CError::UndefinedVariable(self.clone()));
        };

        Ok(CExpr::from_inner(CExprInner {
            required_vars: HashSet::from([self.clone()]),
            value_type: value_type.clone(),
            kind: CExprInnerKind::Variable(self),
        }))
    }
}

impl Compile for FuncCallExpr {
    fn compile(self, cscope: &CScope) -> CResult {
        let FuncCallExpr { name, args } = self;
        let args = args
            .clone()
            .into_iter()
            .map(|arg| arg.compile(cscope))
            .collect::<Result<Vec<_>, _>>()?;

        let sign = FunctionSignature {
            name: name.clone(),
            arg_types: args.iter().map(|arg| arg.0.value_type.clone()).collect(),
        };

        let Some(func) = cscope.get_func(&sign) else {
            return Err(CError::UndefinedFunction(sign));
        };

        Ok(CExpr::from_inner(CExprInner {
            required_vars: args
                .iter()
                .flat_map(|arg| arg.0.required_vars.clone().into_iter())
                .collect(),
            value_type: func.return_type(),
            kind: CExprInnerKind::FuncCall(FuncCallCExpr { func, args }),
        }))
    }
}

impl Compile for IfExpr {
    fn compile(self, cscope: &CScope) -> CResult {
        let IfExpr {
            cases,
            default_value: default_case_value,
        } = self;

        let cases = cases
            .into_iter()
            .map(|case| {
                Ok(IfCExprCase {
                    cond: case.cond.compile(cscope)?,
                    value: case.value.compile(cscope)?,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

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
            let ans = default_value.compile(cscope)?;
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

        Ok(CExpr::from_inner(CExprInner {
            required_vars: vars,
            value_type,
            kind: CExprInnerKind::If(IfCExpr {
                cases,
                default_case_value,
            }),
        }))
    }
}

impl Compile for LetExpr {
    fn compile(self, cscope: &CScope) -> CResult {
        let LetExpr {
            defs: definitions,
            body,
        } = self;
        let mut new_cscope = cscope.push();
        for LetExprDefinition {
            name,
            value_type,
            body,
        } in definitions
        {
            let body = body.compile(&new_cscope)?;
            if let Some(value_type) = value_type {
                if body.0.value_type != value_type {
                    return Err(CError::UnexpectedLetDefinitionType {
                        expected: value_type,
                        got: body.0.value_type.clone(),
                    });
                }
            }
            new_cscope.insert_binding(name, body)?;
        }
        body.compile(&new_cscope)
    }
}
