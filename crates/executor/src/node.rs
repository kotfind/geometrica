use std::{
    collections::HashMap,
    sync::{Arc, Mutex, Weak},
};

use types::{
    core::{Value, ValueType},
    lang::{Ident, ValueDefinition},
};

use crate::{
    cexpr::{
        compile::{CScope, Compile},
        eval::{Eval, EvalError},
        CExpr,
    },
    exec::{ExecError, ExecScope},
};

#[derive(Clone, Debug)]
pub struct WeakNode(Weak<NodeInner>);

impl WeakNode {
    pub fn upgrade(&self) -> Option<Node> {
        Weak::upgrade(&self.0).map(Node)
    }
}

#[derive(Clone, Debug)]
pub struct Node(Arc<NodeInner>);

impl Node {
    fn from_value(value: Value) -> Self {
        Node::from(NodeInnerKind::Value(Mutex::new(value)))
    }

    fn from_cexpr(body: CExpr, bindings: Vec<(Ident, Node)>) -> Result<Self, EvalError> {
        let node = Node::from(NodeInnerKind::CExpr(CExprNode {
            value: Mutex::new(
                body.eval(
                    &bindings
                        .iter()
                        .map(|(name, node)| (name.clone(), node.get_value()))
                        .collect(),
                )?,
            ),
            body,
            bindings: bindings.clone(),
        }));

        for (_, binding_node) in bindings {
            binding_node
                .0
                .required_by
                .lock()
                .unwrap()
                .push(node.downgrade());
        }

        Ok(node)
    }

    pub fn downgrade(&self) -> WeakNode {
        WeakNode(Arc::downgrade(&self.0))
    }

    pub fn value_type(&self) -> ValueType {
        match &self.0.kind {
            NodeInnerKind::Value(value) => value.lock().unwrap().value_type(),
            NodeInnerKind::CExpr(cexpr_node) => cexpr_node.body.value_type(),
        }
    }

    pub fn get_value(&self) -> Value {
        match &self.0.kind {
            NodeInnerKind::Value(value) => value.lock().unwrap().clone(),
            NodeInnerKind::CExpr(cexpr) => cexpr.value.lock().unwrap().clone(),
        }
    }

    pub fn from_value_definition(
        def: ValueDefinition,
        scope: &ExecScope,
    ) -> Result<Node, ExecError> {
        let ValueDefinition {
            value_type, body, ..
        } = def;

        let body = body.compile(&CScope::new(scope))?;

        if let Some(expected_type) = value_type {
            if body.value_type() != expected_type {
                return Err(ExecError::UnexpectedType {
                    expected: expected_type,
                    got: body.value_type(),
                });
            }
        }

        let node = if body.required_vars().is_empty() {
            Node::from_value(body.eval(&HashMap::new())?)
        } else {
            let bindings: Vec<(Ident, Node)> = body
                .required_vars()
                .iter()
                .map(|var| {
                    (
                        var.clone(),
                        scope
                            .get_node(var)
                            .expect("var should be defined as body was successfully compiled"),
                    )
                })
                .collect();

            Node::from_cexpr(body, bindings.clone())?
        };

        Ok(node)
    }

    pub fn set(&self, value: Value) -> Result<(), EvalError> {
        assert!(self.value_type() == value.value_type());

        let NodeInnerKind::Value(val) = &self.0.kind else {
            panic!("set method should only be called on Value-Nodes");
        };

        *val.lock().unwrap() = value;
        self.update()
    }

    /// Returns
    ///     - Err(_) on error
    ///     - Ok(true) if value changed
    ///     - Ok(false) if value is same
    fn update_self(&self) -> Result<bool, EvalError> {
        if let NodeInnerKind::CExpr(CExprNode {
            value,
            body,
            bindings,
        }) = &self.0.kind
        {
            let mut value = value.lock().unwrap();
            let old_value = value.clone();
            *value = body.eval(
                &bindings
                    .iter()
                    .map(|(name, node)| (name.clone(), node.get_value()))
                    .collect(),
            )?;
            Ok(&value as &Value != &old_value)
        } else {
            Ok(true)
        }
    }

    fn update(&self) -> Result<(), EvalError> {
        if !self.update_self()? {
            return Ok(());
        }

        let required_by = &mut self.0.required_by.lock().unwrap();
        required_by.retain(|node| node.upgrade().is_some());

        for node in required_by.iter() {
            if let Some(node) = node.upgrade() {
                node.update()?;
            }
        }

        Ok(())
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl From<NodeInnerKind> for Node {
    fn from(kind: NodeInnerKind) -> Self {
        Node(Arc::new(NodeInner {
            required_by: Mutex::new(Vec::new()),
            kind,
        }))
    }
}

#[derive(Debug)]
struct NodeInner {
    required_by: Mutex<Vec<WeakNode>>,
    kind: NodeInnerKind,
}

#[derive(Debug)]
enum NodeInnerKind {
    Value(Mutex<Value>),
    CExpr(CExprNode),
}

#[derive(Debug)]
struct CExprNode {
    /// Last evaluated value
    value: Mutex<Value>,
    body: CExpr,
    bindings: Vec<(Ident, Node)>,
}

#[cfg(test)]
mod test {
    use crate::exec::{Exec, ExecScope};

    use super::*;

    #[test]
    fn simple() {
        let mut scope = ExecScope::new();
        parser::statement("x = 1")
            .unwrap()
            .exec(&mut scope)
            .unwrap();
        assert_eq!(
            scope.get_node(&Ident::from("x")).unwrap().get_value(),
            1.into()
        );
    }

    #[test]
    fn multiple_statements() {
        let mut scope = ExecScope::new();
        parser::script(
            r#"
            x = 2;
            y = 3;
            z = x^2 + y^2;
            "#,
        )
        .unwrap()
        .exec(&mut scope)
        .unwrap();
        assert_eq!(
            scope.get_node(&Ident::from("z")).unwrap().get_value(),
            13.into()
        );
    }

    #[test]
    fn with_function() {
        let mut scope = ExecScope::new();
        parser::script(
            r#"
            sqsum a:int b:int -> int = a^2 + b^2;
            x = 2;
            y = 3;
            z = sqsum x y;
            "#,
        )
        .unwrap()
        .exec(&mut scope)
        .unwrap();
        assert_eq!(
            scope.get_node(&Ident::from("z")).unwrap().get_value(),
            13.into()
        );
    }

    #[test]
    fn with_set() {
        let mut scope = ExecScope::new();
        parser::script(
            r#"
            sqsum a:int b:int -> int = a^2 + b^2;
            x = 2;
            y = 3;
            z = sqsum x y;
            "#,
        )
        .unwrap()
        .exec(&mut scope)
        .unwrap();

        assert_eq!(
            scope.get_node(&Ident::from("z")).unwrap().get_value(),
            13.into()
        );

        scope
            .get_node(&Ident::from("x"))
            .unwrap()
            .set(10.into())
            .unwrap();

        assert_eq!(
            scope.get_node(&Ident::from("z")).unwrap().get_value(),
            109.into()
        );
    }

    #[test]
    fn with_set_chain() {
        let mut scope = ExecScope::new();
        parser::script(
            r#"
            x1 = 1;
            x2 = 2 * x1;
            x3 = 2 * x2;
            x4 = 2 * x3;
            x5 = 2 * x4;
            "#,
        )
        .unwrap()
        .exec(&mut scope)
        .unwrap();

        assert_eq!(
            scope.get_node(&Ident::from("x5")).unwrap().get_value(),
            16.into()
        );

        scope
            .get_node(&Ident::from("x1"))
            .unwrap()
            .set(10.into())
            .unwrap();

        assert_eq!(
            scope.get_node(&Ident::from("x5")).unwrap().get_value(),
            160.into()
        );
    }
}
