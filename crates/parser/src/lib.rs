use types::core::Value;

peg::parser! {
    grammar lang() for str {
        // rule script() ->

        // -------------------- Value --------------------
        pub rule value() -> Value
            = none() / real() / int() / _bool() / _str()

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
                &("." / "e") // requires either . or e; otherwise it's int
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
            = "none" { Value::none() }

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
        rule _ = quiet!{(comment() / whitespace())*}

        // Just for testing `_` rule as it cannot be `pub`
        pub rule empty() = _

        pub rule comment()
            = "/*" [^ '*']* "*/"
            / "//" [^ '\n']* "\n"

        pub rule whitespace()
            = "\n" / " " / "\t"
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn value() {
        assert_eq!(lang::value("42"), Ok(42.into()));
        assert_eq!(lang::value("3.14"), Ok((3.14).into()));
        assert_eq!(lang::value("true"), Ok(true.into()));
        assert_eq!(lang::value("none"), Ok(Value::none()));
        assert_eq!(
            lang::value(r#""Hello, world!""#),
            Ok("Hello, world!".to_string().into())
        );
    }

    #[test]
    fn int() {
        assert_eq!(lang::int("-123"), Ok((-123).into()));
    }

    #[test]
    fn real() {
        assert_eq!(lang::real("-12.13e-3"), Ok((-12.13e-3).into()));
    }

    #[test]
    fn bool() {
        assert_eq!(lang::_bool("true"), Ok(true.into()));
        assert_eq!(lang::_bool("false"), Ok(false.into()));
    }

    #[test]
    fn none() {
        assert_eq!(lang::none("none"), Ok(Value::none()));
    }

    #[test]
    fn char() {
        assert_eq!(lang::_char(r#"a"#), Ok('a'));
        assert_eq!(lang::_char(r#"\n"#), Ok('\n'));
        assert_eq!(lang::_char(r#"\\"#), Ok('\\'));
        assert_eq!(lang::_char(r#"\""#), Ok('"'));
    }

    #[test]
    fn array() {
        assert_eq!(lang::array("()"), Ok(vec![].into()));
        assert_eq!(
            lang::array("(1, 2, 3)"),
            Ok(vec![1.into(), 2.into(), 3.into()].into())
        );
        assert_eq!(
            lang::array(r#"(1, 3.14, "hello")"#),
            Ok(vec![1.into(), (3.14).into(), "hello".to_string().into()].into())
        );
    }

    #[test]
    fn str() {
        assert_eq!(lang::_str(r#""abc""#), Ok("abc".to_string().into()));

        assert_eq!(
            lang::_str(r#""abc\"def\"ghi""#),
            Ok("abc\"def\"ghi".to_string().into())
        );

        assert_eq!(lang::_str(r#""\\ \\\n""#), Ok("\\ \\\n".to_string().into()));
    }

    #[test]
    fn comment() {
        assert_eq!(lang::comment("/* abc */"), Ok(()));
        assert_eq!(lang::comment("// abc\n"), Ok(()));
    }

    #[test]
    fn empty() {
        assert_eq!(lang::empty("/* abc */   // xy\n "), Ok(()));
        assert_eq!(lang::empty(""), Ok(()));
    }
}
