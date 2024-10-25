use types::{
    core::Value,
    lang::{Expr, FunctionSignature, Ident},
};

pub struct Function(Box<FunctionInner>);

struct FunctionInner {
    signature: FunctionSignature,
    kind: FunctionKind,
}

enum FunctionKind {
    BuiltIn(Box<dyn Fn(Vec<Value>) -> Value>),
    CustomFunction(CustomFunction),
}

struct CustomFunction {
    arg_names: Vec<Ident>,
    body: Expr,
}
