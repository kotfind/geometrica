use super::*;

pub(super) fn populate(builtins: &mut FuncMap) {
    simple_builtin!(INTO builtins INSERT
        // As
        fn "#as_bool" (v: Bool) -> Bool { v }
        fn "#as_bool" (v:  Int) -> Bool { v != 0 }
        fn "#as_bool" (v: Real) -> Bool { v != 0.0 }
        fn "#as_int"  (v: Bool) -> Int  { if v { 1 } else { 0 } }
        fn "#as_int"  (v:  Int) -> Int  { v }
        fn "#as_int"  (v: Real) -> Int  { v as i64 }
        fn "#as_real" (v: Bool) -> Real { if v { 1.0 } else { 0.0 } }
        fn "#as_real" (v:  Int) -> Real { v as f64 }
        fn "#as_real" (v: Real) -> Real { v }
    );

    // TODO?: cast str -> *
    // TODO: optimize as_str (don't construct value after just deconstruncting it)
    builtin!(INTO builtins INSERT
        fn "#as_str" (v:  Bool) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v:   Int) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v:  Real) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v:   Str) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v: Array) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v:    Pt) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v:  Line) -> Str { Ok(Value::from(v).to_string()) }
        fn "#as_str" (v:  Circ) -> Str { Ok(Value::from(v).to_string()) }
        // // is_none
        // // XXX: those are temporaryly disabled
        // fn "#is_none" (v:  Bool) -> Bool { Ok(v.is_none()) }
        // fn "#is_none" (v:   Int) -> Bool { Ok(v.is_none()) }
        // fn "#is_none" (v:  Real) -> Bool { Ok(v.is_none()) }
        // fn "#is_none" (v:   Str) -> Bool { Ok(v.is_none()) }
        // fn "#is_none" (v: Array) -> Bool { Ok(v.is_none()) }
        // fn "#is_none" (v:    Pt) -> Bool { Ok(v.is_none()) }
        // fn "#is_none" (v:  Line) -> Bool { Ok(v.is_none()) }
        // fn "#is_none" (v:  Circ) -> Bool { Ok(v.is_none()) }
    );
}

#[cfg(test)]
mod test {
    use crate::cexpr::eval::eval;
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
        assert_eq!(eval("1.0 as str"), "1.000".to_string().into());
        assert_eq!(
            eval(r#""abacaba" as str"#),
            r#""abacaba""#.to_string().into()
        );
        assert_eq!(eval("(1, 2, 3) as str"), "(1, 2, 3)".to_string().into());
        assert_eq!(eval("(pt 1.0 2.0) as str"), "pt 1 2".to_string().into());
        assert_eq!(
            eval("(line (pt 1.0 2.0) (pt 3.0 4.0)) as str"),
            "line (pt 1 2) (pt 3 4)".to_string().into()
        );
        assert_eq!(
            eval("(circ (pt 1.0 2.0) 3.0) as str"),
            "circ (pt 1 2) 3".to_string().into()
        );
    }

    #[test]
    fn none_as_str() {
        assert_eq!(eval("none bool as str"), "none bool".to_string().into());
        assert_eq!(eval("none int as str"), "none int".to_string().into());
        assert_eq!(eval("none real as str"), "none real".to_string().into());
        assert_eq!(eval("none str as str"), "none str".to_string().into());
        assert_eq!(eval("none array as str"), "none array".to_string().into());
        assert_eq!(eval("none pt as str"), "none pt".to_string().into());
        assert_eq!(eval("none line as str"), "none line".to_string().into());
        assert_eq!(eval("none circ as str"), "none circ".to_string().into());
    }

    // XXX: those are temporaryly disabled
    // #[test]
    // fn is_none() {
    //     assert_eq!(eval("none bool is none"), true.into());
    //     assert_eq!(eval("none int is none"), true.into());
    //     assert_eq!(eval("none real is none"), true.into());
    //     assert_eq!(eval("none str is none"), true.into());
    //     assert_eq!(eval("none array is none"), true.into());
    //     assert_eq!(eval("none pt is none"), true.into());
    //     assert_eq!(eval("none line is none"), true.into());
    //     assert_eq!(eval("none circ is none"), true.into());
    //     assert_eq!(eval("false is none"), false.into());
    //     assert_eq!(eval("1 is none"), false.into());
    //     assert_eq!(eval("1.0 is none"), false.into());
    //     assert_eq!(eval("\"abacaba\" is none"), false.into());
    //     assert_eq!(eval("(1, 2, 3) is none"), false.into());
    //     assert_eq!(eval("(pt 1.0 2.0) is none"), false.into());
    //     assert_eq!(
    //         eval("(line (pt 1.0 2.0) (pt 3.0 4.0)) is none"),
    //         false.into()
    //     );
    //     assert_eq!(eval("(circ (pt 1.0 2.0) 3.0) is none"), false.into());
    // }
}
