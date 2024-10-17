= Lang Crate

```rust
struct Ident(String);

enum Command {
    FuncDef(Ident, FuncExpr),
    ConstDef(Ident, Expr /* TODO: not all exprs work here */),
    CreateCommand(/* ... */),
    // ...
}

enum Expr {
    Value(Value),
    Math(MathExpr),
    If(Vec<IfExprItem>),
    For(ForExpr),
    Let(LetExpr),
    Assign(AssignExpr),
    Block(Vec<Expr>),
    // TODO?: clone expr
    // TODO: func defenition
    // TODO: func execution
}

impl Expr {
    // Returns Error when, for instance, if's cases have different types
    fn type() -> Result<ValueType, Error /* TODO: actual type */>;
}

enum MathExpr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

struct IfExprItem {
    cond: Box<Expr>,
    body: Box<Expr>,
}

struct ForExpr {
    var_ident: Ident,
    from: f64, // TODO?: integer type
    to: f64,
    body: Expr,
}

struct AssignExpr(Ident /* TODO: Expr as lhs */, Box<Expr>);

struct LetExpr(Ident, Option<Box<Expr>>);

struct FuncExpr(/* TODO */)
```
