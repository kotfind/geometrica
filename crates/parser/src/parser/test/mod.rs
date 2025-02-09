use super::*;

mod expr;
mod ident;
mod statements;
mod value;
mod whitespace;

fn binary_call(
    ident: impl Into<Ident>,
    lhs: impl Into<Expr>,
    rhs: impl Into<Expr>,
) -> FuncCallExpr {
    FuncCallExpr {
        name: ident.into(),
        args: vec![Box::new(lhs.into()), Box::new(rhs.into())],
    }
}
