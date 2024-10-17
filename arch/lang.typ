= Lang Crate

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

// Non-declarative commands like move, pin, delete
struct Command { /* TODO */ }

type Expr = Rc<ExprInner>;

// Note: operator calls are represented as function calls.
// E.g. `1 + 2` and `add 1 2` are the same
enum ExprInner {
    Value(Value),
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
    values: HashMap<Ident, Expr>,

    // Things that have arguments
    functions: HashMap<FunctionSignature, Function>,
}

struct FunctionSignature {
    name: Ident,
    arguments: Vec<ValueType>,
}

enum Function {
    BuiltIn(Box<dyn Fn(Vec<Value>, &Scope) -> Value>),
    Expr(Expr),
}
```
