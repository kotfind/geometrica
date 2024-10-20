= Core Crate

```rust
struct Ident(String);

// Top-level object in language
// Any script is represented as Vec<Stmt>
enum Statement {
    Definition(Definition),
    Command(Command),
}

enum Definition {
    ValueDefinition(ValueDefinition),
    FunctionDefinition(FunctionDefinition),
}

struct ValueDefinition {
    name: Ident,
    type: ValueType,
    is_const: bool,
    value: Expr,
}

struct FunctionDefinition {
    name: Ident,
    arguments: Vec<FunctionDefinitionArgument>,
    return_type: ValueType,
    value: Expr,
}

struct FunctionDefinitionArgument {
    name: Ident,
    type: ValueType,
}

// Non-declarative commands like move, pin, delete, set_transform
struct Command { /* TODO */ }

type Expr = Rc<ExprInner> /* or Box<ExprInner> ??? */;

// Note: operator calls are represented as function calls.
// E.g. `1 + 2` and `add 1 2` are the same
//
// Note: type casts are represented as function calls
//
// Expr vs Node:
// - `Expr`
//     - represents a language structure
//     - may contain a ident (a variable; yet unknown value)
// - `Node`
//     - represent an object (final (that is shown in gui) or intermediate)
//     - no variables (yet unknown values allowed)
//     - stores information about dependencies
enum ExprInner {
    Value(Value),
    Variable(Ident),
    FuncCall(FuncCallExpr),
    If(IfExpr),
    Let(LetExpr),
}

// Note: fails if none of the cases matched and default_case_value is not provided
struct IfExpr {
    cases: Vec<IfExprCase>,
    default_case_value: Option<Expr>,
}

struct IfExprCase {
    condition: Expr,
    value: Expr,
}

struct LetExpr {
    definitions: Vec<LetExprDefinition>,
    value: Expr,
}

struct LetExprDefinition {
    name: Ident,
    value: Expr,
}

struct FuncCallExpr {
    name: Ident,
    arguments: Vec<Expr>,
}

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

type Value = Option<ValueInner>;

enum ValueInner {
    Int(i64),
    Real(f64),
    // A heterogeneous array
    Array(Vec<Value>),
    Point(Point),
    Line(Line),
    Circle(Circle),
}

impl Value {
    fn type(&self) -> Type;
}

enum ValueType {
    Int,
    Real,
    Array,
    Point,
    Line,
    Circle,
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    p1: Point,
    p2: Point,
}

struct Circle {
    center: Point,
    radius: f64,
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
