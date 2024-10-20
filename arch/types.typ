= Types Crate

== Base

```rust
type Value = Option<ValueInner>;

enum ValueInner {
    Bool(bool),
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
    Bool,
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

struct Transformation {
    offset: (f64, f64),
    zoom: f64,
}
```

== Lang

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

struct FunctionSignature {
    name: Ident,
    arguments: Vec<ValueType>,
}
```

== Queries

```rust
struct ClientMessage {
    statements: Vec<Statement>,
}

struct ServerMessage {
    warnings: Vec<Warning>,

    errors: Vec<Error>,

    // XXX: Same names may refer to a same name. Add ident to objects?
    values: HashMap<ServerMessageValue>, 

    new_functions: Vec<FunctionSignature>
}

struct ServerMessageValue {
    names: Vec<Ident>,

    // Real (untransformed) value of object. Is used to print info about object.
    original: Value,

    // Transformed value (with transformation applied). Is used to display in
    // GUI.
    transformed: Value,
}

struct Warning {
    // TODO
}

struct Error {
    // TODO
}
```
