= Core Crate

```rust
struct Scope {
    // Things that don't have arguments
    // TODO?: rename
    named_values: HashMap<Ident, Expr>,

    // Things that don't have arguments
    values: HashSet<Expr>,

    // Things that have arguments
    functions: HashMap<FunctionSignature, Function>,
}

struct FunctionSignature {
    name: Ident,
    arguments: Vec<ValueType>,
}

enum Function {
    BuiltIn(Box<dyn Fn(Vec<Value>) -> Value>),
    Expr(Expr),
}

type Node = Rc<NodeInner>;

struct NodeInner {
    required_by: HashSet<Weak<Node>>,

    // evaluated value
    value: Value,

    kind: NodeInnerKind,
}

enum NodeInnerKind {
    Value(Value),
    FuncCall(FuncCallNode),
    If(IfNode)
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
```
