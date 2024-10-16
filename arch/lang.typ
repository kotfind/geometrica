= Lang Crate

```rust
enum Expr {
    Var(Var),
    Math(MathExpr),
    If(Vec<CaseIfItem>),
    For(ForExpr),
    // TODO: clone expr
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
    var_name: String, // TODO: ident
    from: f64, // TODO?: integer type
    to: f64,
}
```
