use std::{
    fmt::{self, Display, Formatter, Write},
    iter,
};

use crate::{
    core::{Circ, Line, Pt, Value, ValueType},
    lang::{
        AsExpr, DotExpr, Expr, FuncCallExpr, FunctionSignature, IfExpr, IfExprCase, InfixExpr,
        InfixOp, LetExpr, LetExprDefinition, UnaryExpr, UnaryOp,
    },
};

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Value(value) => write!(f, "{value}"),
            Expr::Variable(ident) => write!(f, "{ident}"),
            Expr::FuncCall(func_call_expr) => write!(f, "{func_call_expr}"),
            Expr::If(if_expr) => write!(f, "{if_expr}"),
            Expr::Let(let_expr) => write!(f, "{let_expr}"),
            Expr::Infix(infix_expr) => write!(f, "{infix_expr}"),
            Expr::Unary(unary_expr) => write!(f, "{unary_expr}"),
            Expr::As(as_expr) => write!(f, "{as_expr}"),
            Expr::Dot(dot_expr) => write!(f, "{dot_expr}"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(Some(v)) => write!(f, "{v}"),
            Value::Int(Some(v)) => write!(f, "{v}"),
            Value::Real(Some(v)) => {
                let precision = f.precision().unwrap_or(3);
                write!(f, "{v:#.*}", precision)
            }
            Value::Str(Some(v)) => write!(f, "\"{v}\""),
            Value::Pt(Some(v)) => write!(f, "{v}"),
            Value::Line(Some(v)) => write!(f, "{v}"),
            Value::Circ(Some(v)) => write!(f, "{v}"),
            Value::Bool(None)
            | Value::Int(None)
            | Value::Real(None)
            | Value::Str(None)
            | Value::Pt(None)
            | Value::Line(None)
            | Value::Circ(None) => write!(f, "none {}", self.value_type()),
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ValueType::Bool => "bool",
            ValueType::Int => "int",
            ValueType::Real => "real",
            ValueType::Str => "str",
            ValueType::Pt => "pt",
            ValueType::Line => "line",
            ValueType::Circ => "circ",
        };
        write!(f, "{}", s)
    }
}

impl Display for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pt {x} {y}", x = self.x, y = self.y)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line ({p1}) ({p2})", p1 = self.p1, p2 = self.p2)
    }
}

impl Display for Circ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "circ ({o}) {r}", o = self.o, r = self.r)
    }
}

impl Display for FuncCallExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let FuncCallExpr { name, args } = self;

        write!(
            f,
            "{}",
            iter::once(name.to_string())
                .chain(args.iter().map(|arg| format!("({arg})")))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl Display for IfExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let IfExpr {
            cases,
            default_value,
        } = self;

        let mut parts = Vec::new();

        parts.push("if".to_string());

        for case in cases {
            let IfExprCase { cond, value } = case;

            parts.push(format!("({cond}) then ({value}),"));
        }

        if let Some(default_value) = default_value {
            parts.push(format!("else ({default_value})"));
        }

        write!(f, "{}", parts.join(" "))
    }
}

impl Display for LetExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let LetExpr { defs, body } = self;

        let mut parts = Vec::new();

        parts.push("let".to_string());

        for def in defs {
            let LetExprDefinition {
                name,
                value_type,
                body,
            } = def;

            let mut s = String::new();
            write!(s, "{name}")?;
            if let Some(value_type) = value_type {
                write!(s, ":{value_type}")?;
            }
            write!(s, " = {body},")?;

            parts.push(s);
        }

        parts.push("in".to_string());

        parts.push(format!("{body}"));

        write!(f, "{}", parts.join(" "))
    }
}

impl Display for InfixExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let InfixExpr { lhs, op, rhs } = self;

        write!(f, "({lhs}) {op} ({rhs})")
    }
}

impl Display for InfixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            InfixOp::OR => "|",
            InfixOp::AND => "&",
            InfixOp::GR => ">",
            InfixOp::LE => "<",
            InfixOp::GEQ => ">=",
            InfixOp::LEQ => "<=",
            InfixOp::EQ => "==",
            InfixOp::NEQ => "!=",
            InfixOp::ADD => "+",
            InfixOp::SUB => "-",
            InfixOp::MUL => "*",
            InfixOp::DIV => "/",
            InfixOp::MOD => "%",
            InfixOp::POW => "^",
        };

        write!(f, "{s}")
    }
}
impl Display for UnaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let UnaryExpr { op, body } = self;

        write!(f, "{op}({body})")
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UnaryOp::NOT => "!",
            UnaryOp::NEG => "-",
        };

        write!(f, "{s}")
    }
}

impl Display for AsExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let AsExpr { body, value_type } = self;

        write!(f, "({body}) as {value_type}")
    }
}

impl Display for DotExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let DotExpr { name, body } = self;

        write!(f, "({body}).{name}")
    }
}

impl Display for FunctionSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let FunctionSignature { name, arg_types } = self;

        let mut parts = Vec::new();

        parts.push(name.0.clone());

        for arg_type in arg_types {
            parts.push(arg_type.to_string());
        }

        write!(f, "{}", parts.join(" "))
    }
}
