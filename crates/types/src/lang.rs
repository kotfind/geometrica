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

// Note: operator calls are represented as function calls.
// E.g. `1 + 2` and `#add 1 2` are the same
//
// Note: type casts are represented as function calls
//
// Note: type checks (`is` operator) are represented as function calls.
// E.g. `x is int` and `#is_int x` are the same
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Value(Value),
    Variable(Ident),
    FuncCall(FuncCallExpr),
    If(IfExpr),
    Let(LetExpr),
}

enum_from_variant!(Expr, Value, Value);
enum_from_variant!(Expr, Variable, Ident);
enum_from_variant!(Expr, FuncCall, FuncCallExpr);
enum_from_variant!(Expr, If, IfExpr);
enum_from_variant!(Expr, Let, LetExpr);

// Note: fails if none of the cases matched and default_case_value is not provided
#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub cases: Vec<IfExprCase>,
    pub default_value: Option<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExprCase {
    pub cond: Box<Expr>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetExpr {
    pub defs: Vec<LetExprDefinition>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetExprDefinition {
    pub name: Ident,
    pub value_type: Option<ValueType>,
    pub body: Box<Expr>,
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
