use types::core::Pt;

use super::*;

pub(super) fn populate(builtins: &mut FuncMap) {
    simple_builtin!(INTO builtins INSERT
        // Add
        fn "#add" (lhs:   Pt, rhs:   Pt) -> Pt   { Pt { x: lhs.x + rhs.x, y: lhs.y + rhs.y } }
        fn "#add" (lhs:  Int, rhs:  Int) ->  Int { lhs        + rhs }
        fn "#add" (lhs: Real, rhs: Real) -> Real { lhs        + rhs }
        fn "#add" (lhs:  Int, rhs: Real) -> Real { lhs as f64 + rhs }
        fn "#add" (lhs: Real, rhs:  Int) -> Real { lhs        + rhs as f64 }
        fn "#add" (lhs:  Str, rhs:  Str) -> Str  { lhs + &rhs }
        fn "#add" (lhs: Array, rhs: Array) -> Array {
                let mut lhs = lhs;
                let mut rhs = rhs;
                lhs.append(&mut rhs);
                lhs
        }
        // Sub
        fn "#sub" (lhs:   Pt, rhs:   Pt) -> Pt   { Pt { x: lhs.x - rhs.x, y: lhs.y - rhs.y } }
        fn "#sub" (lhs:  Int, rhs:  Int) -> Int  { lhs        - rhs }
        fn "#sub" (lhs: Real, rhs: Real) -> Real { lhs        - rhs }
        fn "#sub" (lhs:  Int, rhs: Real) -> Real { lhs as f64 - rhs }
        fn "#sub" (lhs: Real, rhs:  Int) -> Real { lhs        - rhs as f64 }
        // Mul
        fn "#mul" (lhs: Real, rhs:   Pt) -> Pt   { Pt { x: rhs.x * lhs,        y: lhs        * rhs.y } }
        fn "#mul" (lhs:  Int, rhs:   Pt) -> Pt   { Pt { x: rhs.x * lhs as f64, y: lhs as f64 * rhs.y } }
        fn "#mul" (lhs:   Pt, rhs: Real) -> Pt   { Pt { x: lhs.x * rhs,        y: rhs        * lhs.y } }
        fn "#mul" (lhs:   Pt, rhs:  Int) -> Pt   { Pt { x: lhs.x * rhs as f64, y: rhs as f64 * lhs.y } }
        fn "#mul" (rhs:  Int, lhs:  Int) -> Int  { lhs        * rhs }
        fn "#mul" (lhs: Real, rhs: Real) -> Real { lhs        * rhs }
        fn "#mul" (lhs:  Int, rhs: Real) -> Real { lhs as f64 * rhs }
        fn "#mul" (lhs: Real, rhs:  Int) -> Real { lhs        * rhs as f64 }
        // Div
        fn "#div" (lhs:   Pt, rhs: Real) -> Pt   { Pt { x: lhs.x / rhs,        y: lhs.y / rhs        } }
        fn "#div" (lhs:   Pt, rhs:  Int) -> Pt   { Pt { x: lhs.x / rhs as f64, y: lhs.y / rhs as f64 } }
        fn "#div" (lhs:  Int, rhs:  Int) -> Int  { lhs        / rhs }
        fn "#div" (lhs: Real, rhs: Real) -> Real { lhs        / rhs }
        fn "#div" (lhs:  Int, rhs: Real) -> Real { lhs as f64 / rhs }
        fn "#div" (lhs: Real, rhs:  Int) -> Real { lhs        / rhs as f64 }
        // Pow
        fn "#pow" (lhs:  Int, rhs:  Int) -> Int  { lhs         .pow(rhs as u32 /* TODO: check if cast fails */) }
        fn "#pow" (lhs: Real, rhs: Real) -> Real { lhs         .powf(rhs) }
        fn "#pow" (lhs:  Int, rhs: Real) -> Real { (lhs as f64).powf(rhs) }
        fn "#pow" (lhs: Real, rhs:  Int) -> Real { lhs         .powi(rhs as i32 /* TODO: check if cast fails */) }
        // Rem
        fn "#rem" (lhs:  Int, rhs:  Int) -> Int  { lhs        % rhs }
        fn "#rem" (lhs: Real, rhs: Real) -> Real { lhs        % rhs }
        fn "#rem" (lhs:  Int, rhs: Real) -> Real { lhs as f64 % rhs }
        fn "#rem" (lhs: Real, rhs:  Int) -> Real { lhs        % rhs as f64 }
        // Minus
        fn "#minus" (v:  Int) -> Int  { -v }
        fn "#minus" (v: Real) -> Real { -v }
        fn "#minus" (pt:  Pt) -> Pt   { Pt { x: -pt.x, y: -pt.y } }
        // Dot & cross
        fn "dot"   (p1: Pt, p2: Pt) -> Real { p1.x * p2.x + p1.y * p2.y }
        fn "cross" (p1: Pt, p2: Pt) -> Real { p1.x * p2.y - p1.y * p2.x }
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cexpr::eval::eval;

    #[test]
    fn add() {
        assert_eq!(
            eval("(pt 1.0 2.0) + (pt 3.0 4.0)"),
            Pt { x: 4.0, y: 6.0 }.into()
        );
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
        assert_eq!(
            eval("(pt 1.0 5.0) - (pt 3.0 4.0)"),
            Pt { x: -2.0, y: 1.0 }.into()
        );
        assert_eq!(eval("1   - 1"), 0.into());
        assert_eq!(eval("1.0 - 1"), 0.0.into());
        assert_eq!(eval("1   - 1.0"), 0.0.into());
        assert_eq!(eval("1.0 - 1.0"), 0.0.into());
    }

    #[test]
    fn mul() {
        assert_eq!(eval("(pt 1.0 2.0) * 3.0"), Pt { x: 3.0, y: 6.0 }.into());
        assert_eq!(eval("(pt 1.0 2.0) * 3"), Pt { x: 3.0, y: 6.0 }.into());
        assert_eq!(eval("3 * (pt 1.0 2.0)"), Pt { x: 3.0, y: 6.0 }.into());
        assert_eq!(eval("3.0 * (pt 1.0 2.0)"), Pt { x: 3.0, y: 6.0 }.into());
        assert_eq!(eval("2   * 2"), 4.into());
        assert_eq!(eval("2.0 * 2"), 4.0.into());
        assert_eq!(eval("2   * 2.0"), 4.0.into());
        assert_eq!(eval("2.0 * 2.0"), 4.0.into());
    }

    #[test]
    fn div() {
        assert_eq!(eval("(pt 3.0 6.0) / 3.0"), Pt { x: 1.0, y: 2.0 }.into());
        assert_eq!(eval("(pt 3.0 6.0) / 3"), Pt { x: 1.0, y: 2.0 }.into());
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

    #[test]
    fn dot() {
        assert_eq!(eval("dot (pt 2.0 3.0) (pt 4.0 5.0)"), 23.0.into());
    }

    #[test]
    fn cross() {
        assert_eq!(eval("cross (pt 2.0 3.0) (pt 4.0 5.0)"), (-2.0).into());
    }
}
