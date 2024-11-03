use std::sync::{Arc, Mutex, Weak};

use types::{
    core::{Value, ValueType},
    lang::Ident,
};

use crate::cexpr::CExpr;

#[derive(Clone)]
pub struct WeakNode(pub Weak<Mutex<NodeInner>>);

impl WeakNode {
    pub fn upgrade(&self) -> Option<Node> {
        Weak::upgrade(&self.0).map(Node)
    }
}

#[derive(Clone)]
pub struct Node(pub Arc<Mutex<NodeInner>>);

impl Node {
    pub fn downgrade(&self) -> WeakNode {
        WeakNode(Arc::downgrade(&self.0))
    }

    pub fn value_type(&self) -> ValueType {
        let inner = self.0.lock().unwrap();
        match &inner.kind {
            NodeInnerKind::Value(value) => value.value_type(),
            NodeInnerKind::CExpr(cexpr_node) => cexpr_node.body.0.value_type.clone(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl From<NodeInnerKind> for Node {
    fn from(kind: NodeInnerKind) -> Self {
        Node(Arc::new(Mutex::new(NodeInner {
            required_by: Vec::new(),
            kind,
        })))
    }
}

pub struct NodeInner {
    pub required_by: Vec<WeakNode>,
    pub kind: NodeInnerKind,
}

pub enum NodeInnerKind {
    Value(Value),
    CExpr(CExprNode),
}

pub struct CExprNode {
    pub body: CExpr,
    pub bindings: Vec<(Ident, Node)>,
}
