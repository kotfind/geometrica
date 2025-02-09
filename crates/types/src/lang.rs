#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Ident, Value, ValueType};

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
/// Top-level object in the language.
/// Any script is represented as Vec<Stmt>
pub enum Statement {
    Definition(Definition),
    Command(Command),
}

enum_from_variant!(Statement, Definition, Definition);
enum_from_variant!(Statement, Command, Command);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Definition {
    ValueDefinition(ValueDefinition),
    FunctionDefinition(FunctionDefinition),
}

enum_from_variant!(Definition, ValueDefinition, ValueDefinition);
enum_from_variant!(Definition, FunctionDefinition, FunctionDefinition);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValueDefinition {
    pub name: Ident,
    pub value_type: Option<ValueType>,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionDefinition {
    pub name: Ident,
    pub args: Vec<FunctionDefinitionArgument>,
    pub return_type: ValueType,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionDefinitionArgument {
    pub name: Ident,
    pub value_type: ValueType,
}

// Non-declarative style commands like move, pin, delete, set_transform, load,
// save
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub name: Ident,
    pub args: Vec<CommandArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandArg {
    Expr(Expr),
    Ident(Ident),
}

enum_from_variant!(CommandArg, Expr, Expr);
enum_from_variant!(CommandArg, Ident, Ident);

// Note: operator calls are represented as function calls.
// E.g. `1 + 2` and `#add 1 2` are the same
//
// Note: type casts are represented as function calls
//
// Note: type checks (`is` operator) are represented as function calls.
// E.g. `x is int` and `#is_int x` are the same
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expr {
    Value(Value),
    Variable(Ident),
    FuncCall(FuncCallExpr),
    If(IfExpr),
    Let(LetExpr),
    Infix(InfixExpr),
    Unary(UnaryExpr),
    As(AsExpr),
    Dot(DotExpr),
}

enum_from_variant!(Expr, Value, Value);
enum_from_variant!(Expr, Variable, Ident);
enum_from_variant!(Expr, FuncCall, FuncCallExpr);
enum_from_variant!(Expr, If, IfExpr);
enum_from_variant!(Expr, Let, LetExpr);
enum_from_variant!(Expr, Infix, InfixExpr);
enum_from_variant!(Expr, Unary, UnaryExpr);
enum_from_variant!(Expr, As, AsExpr);
enum_from_variant!(Expr, Dot, DotExpr);

// Note: fails if none of the cases matched and default_case_value is not provided
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IfExpr {
    pub cases: Vec<IfExprCase>,
    pub default_value: Option<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IfExprCase {
    pub cond: Box<Expr>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LetExpr {
    pub defs: Vec<LetExprDefinition>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LetExprDefinition {
    pub name: Ident,
    pub value_type: Option<ValueType>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FuncCallExpr {
    pub name: Ident,
    pub args: Vec<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionSignature {
    pub name: Ident,
    pub arg_types: Vec<ValueType>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InfixExpr {
    pub lhs: Box<Expr>,
    pub op: InfixOp,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InfixOp {
    OR,  // |
    AND, // &
    GR,  // >
    LE,  // <
    GEQ, // >=
    LEQ, // <=
    EQ,  // ==
    NEQ, // !=
    ADD, // +
    SUB, // -
    MUL, // *
    DIV, // /
    MOD, // %
    POW, // ** or ^
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnaryOp {
    NOT, // !
    NEG, // -
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AsExpr {
    pub body: Box<Expr>,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DotExpr {
    pub name: Ident,
    pub body: Box<Expr>,
}
