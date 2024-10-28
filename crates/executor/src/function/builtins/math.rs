use super::*;

pub fn populate(builtins: &mut FuncMap) {
    simple_builtin!(INTO builtins INSERT
        // Add
        fn add (lhs:  Int, rhs:  Int) ->  Int { lhs        + rhs }
        fn add (lhs: Real, rhs: Real) -> Real { lhs        + rhs }
        fn add (lhs:  Int, rhs: Real) -> Real { lhs as f64 + rhs }
        fn add (lhs: Real, rhs:  Int) -> Real { lhs        + rhs as f64 }
        fn add (lhs:  Str, rhs:  Str) -> Str  { lhs + &rhs }
        fn add (lhs: Array, rhs: Array) -> Array {
                let mut lhs = lhs;
                let mut rhs = rhs;
                lhs.append(&mut rhs);
                lhs
        }
        // Sub
        fn sub (lhs:  Int, rhs:  Int) -> Int  { lhs        - rhs }
        fn sub (lhs: Real, rhs: Real) -> Real { lhs        - rhs }
        fn sub (lhs:  Int, rhs: Real) -> Real { lhs as f64 - rhs }
        fn sub (lhs: Real, rhs:  Int) -> Real { lhs        - rhs as f64 }
        // Mul
        fn mul (lhs:  Int, rhs:  Int) -> Int  { lhs        * rhs }
        fn mul (lhs: Real, rhs: Real) -> Real { lhs        * rhs }
        fn mul (lhs:  Int, rhs: Real) -> Real { lhs as f64 * rhs }
        fn mul (lhs: Real, rhs:  Int) -> Real { lhs        * rhs as f64 }
        // Div
        fn div (lhs:  Int, rhs:  Int) -> Int  { lhs        / rhs }
        fn div (lhs: Real, rhs: Real) -> Real { lhs        / rhs }
        fn div (lhs:  Int, rhs: Real) -> Real { lhs as f64 / rhs }
        fn div (lhs: Real, rhs:  Int) -> Real { lhs        / rhs as f64 }
        // Pow
        fn pow (lhs:  Int, rhs:  Int) -> Int  { lhs       .pow(rhs as u32 /* TODO: check if cast fails */) }
        fn pow (lhs: Real, rhs: Real) -> Real { lhs       .powf(rhs) }
        fn pow (lhs:  Int, rhs: Real) -> Real { (lhs as f64).powf(rhs) }
        fn pow (lhs: Real, rhs:  Int) -> Real { lhs       .powi(rhs as i32 /* TODO: check if cast fails */) }
        // Rem
        fn rem (lhs:  Int, rhs:  Int) -> Int  { lhs        % rhs }
        fn rem (lhs: Real, rhs: Real) -> Real { lhs        % rhs }
        fn rem (lhs:  Int, rhs: Real) -> Real { lhs as f64 % rhs }
        fn rem (lhs: Real, rhs:  Int) -> Real { lhs        % rhs as f64 }
        // Minus
        fn minus (v:  Int) -> Int  { -v }
        fn minus (v: Real) -> Real { -v }
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(eval("1   + 1"), 2.into());
        assert_eq!(eval("1.0 + 1"), 2.0.into());
        assert_eq!(eval("1   + 1.0"), 2.0.into());
        assert_eq!(eval("1.0 + 1.0"), 2.0.into());
        assert_eq!(eval(r#""aba" + "caba""#), "abacaba".to_string().into());
        assert_eq!(
            eval("(1, 2, 3) + (4, 5, 6)"),
            (1..=6).map(Value::from).collect::<Vec<Value>>().into()
        );
    }

    #[test]
    fn sub() {
        assert_eq!(eval("1   - 1"), 0.into());
        assert_eq!(eval("1.0 - 1"), 0.0.into());
        assert_eq!(eval("1   - 1.0"), 0.0.into());
        assert_eq!(eval("1.0 - 1.0"), 0.0.into());
    }

    #[test]
    fn mul() {
        assert_eq!(eval("2   * 2"), 4.into());
        assert_eq!(eval("2.0 * 2"), 4.0.into());
        assert_eq!(eval("2   * 2.0"), 4.0.into());
        assert_eq!(eval("2.0 * 2.0"), 4.0.into());
    }

    #[test]
    fn div() {
        assert_eq!(eval("4   / 2"), 2.into());
        assert_eq!(eval("4.0 / 2"), 2.0.into());
        assert_eq!(eval("4   / 2.0"), 2.0.into());
        assert_eq!(eval("4.0 / 2.0"), 2.0.into());
    }

    #[test]
    fn pow() {
        assert_eq!(eval("2   ^ 2"), 4.into());
        assert_eq!(eval("2.0 ^ 2"), 4.0.into());
        assert_eq!(eval("2   ^ 2.0"), 4.0.into());
        assert_eq!(eval("2.0 ^ 2.0"), 4.0.into());
    }

    #[test]
    fn rem() {
        assert_eq!(eval("9   % 7"), 2.into());
        assert_eq!(eval("9.0 % 7"), 2.0.into());
        assert_eq!(eval("9   % 7.0"), 2.0.into());
        assert_eq!(eval("9.0 % 7.0"), 2.0.into());
    }

    #[test]
    fn minus() {
        assert_eq!(eval("-1"), (-1).into());
        assert_eq!(eval("-1.0"), (-1.0).into());
    }
}
