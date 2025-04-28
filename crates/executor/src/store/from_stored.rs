use std::{
    collections::{HashMap, HashSet},
    sync::OnceLock,
};

use crate::{
    cexpr::{CExpr, CExprInner, CExprInnerKind, FuncCallCExpr, IfCExpr, IfCExprCase},
    exec::{ExecError, ExecScope},
    function::{CustomFunction, Function, FunctionInner, FunctionInnerKind},
    node::Node,
    store::LoadError,
};

use super::models::*;

impl ExecScope {
    pub(super) fn from_stored(stored_exec_scope: StoredExecScope) -> Result<ExecScope, ExecError> {
        let mut scope = FromStoredScope {
            stored_exec_scope,
            nodes: HashMap::new(),
            cexprs: HashMap::new(),
            funcs: HashMap::new(),
            processing: HashSet::new(),
        };

        let mut funcs = HashMap::new();
        let mut nodes = HashMap::new();

        for (func_sign, func_id) in scope.stored_exec_scope.sign_to_func.clone() {
            assert!(funcs
                .insert(func_sign, Function::from_stored(func_id, &mut scope)?)
                .is_none());
        }

        for (node_name, node_id) in scope.stored_exec_scope.name_to_node.clone() {
            assert!(nodes
                .insert(node_name, Node::from_stored(node_id, &mut scope)?)
                .is_none());
        }

        Ok(ExecScope { funcs, nodes })
    }
}

struct FromStoredScope {
    // Immutable
    stored_exec_scope: StoredExecScope,

    nodes: HashMap<StoredNodeId, Node>,
    cexprs: HashMap<StoredCExprId, CExpr>,
    funcs: HashMap<StoredFunctionId, Function>,

    /// Only for checking for circular dependencies
    processing: HashSet<Id>,
}

impl Node {
    fn from_stored(id: StoredNodeId, scope: &mut FromStoredScope) -> Result<Self, ExecError> {
        if let Some(node) = scope.nodes.get(&id) {
            return Ok(node.clone());
        }
        if !scope.processing.insert(id) {
            return Err(LoadError::CorruptedData {
                msg: format!(
                    "circular dependency detected, processing: {:#?}",
                    scope.processing
                ),
            }
            .into());
        }

        let Some(stored_node) = scope.stored_exec_scope.nodes.remove(&id) else {
            return Err(LoadError::CorruptedData {
                msg: format!("node with id = {id} is undefined"),
            }
            .into());
        };

        let node = match stored_node {
            StoredNode::Value(value) => Node::from_value(value.clone()),
            StoredNode::CExpr { body, bindings } => Node::from_cexpr(
                CExpr::from_stored(body, scope)?,
                bindings
                    .into_iter()
                    .map(|(name, id)| -> Result<_, ExecError> {
                        Ok((name, Node::from_stored(id, scope)?))
                    })
                    .collect::<Result<_, _>>()?,
            )?,
        };

        assert!(scope.nodes.insert(id, node.clone()).is_none());
        assert!(scope.processing.remove(&id));

        Ok(node)
    }
}

impl CExpr {
    fn from_stored(id: StoredCExprId, scope: &mut FromStoredScope) -> Result<Self, ExecError> {
        if let Some(cexpr) = scope.cexprs.get(&id) {
            return Ok(cexpr.clone());
        }
        if !scope.processing.insert(id) {
            return Err(LoadError::CorruptedData {
                msg: format!(
                    "circular dependency detected, processing: {:#?}",
                    scope.processing
                ),
            }
            .into());
        }

        let Some(stored_cexpr) = scope.stored_exec_scope.cexprs.remove(&id) else {
            return Err(LoadError::CorruptedData {
                msg: format!("cexpr with id = {id} is undefined"),
            }
            .into());
        };

        let StoredCExpr {
            required_vars,
            value_type,
            kind,
        } = stored_cexpr;

        let new_kind = match kind {
            StoredCExprKind::Value(value) => CExprInnerKind::Value(value),
            StoredCExprKind::Variable(ident) => CExprInnerKind::Variable(ident.clone()),
            StoredCExprKind::FuncCall { func, args } => CExprInnerKind::FuncCall(FuncCallCExpr {
                func: Function::from_stored(func, scope)?,
                args: args
                    .into_iter()
                    .map(|id| CExpr::from_stored(id, scope))
                    .collect::<Result<_, _>>()?,
            }),
            StoredCExprKind::If {
                cases,
                default_case_value,
            } => CExprInnerKind::If(IfCExpr {
                cases: cases
                    .into_iter()
                    .map(|(cond_id, value_id)| -> Result<_, ExecError> {
                        Ok(IfCExprCase {
                            cond: CExpr::from_stored(cond_id, scope)?,
                            value: CExpr::from_stored(value_id, scope)?,
                        })
                    })
                    .collect::<Result<_, _>>()?,
                default_case_value: default_case_value
                    .map_or(Ok(None), |id| CExpr::from_stored(id, scope).map(Some))?,
            }),
        };

        let cexpr = CExpr::from(CExprInner {
            required_vars,
            value_type,
            kind: new_kind,
        });

        assert!(scope.cexprs.insert(id, cexpr.clone()).is_none());
        assert!(scope.processing.remove(&id));

        Ok(cexpr)
    }
}

impl Function {
    fn from_stored(id: StoredFunctionId, scope: &mut FromStoredScope) -> Result<Self, ExecError> {
        if let Some(func) = scope.funcs.get(&id) {
            return Ok(func.clone());
        }

        let Some(stored_func) = scope.stored_exec_scope.funcs.remove(&id) else {
            return Err(LoadError::CorruptedData {
                msg: format!("func with id = {id} is undefined"),
            }
            .into());
        };

        let StoredFunction {
            sign,
            return_type,
            mut kind,
        } = stored_func;

        let func = match kind
            .0
            .take()
            .expect("only initialized OnceLocks are deserialized")
        {
            StoredFunctionKind::Builtin(sign) => match Function::get_builtin(&sign) {
                Some(func) => {
                    assert!(scope.funcs.insert(id, func.clone()).is_none());
                    func
                }
                None => {
                    return Err(LoadError::CorruptedData {
                        msg: format!("undefined builtin function: {sign}"),
                    }
                    .into())
                }
            },
            StoredFunctionKind::CExpr { arg_names, body } => {
                let mut func = Function::from(FunctionInner {
                    sign,
                    return_type,
                    kind: OnceLock::new(),
                });

                assert!(scope.funcs.insert(id, func.clone()).is_none());

                let kind = FunctionInnerKind::CustomFunction(CustomFunction {
                    arg_names,
                    body: CExpr::from_stored(body, scope)?,
                });

                func.set_kind(kind);

                func
            }
        };

        Ok(func)
    }
}
