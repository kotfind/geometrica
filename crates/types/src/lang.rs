use std::rc::Rc;

use crate::core::{Value, ValueType};

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
// Top-level object in language
// Any script is represented as Vec<Stmt>
pub enum Statement {
    Definition(Definition),
    Command(Command),
}

#[derive(Debug, Clone)]
pub enum Definition {
    ValueDefinition(ValueDefinition),
    FunctionDefinition(FunctionDefinition),
}

#[derive(Debug, Clone)]
pub struct ValueDefinition {
    pub name: Ident,
    pub value_type: ValueType,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub signature: FunctionSignature,
    pub return_type: ValueType,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinitionArgument {
    pub name: Ident,
    pub value_type: ValueType,
}

// Non-declarative style commands like move, pin, delete, set_transform, load,
// save
#[derive(Debug, Clone)]
pub struct Command {
    pub name: Ident, // TODO?: Or enum CommandKind
    pub arguments: Vec<Expr>,
}

pub type Expr = Rc<ExprInner>;

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
#[derive(Debug, Clone)]
pub enum ExprInner {
    Value(Value),
    Variable(Ident),
    FuncCall(FuncCallExpr),
    If(IfExpr),
    Let(LetExpr),
}

// Note: fails if none of the cases matched and default_case_value is not provided
#[derive(Debug, Clone)]
pub struct IfExpr {
    pub cases: Vec<IfExprCase>,
    pub default_case_value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct IfExprCase {
    pub condition: Expr,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct LetExpr {
    pub definitions: Vec<LetExprDefinition>,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct LetExprDefinition {
    pub name: Ident,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct FuncCallExpr {
    pub name: Ident,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: Ident,
    pub arguments: Vec<FunctionArgumentType>,
}

// Overrides will conflict if they differ only in `any` argument.
// E.g. having ``, doing `` and `f x:int y:str = ...` is ok, but
// doing or ``
// `
// f x:any y:int = ... // (1) Original function
// f x:any y:str = ... // (2) OK: as `y` has different type
// f x:int y:str = ... // (3) OK: as `y` has different type (but conflicts with (2))
// f x:int y:int = ... // (4) Error: conflicts with (1)
// `
#[derive(Debug, Clone)]
pub enum FunctionArgumentType {
    Any,
    Value(ValueType),
}
