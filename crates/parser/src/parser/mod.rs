// Note: this file is so big and ugly as peg won't allow splitting grammar in multiple files

use types::{core::*, lang::*};

use super::{infix, unary};

#[cfg(test)]
mod test;

peg::parser! {
    pub grammar lang() for str {
        // -------------------- Statements --------------------
        pub rule script() -> Vec<Statement>
            = _ stmts:(statement() ** __) _
        {
            stmts
        }

        pub rule statement() -> Statement
            = (def:definition() { def.into() })
            / (cmd:command() { cmd.into() })

        pub rule command() -> Command
            = name:ident() "!"
            args:(__ arg:command_arg() { arg })*
        {
            Command { name, args }
        }

        pub rule command_arg() -> CommandArg
            = !statement()
            arg:(
                ident:ident() { ident.into() }
                / expr:expr() { expr.into() }
            ) { arg }

        pub rule definitions() -> Vec<Definition>
            = _ defs:(definition() ** __) _
        {
            defs
        }

        pub rule definition() -> Definition
            = d:function_definition() { d.into() }
            / d:value_definition() { d.into() }

        pub rule function_definition() -> FunctionDefinition
            = name:ident()
            __ args:(function_definition_argument() ** __)
            _ "->" _ return_type:value_type()
            _ "="
            _ body:expr()
        {
            FunctionDefinition { name, args, return_type, body }
        }

        rule function_definition_argument() -> FunctionDefinitionArgument
            = name:ident() _ ":" _ value_type:value_type()
        {
            FunctionDefinitionArgument { name, value_type }
        }

        pub rule value_definition() -> ValueDefinition
            = name:ident() value_type:(":" v:value_type() {v})?
            _ "="
            _ body:expr()
        {
            ValueDefinition { name, value_type, body }
        }

        // -------------------- Expr --------------------
        pub rule expr() -> Expr
            =  !statement() e:precedence! {
                lhs:(@) _ "|" _ rhs:@ { infix(lhs, InfixOp::OR, rhs).into() }

                --

                lhs:(@) _ "&" _ rhs:@ { infix(lhs, InfixOp::AND, rhs).into() }

                --

                // TODO: is none
                lhs:(@) _ ">"  _ rhs:@ { infix(lhs, InfixOp::GR , rhs).into() }
                lhs:(@) _ "<"  _ rhs:@ { infix(lhs, InfixOp::LE , rhs).into() }
                lhs:(@) _ ">=" _ rhs:@ { infix(lhs, InfixOp::GEQ, rhs).into() }
                lhs:(@) _ "<=" _ rhs:@ { infix(lhs, InfixOp::LEQ, rhs).into() }
                lhs:(@) _ "==" _ rhs:@ { infix(lhs, InfixOp::EQ , rhs).into() }
                lhs:(@) _ "!=" _ rhs:@ { infix(lhs, InfixOp::NEQ, rhs).into() }

                --

                lhs:(@) _ "+" _ rhs:@ { infix(lhs, InfixOp::ADD, rhs).into() }
                lhs:(@) _ "-" _ rhs:@ { infix(lhs, InfixOp::SUB, rhs).into() }

                --

                lhs:(@) _ "*" _ rhs:@ { infix(lhs, InfixOp::MUL, rhs).into() }
                lhs:(@) _ "/" _ rhs:@ { infix(lhs, InfixOp::DIV, rhs).into() }
                lhs:(@) _ "%" _ rhs:@ { infix(lhs, InfixOp::MOD, rhs).into() }

                --

                lhs:@ _ ("^" / "**") _ rhs:(@) { infix(lhs, InfixOp::POW, rhs).into() }

                --

                "-" _ rhs:@ { unary(UnaryOp::NEG, rhs).into() }
                "!" _ rhs:@ { unary(UnaryOp::NOT, rhs).into() }

                --

                body:@ _ "." _ name:ident() { DotExpr { name, body: Box::new(body) }.into() }

                --

                body:(@) __ "as" __ value_type:value_type() {
                    AsExpr { body: Box::new(body), value_type }.into()
                }

                array:array() { array.into() } // array

                "(" _ e:expr() _ ")" { e } // braced

                func_call:func_call_expr() { func_call.into() } // function call

                let_expr:let_expr() { let_expr.into() } // let expr

                if_expr:if_expr() { if_expr.into() } // if expr

                val:value() { val.into() } // value

                var:ident() { var.into() } // variable
        } { e }

        // A kind of expr, using that won't be ambiguous without brackets
        rule simple_expr() -> Expr
            = !statement()
            e:(
                ("(" _ e:expr() _ ")" { e }) // braced
                / (var:ident() { var.into() }) // variable
                / (val:value() { val.into() }) // value
            ) { e }

        pub rule func_call_expr() -> FuncCallExpr
            = name:ident() _ args:(simple_expr() ++ __)
        {
            FuncCallExpr { name, args: args.into_iter().map(Box::new).collect() }
        }

        pub rule if_expr() -> IfExpr
            = "if"
            _ cases:(if_expr_case() ++ (_ "," _))
            (_ ",")?
            _ default_value:("else" _ e:expr() { e })?
        {
            IfExpr { cases, default_value: default_value.map(Box::new) }
        }

        rule if_expr_case() -> IfExprCase
            = cond:expr() _ "then" _ value:expr()
        {
            IfExprCase { cond: Box::new(cond), value: Box::new(value) }
        }

        pub rule let_expr() -> LetExpr
            = "let"
            _ defs:(let_expr_definition() ++ (_ "," _))
            (_ ",")?
            _ "in"
            _ body:expr()
        {
            LetExpr { defs, body: Box::new(body) }
        }

        rule let_expr_definition() -> LetExprDefinition
            = name:ident() value_type:(":" t:value_type() {t})?
            _ "="
            _ body:expr()
        {
            LetExprDefinition { name, value_type, body: Box::new(body) }
        }

        // -------------------- Ident --------------------
        pub rule ident() -> Ident
            = !keyword()
                v:$(ident_first_char() ident_char()*)
        {
            Ident(v.to_string())
        }

        rule keyword()
            = ("if" / "let" / "in" / "is" / "as" / "then" / "else" / "none")
                &(whitespace() / eof())

        rule eof() = ![_]

        rule ident_char() -> char
            = c:['0'..='9'] { c } / ident_first_char()

        rule ident_first_char() -> char
            = c:['a'..='z' | 'A'..='Z' | '_'] { c }

        // -------------------- Value --------------------
        pub rule value() -> Value
            = none() / real() / int() / _bool() / _str()

        pub rule value_type() -> ValueType
            = value_type:$(
              "bool"
            / "int"
            / "real"
            / "str"
            / "array"
            / "pt"
            / "line"
            / "circ")
        {
            match value_type {
                "bool" => ValueType::Bool,
                "int" => ValueType::Int,
                "real" => ValueType::Real,
                "str" => ValueType::Str,
                "array" => ValueType::Array,
                "pt" => ValueType::Pt,
                "line" => ValueType::Line,
                "circ" => ValueType::Circ,
                _ => unreachable!()
            }
        }

        pub rule int() -> Value
            = n:$(['+'|'-']?['0'..='9']+)
        {?
            n.parse::<i64>()
                .map(Value::from)
                .or(Err("failed to parse int"))
        }

        pub rule real() -> Value
            = n:$(
                ['+'|'-']? // sign
                ['0'..='9']+ // before dot
                &(("." ['0'..='9']) / "e") // requires either . with digit or e; otherwise it's int
                ("." ['0'..='9']+)? // after dot
                ("e" ['+'|'-']? ['0'..='9']+)? // exponent
            )
        {?
            n.parse::<f64>()
                .map(Value::from)
                .or(Err("failed to parse real"))
        }

        pub rule _bool() -> Value
            = v:$("true" / "false")
        {
            match v {
                "true" => true,
                "false" => false,
                _ => unreachable!()
            }.into()
        }

        pub rule none() -> Value
            = "none" __ value_type:value_type() { Value::none(value_type) }

        pub rule _str() -> Value
            = "\"" s:(_char()*) "\""
        {
            s.iter().collect::<String>().into()
        }

        pub rule _char() -> char
            = r#"\""# { '"' }
            / r#"\n"# { '\n' }
            / r#"\\"# { '\\' }
            / c:[^ '\\' | '"'] { c }

        pub rule array() -> Value
            = "(" _ v:(value() ** (_ "," _)) _ ")"
        {
            v.into()
        }

        // -------------------- Whitespace & Comments --------------------
        // Optional whitespace
        rule _ = quiet!{(comment() / whitespace())*}

        // Mandatory whitespace
        rule __ = quiet!{(comment() / whitespace())+}

        // Just for testing `_` rule as it cannot be `pub`
        pub rule empty() = _

        pub rule comment()
            = "/*" [^ '*']* "*/"
            / "//" [^ '\n']* "\n"

        pub rule whitespace()
            = "\n" / " " / "\t"
    }
}
