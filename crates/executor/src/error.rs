use thiserror::Error;
use types::{
    core::ValueType,
    lang::{FunctionSignature, Ident},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("variable '{0}' undefined")]
    UndefinedVariable(Ident),

    #[error("function '{0:?}' undefined")]
    UndefinedFunction(FunctionSignature),

    #[error("unexpected type for {_for}: expected {expected}, got {got}")]
    UnexpectedType {
        _for: String,
        expected: ValueType,
        got: ValueType,
    },

    #[error("got unexpected none value")]
    UnexpectedNone,

    #[error("if: no cases matched, default value not provided")]
    NotingMatched,

    #[error("Redefinition of variable '{0}'")]
    VariableRedefinition(Ident),

    #[error("Redefinition of function '{0:?}'")]
    FunctionRedefinition(FunctionSignature),
}
