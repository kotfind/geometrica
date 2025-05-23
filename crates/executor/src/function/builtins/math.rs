use types::core::Pt;

use super::*;

pub(super) fn populate(builtins: &mut FuncMap) {
    simple_builtin!(INTO builtins INSERT
        // Add
        fn "#add" (lhs:   Pt, rhs:   Pt) -> Pt   { Pt { x: lhs.x + rhs.x, y: lhs.y + rhs.y } }
        fn "#add" (lhs:  Int, rhs:  Int) ->  Int { lhs + rhs }
        fn "#add" (lhs: Real, rhs: Real) -> Real { lhs + rhs }
        fn "#add" (lhs:  Str, rhs:  Str) -> Str  { lhs + &rhs }
        // Sub
        fn "#sub" (lhs:   Pt, rhs:   Pt) -> Pt   { Pt { x: lhs.x - rhs.x, y: lhs.y - rhs.y } }
        fn "#sub" (lhs:  Int, rhs:  Int) -> Int  { lhs - rhs }
        fn "#sub" (lhs: Real, rhs: Real) -> Real { lhs - rhs }
        // Mul
        fn "#mul" (lhs: Real, rhs:   Pt) -> Pt   { Pt { x: rhs.x * lhs, y: lhs * rhs.y } }
        fn "#mul" (lhs:   Pt, rhs: Real) -> Pt   { Pt { x: lhs.x * rhs, y: rhs * lhs.y } }
        fn "#mul" (rhs:  Int, lhs:  Int) -> Int  { lhs * rhs }
        fn "#mul" (lhs: Real, rhs: Real) -> Real { lhs * rhs }
        // Div
        fn "#div" (lhs:   Pt, rhs: Real) -> Pt   { Pt { x: lhs.x / rhs,        y: lhs.y / rhs        } }
        fn "#div" (lhs:  Int, rhs:  Int) -> Int  { lhs / rhs }
        fn "#div" (lhs: Real, rhs: Real) -> Real { lhs / rhs }
        // Pow
        fn "#pow" (lhs:  Int, rhs:  Int) -> Int  { lhs.pow(rhs as u32 /* TODO: check if cast fails */) }
        fn "#pow" (lhs: Real, rhs: Real) -> Real { lhs.powf(rhs) }
        // Mod
        fn "#mod" (lhs:  Int, rhs:  Int) -> Int  { lhs % rhs }
        fn "#mod" (lhs: Real, rhs: Real) -> Real { lhs % rhs }
        // Neg
        fn "#neg" (v:  Int) -> Int  { -v }
        fn "#neg" (v: Real) -> Real { -v }
        fn "#neg" (pt:  Pt) -> Pt   { Pt { x: -pt.x, y: -pt.y } }
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
        assert_eq!(eval("1.0 + 1.0"), 2.0.into());
        assert_eq!(eval(r#""aba" + "caba""#), "abacaba".to_string().into());
    }

    #[test]
    fn sub() {
        assert_eq!(
            eval("(pt 1.0 5.0) - (pt 3.0 4.0)"),
            Pt { x: -2.0, y: 1.0 }.into()
        );
        assert_eq!(eval("1   - 1"), 0.into());
        assert_eq!(eval("1.0 - 1.0"), 0.0.into());
    }

    #[test]
    fn mul() {
        assert_eq!(eval("(pt 1.0 2.0) * 3.0"), Pt { x: 3.0, y: 6.0 }.into());
        assert_eq!(eval("3.0 * (pt 1.0 2.0)"), Pt { x: 3.0, y: 6.0 }.into());
        assert_eq!(eval("2   * 2"), 4.into());
        assert_eq!(eval("2.0 * 2.0"), 4.0.into());
    }

    #[test]
    fn div() {
        assert_eq!(eval("(pt 3.0 6.0) / 3.0"), Pt { x: 1.0, y: 2.0 }.into());
        assert_eq!(eval("4   / 2"), 2.into());
        assert_eq!(eval("4.0 / 2.0"), 2.0.into());
    }

    #[test]
    fn pow() {
        assert_eq!(eval("2   ^ 2"), 4.into());
        assert_eq!(eval("2.0 ^ 2.0"), 4.0.into());
    }

    #[test]
    fn r#mod() {
        assert_eq!(eval("9   % 7"), 2.into());
        assert_eq!(eval("9.0 % 7.0"), 2.0.into());
    }

    #[test]
    fn neg() {
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
