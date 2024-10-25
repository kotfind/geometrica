use super::*;

#[test]
fn value() {
    assert_eq!(lang::value("42"), Ok(42.into()));
    assert_eq!(lang::value("3.14"), Ok((3.14).into()));
    assert_eq!(lang::value("true"), Ok(true.into()));
    assert_eq!(lang::value("none int"), Ok(Value::none(ValueType::Int)));
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
    assert_eq!(lang::none("none bool"), Ok(Value::none(ValueType::Bool)));
    assert_eq!(lang::none("none int"), Ok(Value::none(ValueType::Int)));
    assert_eq!(lang::none("none real"), Ok(Value::none(ValueType::Real)));
    assert_eq!(lang::none("none str"), Ok(Value::none(ValueType::Str)));
    assert_eq!(lang::none("none array"), Ok(Value::none(ValueType::Array)));
    assert_eq!(lang::none("none point"), Ok(Value::none(ValueType::Point)));
    assert_eq!(lang::none("none line"), Ok(Value::none(ValueType::Line)));
    assert_eq!(
        lang::none("none circle"),
        Ok(Value::none(ValueType::Circle))
    );
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
fn value_type() {
    assert_eq!(lang::value_type("bool"), Ok(ValueType::Bool));
    assert_eq!(lang::value_type("int"), Ok(ValueType::Int));
    assert_eq!(lang::value_type("real"), Ok(ValueType::Real));
    assert_eq!(lang::value_type("str"), Ok(ValueType::Str));
    assert_eq!(lang::value_type("array"), Ok(ValueType::Array));
    assert_eq!(lang::value_type("point"), Ok(ValueType::Point));
    assert_eq!(lang::value_type("line"), Ok(ValueType::Line));
    assert_eq!(lang::value_type("circle"), Ok(ValueType::Circle));
}
