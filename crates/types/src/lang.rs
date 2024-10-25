use std::{fmt::Display, sync::Arc};

use crate::core::{Value, ValueType};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Ident(pub String);

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Ident {
    fn from(v: &str) -> Self {
        Ident(v.to_string())
    }
}

macro_rules! enum_from_variant {
    ($enum:ty, $variant:ident, $inner_type:ty) => {
        impl From<$inner_type> for $enum {
            fn from(v: $inner_type) -> $enum {
                <$enum>::$variant(v)
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
// Top-level object in language
// Any script is represented as Vec<Stmt>
pub enum Statement {
    Definition(Definition),
    Command(Command),
}

enum_from_variant!(Statement, Definition, Definition);
enum_from_variant!(Statement, Command, Command);

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    ValueDefinition(ValueDefinition),
    FunctionDefinition(FunctionDefinition),
}

enum_from_variant!(Definition, ValueDefinition, ValueDefinition);
enum_from_variant!(Definition, FunctionDefinition, FunctionDefinition);

#[derive(Debug, Clone, PartialEq)]
pub struct ValueDefinition {
    pub name: Ident,
    pub value_type: Option<ValueType>,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub name: Ident,
    pub args: Vec<FunctionDefinitionArgument>,
    pub return_type: ValueType,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinitionArgument {
    pub name: Ident,
    pub value_type: ValueType,
}

// Non-declarative style commands like move, pin, delete, set_transform, load,
// save
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub name: Ident, // TODO?: Or enum CommandKind
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr(pub Arc<ExprInner>);

impl<T: Into<ExprInner>> From<T> for Expr {
    fn from(v: T) -> Self {
        Expr(Arc::new(v.into()))
    }
}

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
#[derive(Debug, Clone, PartialEq)]
pub enum ExprInner {
    Value(Value),
    Variable(Ident),
    FuncCall(FuncCallExpr),
    If(IfExpr),
    Let(LetExpr),
}

enum_from_variant!(ExprInner, Value, Value);
enum_from_variant!(ExprInner, Variable, Ident);
enum_from_variant!(ExprInner, FuncCall, FuncCallExpr);
enum_from_variant!(ExprInner, If, IfExpr);
enum_from_variant!(ExprInner, Let, LetExpr);

// Note: fails if none of the cases matched and default_case_value is not provided
#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub cases: Vec<IfExprCase>,
    pub default_case_value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExprCase {
    pub condition: Expr,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetExpr {
    pub definitions: Vec<LetExprDefinition>,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetExprDefinition {
    pub name: Ident,
    pub value_type: Option<ValueType>,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncCallExpr {
    pub name: Ident,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct FunctionSignature {
    pub name: Ident,
    pub arg_types: Vec<ValueType>,
}
