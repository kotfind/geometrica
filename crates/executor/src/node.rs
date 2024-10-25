use std::{
    collections::HashSet,
    rc::{Rc, Weak},
};

use types::core::Value;

use crate::function::Function;

pub struct Node(Rc<NodeInner>);

struct NodeInner {
    required_by: HashSet<Weak<Node>>,

    value: Value,

    kind: NodeInnerKind,
}

enum NodeInnerKind {
    Value(Value),
    FuncCall(FuncCallNode),
}

struct FuncCallNode {
    func: Function,
    arguments: Vec<Node>,
}

struct IfNode {
    cases: Vec<IfNodeCase>,
    default_case_value: Option<Node>,
}

struct IfNodeCase {
    condition: Node,
    value: Node,
}
