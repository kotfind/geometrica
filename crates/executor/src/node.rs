use std::sync::{Arc, Mutex, Weak};

use types::{
    core::{Value, ValueType},
    lang::Ident,
};

use crate::{
    cexpr::CExpr,
    eval::{Eval, EvalError},
};

#[derive(Clone)]
pub struct WeakNode(pub Weak<NodeInner>);

impl WeakNode {
    pub fn upgrade(&self) -> Option<Node> {
        Weak::upgrade(&self.0).map(Node)
    }
}

#[derive(Clone)]
pub struct Node(pub Arc<NodeInner>);

impl Node {
    pub fn from_value(value: Value) -> Self {
        Node::from(NodeInnerKind::Value(Mutex::new(value)))
    }

    pub fn from_cexpr(body: CExpr, bindings: Vec<(Ident, Node)>) -> Result<Self, EvalError> {
        Ok(Node::from(NodeInnerKind::CExpr(CExprNode {
            value: Mutex::new(
                body.eval(
                    &bindings
                        .iter()
                        .map(|(name, node)| (name.clone(), node.get_value()))
                        .collect(),
                )?,
            ),
            body,
            bindings,
        })))
    }

    pub fn downgrade(&self) -> WeakNode {
        WeakNode(Arc::downgrade(&self.0))
    }

    pub fn value_type(&self) -> ValueType {
        match &self.0.kind {
            NodeInnerKind::Value(value) => value.lock().unwrap().value_type(),
            NodeInnerKind::CExpr(cexpr_node) => cexpr_node.body.0.value_type.clone(),
        }
    }

    pub fn get_value(&self) -> Value {
        match &self.0.kind {
            NodeInnerKind::Value(value) => value.lock().unwrap().clone(),
            NodeInnerKind::CExpr(cexpr) => cexpr.value.lock().unwrap().clone(),
        }
    }

    pub fn set(&self, value: Value) -> Result<(), EvalError> {
        assert!(self.value_type() == value.value_type());

        let NodeInnerKind::Value(val) = &self.0.kind else {
            panic!("set method should only be called on Value-Nodes");
        };

        *val.lock().unwrap() = value;
        self.update()
    }

    fn update_self(&self) -> Result<(), EvalError> {
        if let NodeInnerKind::CExpr(CExprNode {
            value,
            body,
            bindings,
        }) = &self.0.kind
        {
            *value.lock().unwrap() = body.eval(
                &bindings
                    .iter()
                    .map(|(name, node)| (name.clone(), node.get_value()))
                    .collect(),
            )?;
        }
        Ok(())
    }

    fn update(&self) -> Result<(), EvalError> {
        self.update_self()?;
        let required_by = &self.0.required_by.lock().unwrap();
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

pub struct NodeInner {
    pub required_by: Mutex<Vec<WeakNode>>,
    pub kind: NodeInnerKind,
}

pub enum NodeInnerKind {
    Value(Mutex<Value>),
    CExpr(CExprNode),
}

pub struct CExprNode {
    /// Last evaluated value
    pub value: Mutex<Value>,
    pub body: CExpr,
    pub bindings: Vec<(Ident, Node)>,
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
}
