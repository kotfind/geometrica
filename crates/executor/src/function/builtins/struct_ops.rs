use super::*;

use types::core::{Circ, Line, Pt};

fn line_to_abc(Line { p1, p2 }: Line) -> (f64, f64, f64) {
    let a = p2.y - p1.y;
    let b = p1.x - p2.x;
    let c = -a * p1.x - b * p1.y;
    (a, b, c)
}

pub(super) fn populate(builtins: &mut FuncMap) {
    simple_builtin!(INTO builtins INSERT
        fn "pt" (x: Real, y: Real) -> Pt { Pt {x, y} }
        fn "x" (p: Pt) -> Real { p.x }
        fn "y" (p: Pt) -> Real { p.y }

        fn "line" (p1: Pt, p2: Pt) -> Line { Line {p1, p2} }
        fn "p1" (l: Line) -> Pt { l.p1 }
        fn "p2" (l: Line) -> Pt { l.p2 }
        fn "a" (l: Line) -> Real { line_to_abc(l).0 }
        fn "b" (l: Line) -> Real { line_to_abc(l).1 }
        fn "c" (l: Line) -> Real { line_to_abc(l).2 }

        fn "circ" (o: Pt, r: Real) -> Circ { Circ {o, r} }
        fn "o" (c: Circ) -> Pt { c.o }
        fn "r" (c: Circ) -> Real { c.r }
    );
}

#[cfg(test)]
mod test {
    use core::panic;

    use types::api::eval;

    use super::*;
    use crate::cexpr::eval::eval;

    #[test]
    fn pt() {
        let pt = Pt { x: 1.0, y: 2.0 };
        assert_eq!(eval("pt 1.0 2.0"), pt.clone().into());
        assert_eq!(eval("x (pt 1.0 2.0)"), pt.x.into());
        assert_eq!(eval("y (pt 1.0 2.0)"), pt.y.into());
    }

    #[test]
    fn line() {
        let p1 = Pt { x: 1.0, y: 2.0 };
        let p2 = Pt { x: 3.0, y: 4.0 };
        let l = Line { p1, p2 };
        assert_eq!(eval("line (pt 1.0 2.0) (pt 3.0 4.0)"), l.clone().into());
        assert_eq!(eval("p1 (line (pt 1.0 2.0) (pt 3.0 4.0))"), l.p1.into());
        assert_eq!(eval("p2 (line (pt 1.0 2.0) (pt 3.0 4.0))"), l.p2.into());
    }

    #[test]
    fn line_to_abc() {
        let p1 = Pt { x: 1.0, y: 2.0 };
        let p2 = Pt { x: 3.0, y: 4.0 };
        let l_str = "line (pt 1.0 2.0) (pt 3.0 4.0)";

        let Value::Real(Some(a)) = eval(&format!("({l_str}).a")) else {
            panic!();
        };
        let Value::Real(Some(b)) = eval(&format!("({l_str}).b")) else {
            panic!();
        };
        let Value::Real(Some(c)) = eval(&format!("({l_str}).c")) else {
            panic!();
        };
        assert!(a * p1.x + b * p1.y + c == 0.into());
        assert!(a * p2.x + b * p2.y + c == 0.into());
    }

    #[test]
    fn circ() {
        let o = Pt { x: 1.0, y: 2.0 };
        let r = 3.0;
        let c = Circ { o, r };
        assert_eq!(eval("circ (pt 1.0 2.0) 3.0"), c.clone().into());
        assert_eq!(eval("o (circ (pt 1.0 2.0) 3.0)"), c.o.into());
        assert_eq!(eval("r (circ (pt 1.0 2.0) 3.0)"), c.r.into());
    }
}
