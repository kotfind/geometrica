use super::*;

pub fn val_def(name: Ident, value_type: ValueType, body: Expr) -> Statement {
    let def: Definition = ValueDefinition {
        name,
        value_type,
        body,
    }
    .into();
    def.into()
}

pub fn func_def(
    name: Ident,
    args: Vec<(Ident, FunctionArgumentType)>,
    return_type: ValueType,
    body: Expr,
) -> Statement {
    let def: Definition = FunctionDefinition {
        name,
        argugments: args
            .into_iter()
            .map(|(name, value_type)| FunctionDefinitionArgument { name, value_type })
            .collect(),
        return_type,
        body,
    }
    .into();
    def.into()
}

pub fn cmd(name: Ident, args: Vec<Expr>) -> Statement {
    Command {
        name,
        arguments: args,
    }
    .into()
}

pub fn val_expr(v: Value) -> Expr {
    Rc::new(v.into())
}

pub fn ident_expr(v: Ident) -> Expr {
    Rc::new(v.into())
}

pub fn func_expr(name: Ident, args: Vec<Expr>) -> Expr {
    Rc::new(
        FuncCallExpr {
            name,
            arguments: args,
        }
        .into(),
    )
}

pub fn if_expr(cases: Vec<(Expr, Expr)>, defalut_value: Option<Expr>) -> Expr {
    Rc::new(
        IfExpr {
            cases: cases
                .into_iter()
                .map(|(cond, val)| IfExprCase {
                    condition: cond,
                    value: val,
                })
                .collect(),
            default_case_value: defalut_value,
        }
        .into(),
    )
}

pub fn let_expr(defs: Vec<(Ident, Expr)>, val: Expr) -> Expr {
    Rc::new(
        LetExpr {
            definitions: defs
                .into_iter()
                .map(|(name, val)| LetExprDefinition { name, value: val })
                .collect(),
            value: val,
        }
        .into(),
    )
}

pub fn ident(v: impl ToString) -> Ident {
    Ident(v.to_string())
}
