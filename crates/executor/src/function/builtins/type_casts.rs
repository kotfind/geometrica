use super::*;

pub fn populate(builtins: &mut FuncMap) {
    simple_builtin!(INTO builtins INSERT
        // As
        fn "#as_bool" (v: Bool) -> Bool { v }
        fn "#as_bool" (v: Int) -> Bool { v != 0 }
        fn "#as_bool" (v: Real) -> Bool { v != 0.0 }
        fn "#as_int" (v: Bool) -> Int { if v { 1 } else { 0 } }
        fn "#as_int" (v: Int) -> Int { v }
        fn "#as_int" (v: Real) -> Int { v as i64 }
        fn "#as_real" (v: Bool) -> Real { if v { 1.0 } else { 0.0 } }
        fn "#as_real" (v: Int) -> Real { v as f64 }
        fn "#as_real" (v: Real) -> Real { v }
    );

    // TODO?: cast str -> *
    // TODO: optimize as_str (don't construct value after just deconstruncting it)
    builtin!(INTO builtins INSERT
        fn "#as_str" (v: Bool) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v: Int) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v: Real) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v: Str) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v: Array) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v: Point) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v: Line) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v: Circle) -> Str { Ok(Value::from(v).to_string()) }
        // Is
        fn "#is_bool" (_v: Bool) -> Bool { Ok(true) }
        fn "#is_bool" (_v: Int) -> Bool { Ok(false) }
        fn "#is_bool" (_v: Real) -> Bool { Ok(false) }
        fn "#is_bool" (_v: Str) -> Bool { Ok(false) }
        fn "#is_bool" (_v: Array) -> Bool { Ok(false) }
        fn "#is_bool" (_v: Point) -> Bool { Ok(false) }
        fn "#is_bool" (_v: Line) -> Bool { Ok(false) }
        fn "#is_bool" (_v: Circle) -> Bool { Ok(false) }
        fn "#is_int" (_v: Bool) -> Bool { Ok(false) }
        fn "#is_int" (_v: Int) -> Bool { Ok(true) }
        fn "#is_int" (_v: Real) -> Bool { Ok(false) }
        fn "#is_int" (_v: Str) -> Bool { Ok(false) }
        fn "#is_int" (_v: Array) -> Bool { Ok(false) }
        fn "#is_int" (_v: Point) -> Bool { Ok(false) }
        fn "#is_int" (_v: Line) -> Bool { Ok(false) }
        fn "#is_int" (_v: Circle) -> Bool { Ok(false) }
        fn "#is_real" (_v: Bool) -> Bool { Ok(false) }
        fn "#is_real" (_v: Int) -> Bool { Ok(false) }
        fn "#is_real" (_v: Real) -> Bool { Ok(true) }
        fn "#is_real" (_v: Str) -> Bool { Ok(false) }
        fn "#is_real" (_v: Array) -> Bool { Ok(false) }
        fn "#is_real" (_v: Point) -> Bool { Ok(false) }
        fn "#is_real" (_v: Line) -> Bool { Ok(false) }
        fn "#is_real" (_v: Circle) -> Bool { Ok(false) }
        fn "#is_str" (_v: Bool) -> Bool { Ok(false) }
        fn "#is_str" (_v: Int) -> Bool { Ok(false) }
        fn "#is_str" (_v: Real) -> Bool { Ok(false) }
        fn "#is_str" (_v: Str) -> Bool { Ok(true) }
        fn "#is_str" (_v: Array) -> Bool { Ok(false) }
        fn "#is_str" (_v: Point) -> Bool { Ok(false) }
        fn "#is_str" (_v: Line) -> Bool { Ok(false) }
        fn "#is_str" (_v: Circle) -> Bool { Ok(false) }
        fn "#is_array" (_v: Bool) -> Bool { Ok(false) }
        fn "#is_array" (_v: Int) -> Bool { Ok(false) }
        fn "#is_array" (_v: Real) -> Bool { Ok(false) }
        fn "#is_array" (_v: Str) -> Bool { Ok(false) }
        fn "#is_array" (_v: Array) -> Bool { Ok(true) }
        fn "#is_array" (_v: Point) -> Bool { Ok(false) }
        fn "#is_array" (_v: Line) -> Bool { Ok(false) }
        fn "#is_array" (_v: Circle) -> Bool { Ok(false) }
        fn "#is_point" (_v: Bool) -> Bool { Ok(false) }
        fn "#is_point" (_v: Int) -> Bool { Ok(false) }
        fn "#is_point" (_v: Real) -> Bool { Ok(false) }
        fn "#is_point" (_v: Str) -> Bool { Ok(false) }
        fn "#is_point" (_v: Array) -> Bool { Ok(false) }
        fn "#is_point" (_v: Point) -> Bool { Ok(true) }
        fn "#is_point" (_v: Line) -> Bool { Ok(false) }
        fn "#is_point" (_v: Circle) -> Bool { Ok(false) }
        fn "#is_line" (_v: Bool) -> Bool { Ok(false) }
        fn "#is_line" (_v: Int) -> Bool { Ok(false) }
        fn "#is_line" (_v: Real) -> Bool { Ok(false) }
        fn "#is_line" (_v: Str) -> Bool { Ok(false) }
        fn "#is_line" (_v: Array) -> Bool { Ok(false) }
        fn "#is_line" (_v: Point) -> Bool { Ok(false) }
        fn "#is_line" (_v: Line) -> Bool { Ok(true) }
        fn "#is_line" (_v: Circle) -> Bool { Ok(false) }
        fn "#is_circle" (_v: Bool) -> Bool { Ok(false) }
        fn "#is_circle" (_v: Int) -> Bool { Ok(false) }
        fn "#is_circle" (_v: Real) -> Bool { Ok(false) }
        fn "#is_circle" (_v: Str) -> Bool { Ok(false) }
        fn "#is_circle" (_v: Array) -> Bool { Ok(false) }
        fn "#is_circle" (_v: Point) -> Bool { Ok(false) }
        fn "#is_circle" (_v: Line) -> Bool { Ok(false) }
        fn "#is_circle" (_v: Circle) -> Bool { Ok(true) }
        // TODO optimize is_ construction
        // is_none
        fn "#is_none" (v: Bool) -> Bool { Ok(v.is_none()) }
        fn "#is_none" (v: Int) -> Bool { Ok(v.is_none()) }
        fn "#is_none" (v: Real) -> Bool { Ok(v.is_none()) }
        fn "#is_none" (v: Str) -> Bool { Ok(v.is_none()) }
        fn "#is_none" (v: Array) -> Bool { Ok(v.is_none()) }
        fn "#is_none" (v: Point) -> Bool { Ok(v.is_none()) }
        fn "#is_none" (v: Line) -> Bool { Ok(v.is_none()) }
        fn "#is_none" (v: Circle) -> Bool { Ok(v.is_none()) }
    );
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn is() {
        // TODO: test point, line, circle
        let value_to_type = [
            ("true", "bool"),
            ("1", "int"),
            ("1.0", "real"),
            (r#""abc""#, "str"),
            ("(1, 2, 3)", "array"),
        ];

        for i in 0..value_to_type.len() {
            for j in 0..=i {
                assert_eq!(
                    eval(&format!(
                        "{value} is {type_}",
                        value = value_to_type[i].0,
                        type_ = value_to_type[j].1
                    )),
                    (i == j).into(),
                );
            }
        }
    }

    #[test]
    fn as_bool() {
        assert_eq!(eval("true as bool"), true.into());
        assert_eq!(eval("5 as bool"), true.into());
        assert_eq!(eval("7.0 as bool"), true.into());

        assert_eq!(eval("false as bool"), false.into());
        assert_eq!(eval("0 as bool"), false.into());
        assert_eq!(eval("0.0 as bool"), false.into());
    }

    #[test]
    fn as_int() {
        assert_eq!(eval("true as int"), 1.into());
        assert_eq!(eval("false as int"), 0.into());
        assert_eq!(eval("5 as int"), 5.into());
        assert_eq!(eval("7.0 as int"), 7.into());
    }

    #[test]
    fn as_real() {
        assert_eq!(eval("true as real"), 1.0.into());
        assert_eq!(eval("false as real"), 0.0.into());
        assert_eq!(eval("5 as real"), 5.0.into());
        assert_eq!(eval("7.0 as real"), 7.0.into());
    }

    #[test]
    fn as_str() {
        assert_eq!(eval("true as str"), "true".to_string().into());
        assert_eq!(eval("false as str"), "false".to_string().into());
        assert_eq!(eval("1 as str"), "1".to_string().into());
        assert_eq!(eval("1.0 as str"), "1.0".to_string().into());
        assert_eq!(
            eval(r#""abacaba" as str"#),
            r#""abacaba""#.to_string().into()
        );
        assert_eq!(eval("(1, 2, 3) as str"), "(1, 2, 3)".to_string().into());
        // TODO: test point, line, circle
    }

    #[test]
    fn none_as_str() {
        assert_eq!(eval("none bool as str"), "none bool".to_string().into());
        assert_eq!(eval("none int as str"), "none int".to_string().into());
        assert_eq!(eval("none real as str"), "none real".to_string().into());
        assert_eq!(eval("none str as str"), "none str".to_string().into());
        assert_eq!(eval("none array as str"), "none array".to_string().into());
        assert_eq!(eval("none point as str"), "none point".to_string().into());
        assert_eq!(eval("none line as str"), "none line".to_string().into());
        assert_eq!(eval("none circle as str"), "none circle".to_string().into());
    }

    #[test]
    fn is_none() {
        assert_eq!(eval("none bool is none"), true.into());
        assert_eq!(eval("none int is none"), true.into());
        assert_eq!(eval("none real is none"), true.into());
        assert_eq!(eval("none str is none"), true.into());
        assert_eq!(eval("none array is none"), true.into());
        assert_eq!(eval("none point is none"), true.into());
        assert_eq!(eval("none line is none"), true.into());
        assert_eq!(eval("none circle is none"), true.into());
        assert_eq!(eval("false is none"), false.into());
        assert_eq!(eval("1 is none"), false.into());
        assert_eq!(eval("1.0 is none"), false.into());
        assert_eq!(eval("\"abacaba\" is none"), false.into());
        assert_eq!(eval("(1, 2, 3) is none"), false.into());
        // TODO: test point, line, circle
    }
}
