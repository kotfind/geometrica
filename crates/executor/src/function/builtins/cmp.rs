use super::*;

fn array_eq(lhs: &[Value], rhs: &[Value]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }

    for (l, r) in lhs.iter().zip(rhs.iter()) {
        if l != r {
            return false;
        }
    }
    true
}

pub fn populate(builtins: &mut FuncMap) {
    simple_builtin!(INTO builtins INSERT
        // Gr
        fn gr (lhs:  Int, rhs:  Int) -> Bool { lhs        > rhs }
        fn gr (lhs: Real, rhs: Real) -> Bool { lhs        > rhs }
        fn gr (lhs:  Int, rhs: Real) -> Bool { lhs as f64 > rhs }
        fn gr (lhs: Real, rhs:  Int) -> Bool { lhs        > rhs as f64 }
        fn gr (lhs:  Str, rhs:  Str) -> Bool { lhs        > rhs}
        // Le
        fn le (lhs:  Int, rhs:  Int) -> Bool { lhs        < rhs }
        fn le (lhs: Real, rhs: Real) -> Bool { lhs        < rhs }
        fn le (lhs:  Int, rhs: Real) -> Bool { (lhs as f64) < rhs }
        fn le (lhs: Real, rhs:  Int) -> Bool { lhs        < rhs as f64 }
        fn le (lhs:  Str, rhs:  Str) -> Bool { lhs        < rhs }
        // Leq
        fn leq (lhs:  Int, rhs:  Int) -> Bool { lhs        <= rhs }
        fn leq (lhs: Real, rhs: Real) -> Bool { lhs        <= rhs }
        fn leq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 <= rhs }
        fn leq (lhs: Real, rhs:  Int) -> Bool { lhs        <= rhs as f64 }
        fn leq (lhs:  Str, rhs:  Str) -> Bool { lhs        <= rhs }
        // Geq
        fn geq (lhs:  Int, rhs:  Int) -> Bool { lhs        >= rhs }
        fn geq (lhs: Real, rhs: Real) -> Bool { lhs        >= rhs }
        fn geq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 >= rhs }
        fn geq (lhs: Real, rhs:  Int) -> Bool { lhs        >= rhs as f64 }
        fn geq (lhs:  Str, rhs:  Str) -> Bool { lhs        >= rhs }
        // Eq
        fn eq (lhs:  Int, rhs:  Int) -> Bool { lhs        == rhs }
        fn eq (lhs: Real, rhs: Real) -> Bool { lhs        == rhs }
        fn eq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 == rhs }
        fn eq (lhs: Real, rhs:  Int) -> Bool { lhs        == rhs as f64 }
        fn eq (lhs:  Str, rhs:  Str) -> Bool { lhs        == rhs }
        fn eq (lhs: Array, rhs: Array) -> Bool { array_eq(&lhs, &rhs) }
        // Neq
        fn neq (lhs:  Int, rhs:  Int) -> Bool { lhs        != rhs }
        fn neq (lhs: Real, rhs: Real) -> Bool { lhs        != rhs }
        fn neq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 != rhs }
        fn neq (lhs: Real, rhs:  Int) -> Bool { lhs        != rhs as f64 }
        fn neq (lhs:  Str, rhs:  Str) -> Bool { lhs        != rhs }
        fn neq (lhs: Array, rhs: Array) -> Bool { !array_eq(&lhs, &rhs) }
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gr() {
        assert_eq!(eval("3   > 2"), true.into());
        assert_eq!(eval("3.0 > 2"), true.into());
        assert_eq!(eval("3   > 2.0"), true.into());
        assert_eq!(eval("3.0 > 2.0"), true.into());
        assert_eq!(eval(r#""def" > "abc""#), true.into());

        assert_eq!(eval("3   > 3"), false.into());
        assert_eq!(eval("3.0 > 3"), false.into());
        assert_eq!(eval("3   > 3.0"), false.into());
        assert_eq!(eval("3.0 > 3.0"), false.into());
        assert_eq!(eval(r#""abc" > "abc""#), false.into());
    }

    #[test]
    fn le() {
        assert_eq!(eval("2   < 3"), true.into());
        assert_eq!(eval("2.0 < 3"), true.into());
        assert_eq!(eval("2   < 3.0"), true.into());
        assert_eq!(eval("2.0 < 3.0"), true.into());
        assert_eq!(eval(r#""abc" < "def""#), true.into());

        assert_eq!(eval("3   < 3"), false.into());
        assert_eq!(eval("3.0 < 3"), false.into());
        assert_eq!(eval("3   < 3.0"), false.into());
        assert_eq!(eval("3.0 < 3.0"), false.into());
        assert_eq!(eval(r#""abc" > "abc""#), false.into());
    }

    #[test]
    fn geq() {
        assert_eq!(eval("3   >= 2"), true.into());
        assert_eq!(eval("3.0 >= 2"), true.into());
        assert_eq!(eval("3   >= 2.0"), true.into());
        assert_eq!(eval("3.0 >= 2.0"), true.into());
        assert_eq!(eval(r#""def" >= "abc""#), true.into());

        assert_eq!(eval("3   >= 3"), true.into());
        assert_eq!(eval("3.0 >= 3"), true.into());
        assert_eq!(eval("3   >= 3.0"), true.into());
        assert_eq!(eval("3.0 >= 3.0"), true.into());
        assert_eq!(eval(r#""abc" >= "abc""#), true.into());
    }

    #[test]
    fn leq() {
        assert_eq!(eval("2   <= 3"), true.into());
        assert_eq!(eval("2.0 <= 3"), true.into());
        assert_eq!(eval("2   <= 3.0"), true.into());
        assert_eq!(eval("2.0 <= 3.0"), true.into());
        assert_eq!(eval(r#""abc" <= "def""#), true.into());

        assert_eq!(eval("3   <= 3"), true.into());
        assert_eq!(eval("3.0 <= 3"), true.into());
        assert_eq!(eval("3   <= 3.0"), true.into());
        assert_eq!(eval("3.0 <= 3.0"), true.into());
        assert_eq!(eval(r#""abc" <= "abc""#), true.into());
    }

    #[test]
    fn eq() {
        assert_eq!(eval("3   == 3"), true.into());
        assert_eq!(eval("3.0 == 3"), true.into());
        assert_eq!(eval("3   == 3.0"), true.into());
        assert_eq!(eval("3.0 == 3.0"), true.into());
        assert_eq!(eval(r#""abc" == "abc""#), true.into());
        assert_eq!(eval(r#"(1, 2, 3) == (1, 2, 3)"#), true.into());

        assert_eq!(eval("1   == 3"), false.into());
        assert_eq!(eval("1.0 == 3"), false.into());
        assert_eq!(eval("1   == 3.0"), false.into());
        assert_eq!(eval("1.0 == 3.0"), false.into());
        assert_eq!(eval(r#""abc" == "def""#), false.into());
        assert_eq!(eval(r#"(1, 2, 3) == (4, 5, 6)"#), false.into());
    }

    #[test]
    fn neq() {
        assert_eq!(eval("3   != 3"), false.into());
        assert_eq!(eval("3.0 != 3"), false.into());
        assert_eq!(eval("3   != 3.0"), false.into());
        assert_eq!(eval("3.0 != 3.0"), false.into());
        assert_eq!(eval(r#""abc" != "abc""#), false.into());
        assert_eq!(eval(r#"(1, 2, 3) != (1, 2, 3)"#), false.into());

        assert_eq!(eval("1   != 3"), true.into());
        assert_eq!(eval("1.0 != 3"), true.into());
        assert_eq!(eval("1   != 3.0"), true.into());
        assert_eq!(eval("1.0 != 3.0"), true.into());
        assert_eq!(eval(r#""abc" != "def""#), true.into());
        assert_eq!(eval(r#"(1, 2, 3) != (4, 5, 6)"#), true.into());
    }
}
