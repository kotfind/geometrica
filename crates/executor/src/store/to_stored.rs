use std::{
    collections::{HashMap, HashSet},
    sync::OnceLock,
};

use crate::{
    cexpr::{CExpr, CExprInner, CExprInnerKind, FuncCallCExpr, IfCExpr},
    exec::ExecScope,
    function::{CustomFunction, Function, FunctionInner, FunctionInnerKind},
    node::{CExprNode, Node, NodeInnerKind},
};

use super::models::*;

impl ExecScope {
    pub(super) fn to_stored(&self) -> StoredExecScope {
        let stored_exec_scope = StoredExecScope {
            cexprs: HashMap::new(),
            nodes: HashMap::new(),
            funcs: HashMap::new(),
            name_to_node: HashMap::new(),
            sign_to_func: HashMap::new(),
        };

        let mut scope = ToStoredScope {
            stored_exec_scope,
            processing: HashSet::new(),
        };

        for (func_sign, func_value) in &self.funcs {
            let func_id = func_value.to_stored(&mut scope);
            assert!(scope
                .stored_exec_scope
                .sign_to_func
                .insert(func_sign.clone(), func_id)
                .is_none());
        }

        for (node_name, node_value) in &self.nodes {
            let node_id = node_value.to_stored(&mut scope);
            assert!(scope
                .stored_exec_scope
                .name_to_node
                .insert(node_name.clone(), node_id)
                .is_none());
        }

        scope.stored_exec_scope
    }
}

struct ToStoredScope {
    stored_exec_scope: StoredExecScope,

    /// Only for checking for circular dependencies
    processing: HashSet<Id>,
}

impl Function {
    fn to_stored(&self, scope: &mut ToStoredScope) -> StoredFunctionId {
        let id = self.address() as StoredFunctionId;
        if scope.stored_exec_scope.funcs.contains_key(&id) {
            return id;
        }

        let FunctionInner {
            sign,
            return_type,
            kind,
        } = self.inner();

        let stored_func = StoredFunction {
            sign: sign.clone(),
            return_type: return_type.clone(),
            kind: StoredOnceLock(OnceLock::new()),
        };

        assert!(scope
            .stored_exec_scope
            .funcs
            .insert(id, stored_func)
            .is_none());

        let stored_kind = match kind.get().expect("cannot serialize dummy function") {
            FunctionInnerKind::BuiltIn(_) => StoredFunctionKind::Builtin(sign.clone()),
            FunctionInnerKind::CustomFunction(CustomFunction { arg_names, body }) => {
                StoredFunctionKind::CExpr {
                    arg_names: arg_names.clone(),
                    body: body.to_stored(scope),
                }
            }
        };

        scope
            .stored_exec_scope
            .funcs
            .get_mut(&id)
            .expect("we have just pushed this")
            .kind
            .0
            .set(stored_kind)
            .expect("cannot define function twice");

        id
    }
}

impl CExpr {
    fn to_stored(&self, scope: &mut ToStoredScope) -> StoredCExprId {
        let id = self.address() as StoredCExprId;
        if scope.stored_exec_scope.cexprs.contains_key(&id) {
            return id;
        }
        assert!(
            scope.processing.insert(id),
            "circular dependency detected, processing: {:#?}",
            scope.processing
        );

        let CExprInner {
            kind,
            required_vars,
            value_type,
        } = self.inner().clone();

        let stored_kind = match kind {
            CExprInnerKind::Value(value) => StoredCExprKind::Value(value.clone()),
            CExprInnerKind::Variable(ident) => StoredCExprKind::Variable(ident.clone()),
            CExprInnerKind::FuncCall(FuncCallCExpr { func, args }) => StoredCExprKind::FuncCall {
                func: func.to_stored(scope),
                args: args.iter().map(|arg| arg.to_stored(scope)).collect(),
            },
            CExprInnerKind::If(IfCExpr {
                cases,
                default_case_value,
            }) => StoredCExprKind::If {
                cases: cases
                    .iter()
                    .map(|case| (case.cond.to_stored(scope), case.value.to_stored(scope)))
                    .collect(),
                default_case_value: default_case_value.map(|case| case.to_stored(scope)),
            },
        };

        let stored_cexpr = StoredCExpr {
            required_vars,
            value_type,
            kind: stored_kind,
        };

        assert!(scope
            .stored_exec_scope
            .cexprs
            .insert(id, stored_cexpr)
            .is_none());
        assert!(scope.processing.remove(&id));

        id
    }
}

impl Node {
    fn to_stored(&self, scope: &mut ToStoredScope) -> StoredNodeId {
        let id = self.address() as StoredNodeId;
        if scope.stored_exec_scope.nodes.contains_key(&id) {
            return id;
        }
        assert!(
            scope.processing.insert(id),
            "circular dependency detected, processing: {:?}",
            scope.processing
        );

        let stored_node = match &self.inner().kind {
            NodeInnerKind::Value(value) => StoredNode::Value(value.lock().unwrap().clone()),
            NodeInnerKind::CExpr(CExprNode { body, bindings, .. }) => StoredNode::CExpr {
                body: body.to_stored(scope),
                bindings: bindings
                    .iter()
                    .map(|binding| (binding.0.clone(), binding.1.to_stored(scope)))
                    .collect(),
            },
        };

        assert!(scope
            .stored_exec_scope
            .nodes
            .insert(id, stored_node)
            .is_none());
        assert!(scope.processing.remove(&id));

        id
    }
}

#[cfg(test)]
mod test {
    use types::{
        core::{Ident, ValueType},
        lang::FunctionSignature,
    };

    use crate::exec::Exec;

    use super::*;

    /// Only checks, that:
    /// * Don't panic
    /// * Node names are correct
    /// * Function signatures are correct
    #[test]
    fn complex() {
        let mut scope = ExecScope::new();

        parser::definitions(
            r#"
            x = 1
            y = 2
            int_pt x:int y:int -> pt = pt (x as real) (y as real)
            z = int_pt x y
        "#,
        )
        .unwrap()
        .exec(&mut scope)
        .unwrap();

        let stored_exec_scope = scope.to_stored();

        // Check that it's serializable to JSON
        serde_json::to_string(&stored_exec_scope).unwrap();

        // Check named nodes
        assert_eq!(
            stored_exec_scope
                .name_to_node
                .into_keys()
                .collect::<HashSet<Ident>>(),
            ["x", "y", "z"].into_iter().map(Ident::from).collect()
        );

        // Check user defined functions
        assert_eq!(
            stored_exec_scope
                .sign_to_func
                .into_keys()
                .collect::<HashSet<FunctionSignature>>(),
            [FunctionSignature {
                name: Ident::from("int_pt"),
                arg_types: vec![ValueType::Int, ValueType::Int]
            }]
            .into_iter()
            .collect()
        );
    }

    /// Only checks, that cast won't panic
    #[test]
    fn recursion() {
        let mut scope = ExecScope::new();

        parser::definitions(
            r#"
            fact n:int -> int = if
                n == 0 then 1,
                else n * (fact (n - 1))
            x = fact 5
        "#,
        )
        .unwrap()
        .exec(&mut scope)
        .unwrap();

        let stored_exec_scope = scope.to_stored();
        serde_json::to_string(&stored_exec_scope).unwrap();
    }
}
