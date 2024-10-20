= Types Crate

== Base

```rust
type Value = Option<ValueInner>;

enum ValueInner {
    Bool(bool),
    Int(i64),
    Real(f64),
    Str(Box<String> /* Or w/o Box, or with Rc */),
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
    Str,
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
    body: Expr,
}

struct FunctionDefinition {
    signature: FunctionSignature,
    return_type: ValueType,
    body: Expr,
}

struct FunctionDefinitionArgument {
    name: Ident,
    type: ValueType,
}

// Non-declarative style commands like move, pin, delete, set_transform, load,
// save
struct Command {
    name: Ident, // TODO?: Or enum CommandKind
    arguments: Vec<Expr>,
}

type Expr = Rc<ExprInner> /* or Box<ExprInner> ??? */;

// Note: operator calls are represented as function calls.
// E.g. `1 + 2` and `#add 1 2` are the same
//
// Note: type casts are represented as function calls

// Note: type checks (`is` operator) are represented as function calls.
// E.g. `x is int` and `#is_int x` are the same
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
    arguments: Vec<FunctionArgumentType>,
}

// Overrides will conflict if they differ only in `any` argument.
// E.g. having ``, doing `` and `f x:int y:str = ...` is ok, but
// doing or ``
// `
// f x:any y:int = ... // (1) Original function
// f x:any y:str = ... // (2) OK: as `y` has different type
// f x:int y:str = ... // (3) OK: as `y` has different type (but conflicts with (2))
// f x:int y:int = ... // (4) Error: conflicts with (1)
// `
struct FunctionArgumentType {
    Any,
    Value(ValueType),
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
