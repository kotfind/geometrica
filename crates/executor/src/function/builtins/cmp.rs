use super::*;

pub(super) fn populate(builtins: &mut FuncMap) {
    simple_builtin!(INTO builtins INSERT
        // Gr
        fn "#gr" (lhs:  Int, rhs:  Int) -> Bool { lhs > rhs }
        fn "#gr" (lhs: Real, rhs: Real) -> Bool { lhs > rhs }
        fn "#gr" (lhs:  Str, rhs:  Str) -> Bool { lhs > rhs}
        // Le
        fn "#le" (lhs:  Int, rhs:  Int) -> Bool { lhs < rhs }
        fn "#le" (lhs: Real, rhs: Real) -> Bool { lhs < rhs }
        fn "#le" (lhs:  Str, rhs:  Str) -> Bool { lhs < rhs }
        // Leq
        fn "#leq" (lhs:  Int, rhs:  Int) -> Bool { lhs <= rhs }
        fn "#leq" (lhs: Real, rhs: Real) -> Bool { lhs <= rhs }
        fn "#leq" (lhs:  Str, rhs:  Str) -> Bool { lhs <= rhs }
        // Geq
        fn "#geq" (lhs:  Int, rhs:  Int) -> Bool { lhs >= rhs }
        fn "#geq" (lhs: Real, rhs: Real) -> Bool { lhs >= rhs }
        fn "#geq" (lhs:  Str, rhs:  Str) -> Bool { lhs >= rhs }
    );

    builtin!(INTO builtins INSERT
        // Eq
        fn "#eq" (lhs: Bool, rhs: Bool) -> Bool { Ok(lhs == rhs) }
        fn "#eq" (lhs:  Int, rhs:  Int) -> Bool { Ok(lhs == rhs) }
        fn "#eq" (lhs: Real, rhs: Real) -> Bool { Ok(lhs == rhs) }
        fn "#eq" (lhs:  Str, rhs:  Str) -> Bool { Ok(lhs == rhs) }
        fn "#eq" (lhs:   Pt, rhs:   Pt) -> Bool { Ok(lhs == rhs) }
        fn "#eq" (lhs: Line, rhs: Line) -> Bool { Ok(lhs == rhs) }
        fn "#eq" (lhs: Circ, rhs: Circ) -> Bool { Ok(lhs == rhs) }
        // Neq
        fn "#neq" (lhs: Bool, rhs: Bool) -> Bool { Ok(lhs != rhs) }
        fn "#neq" (lhs:  Int, rhs:  Int) -> Bool { Ok(lhs != rhs) }
        fn "#neq" (lhs: Real, rhs: Real) -> Bool { Ok(lhs != rhs) }
        fn "#neq" (lhs:  Str, rhs:  Str) -> Bool { Ok(lhs != rhs) }
        fn "#neq" (lhs:   Pt, rhs:   Pt) -> Bool { Ok(lhs != rhs) }
        fn "#neq" (lhs: Line, rhs: Line) -> Bool { Ok(lhs != rhs) }
        fn "#neq" (lhs: Circ, rhs: Circ) -> Bool { Ok(lhs != rhs) }
    );
}

#[cfg(test)]
mod test {
    use crate::cexpr::eval::eval;

    #[test]
    fn gr() {
        assert_eq!(eval("3   > 2"), true.into());
        assert_eq!(eval("3.0 > 2.0"), true.into());
        assert_eq!(eval(r#""def" > "abc""#), true.into());

        assert_eq!(eval("3   > 3"), false.into());
        assert_eq!(eval("3.0 > 3.0"), false.into());
        assert_eq!(eval(r#""abc" > "abc""#), false.into());
    }

    #[test]
    fn le() {
        assert_eq!(eval("2   < 3"), true.into());
        assert_eq!(eval("2.0 < 3.0"), true.into());
        assert_eq!(eval(r#""abc" < "def""#), true.into());

        assert_eq!(eval("3   < 3"), false.into());
        assert_eq!(eval("3.0 < 3.0"), false.into());
        assert_eq!(eval(r#""abc" > "abc""#), false.into());
    }

    #[test]
    fn geq() {
        assert_eq!(eval("3   >= 2"), true.into());
        assert_eq!(eval("3.0 >= 2.0"), true.into());
        assert_eq!(eval(r#""def" >= "abc""#), true.into());

        assert_eq!(eval("3   >= 3"), true.into());
        assert_eq!(eval("3.0 >= 3.0"), true.into());
        assert_eq!(eval(r#""abc" >= "abc""#), true.into());
    }

    #[test]
    fn leq() {
        assert_eq!(eval("2   <= 3"), true.into());
        assert_eq!(eval("2.0 <= 3.0"), true.into());
        assert_eq!(eval(r#""abc" <= "def""#), true.into());

        assert_eq!(eval("3   <= 3"), true.into());
        assert_eq!(eval("3.0 <= 3.0"), true.into());
        assert_eq!(eval(r#""abc" <= "abc""#), true.into());
    }

    #[test]
    fn eq() {
        // True
        assert_eq!(eval("true == true"), true.into());
        assert_eq!(eval("3   == 3"), true.into());
        assert_eq!(eval("3.0 == 3.0"), true.into());
        assert_eq!(eval(r#""abc" == "abc""#), true.into());
        assert_eq!(eval("(pt 1.0 2.0) == (pt 1.0 2.0)"), true.into());
        assert_eq!(
            eval("line (pt 1.0 2.0) (pt 3.0 4.0) == line (pt 1.0 2.0) (pt 3.0 4.0)"),
            true.into()
        );
        assert_eq!(
            eval("circ (pt 1.0 2.0) 3.0 == circ (pt 1.0 2.0) 3.0"),
            true.into()
        );

        // False
        assert_eq!(eval("true == false"), false.into());
        assert_eq!(eval("3   == 4"), false.into());
        assert_eq!(eval("3.0 == 4.0"), false.into());
        assert_eq!(eval(r#""abc" == "abcd""#), false.into());
        assert_eq!(eval("pt 1.0 2.0 == pt 4.0 2.0"), false.into());
        assert_eq!(
            eval("line (pt 1.0 2.0) (pt 3.0 4.0) == line (pt 2.0 2.0) (pt 3.0 4.0)"),
            false.into()
        );
        assert_eq!(
            eval("circ (pt 1.0 2.0) 3.0 == circ (pt 1.0 2.0) 9.0"),
            false.into()
        );
    }

    #[test]
    fn neq() {
        // True
        assert_eq!(eval("true != false"), true.into());
        assert_eq!(eval("3   != 4"), true.into());
        assert_eq!(eval("3.0 != 4.0"), true.into());
        assert_eq!(eval(r#""abc" != "abcd""#), true.into());
        assert_eq!(eval("pt 1.0 2.0 != pt 4.0 2.0"), true.into());
        assert_eq!(
            eval("line (pt 1.0 2.0) (pt 3.0 4.0) != line (pt 2.0 2.0) (pt 3.0 4.0)"),
            true.into()
        );
        assert_eq!(
            eval("circ (pt 1.0 2.0) 3.0 != circ (pt 1.0 2.0) 9.0"),
            true.into()
        );

        // False
        assert_eq!(eval("false != false"), false.into());
        assert_eq!(eval("3   != 3"), false.into());
        assert_eq!(eval("3.0 != 3.0"), false.into());
        assert_eq!(eval(r#""abc" != "abc""#), false.into());
        assert_eq!(eval("pt 1.0 2.0 != pt 1.0 2.0"), false.into());
        assert_eq!(
            eval("line (pt 1.0 2.0) (pt 3.0 4.0) != line (pt 1.0 2.0) (pt 3.0 4.0)"),
            false.into()
        );
        assert_eq!(
            eval("circ (pt 1.0 2.0) 3.0 != circ (pt 1.0 2.0) 3.0"),
            false.into()
        );
    }

    #[test]
    fn eq_none() {
        // True
        assert_eq!(eval("none bool == none bool"), true.into());
        assert_eq!(eval("none  int == none  int"), true.into());
        assert_eq!(eval("none real == none real"), true.into());
        assert_eq!(eval("none  str == none  str"), true.into());
        assert_eq!(eval("none   pt == none   pt"), true.into());
        assert_eq!(eval("none line == none line"), true.into());
        assert_eq!(eval("none circ == none circ"), true.into());

        // False
        assert_eq!(eval("true == none bool"), false.into());
        assert_eq!(eval("3   == none int"), false.into());
        assert_eq!(eval("3.0 == none real"), false.into());
        assert_eq!(eval(r#""abc" == none str"#), false.into());
        assert_eq!(eval("pt 1.0 2.0 == none pt"), false.into());
        assert_eq!(
            eval("line (pt 1.0 2.0) (pt 3.0 4.0) == none line"),
            false.into()
        );
        assert_eq!(eval("circ (pt 1.0 2.0) 3.0 == none circ"), false.into());
    }
}
