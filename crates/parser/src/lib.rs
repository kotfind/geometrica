pub use parse_into::ParseInto;
pub use parser::lang::{command, definition, definitions, expr, script, statement};

use types::lang::*;

mod parse_into;
mod parser;

fn unary(op: UnaryOp, body: impl Into<Expr>) -> UnaryExpr {
    UnaryExpr {
        op,
        body: Box::new(body.into()),
    }
}

fn infix(lhs: impl Into<Expr>, op: InfixOp, rhs: impl Into<Expr>) -> InfixExpr {
    InfixExpr {
        lhs: Box::new(lhs.into()),
        op,
        rhs: Box::new(rhs.into()),
    }
}
