use std::{
    collections::HashSet,
    rc::{Rc, Weak},
};

use types::{
    core::Value,
    lang::{Expr, ExprInner},
};

use crate::{exec::ExecScope, function::Function};

#[derive(Clone)]
pub struct Node(Rc<NodeInner>);

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone)]
struct NodeInner {
    required_by: HashSet<Weak<Node>>,

    kind: NodeInnerKind,
}

#[derive(PartialEq, Clone)]
enum NodeInnerKind {
    Value(Value),
    FuncCall(FuncCallNode),
    If(IfNode),
}

impl NodeInnerKind {
    fn from(expr: Expr, scope: &ExecScope) -> Self {
        match &*expr.0 {
            ExprInner::Value(value) => todo!(),
            ExprInner::Variable(ident) => todo!(),
            ExprInner::FuncCall(func_call_expr) => todo!(),
            ExprInner::If(if_expr) => todo!(),
            ExprInner::Let(let_expr) => todo!(),
        }
    }
}

#[derive(PartialEq, Clone)]
struct FuncCallNode {
    func: Function,
    arguments: Vec<Node>,
}

#[derive(PartialEq, Clone)]
struct IfNode {
    cases: Vec<IfNodeCase>,
    default_case_value: Option<Node>,
}

#[derive(PartialEq, Clone)]
struct IfNodeCase {
    condition: Node,
    value: Node,
}
