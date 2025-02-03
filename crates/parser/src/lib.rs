pub use parse_into::ParseInto;
pub use parser::lang::{command, definition, definitions, expr, script, statement};

use types::{core::*, lang::*};

mod parse_into;
mod parser;

fn unary(ident: impl Into<Ident>, arg: impl Into<Expr>) -> FuncCallExpr {
    FuncCallExpr {
        name: ident.into(),
        args: vec![arg.into()],
    }
}

fn binary(ident: impl Into<Ident>, lhs: impl Into<Expr>, rhs: impl Into<Expr>) -> FuncCallExpr {
    FuncCallExpr {
        name: ident.into(),
        args: vec![lhs.into(), rhs.into()],
    }
}
