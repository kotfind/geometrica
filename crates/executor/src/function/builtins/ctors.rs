use super::*;

use types::core::{Circ, Line, Pt};

pub fn populate(builtins: &mut FuncMap) {
    simple_builtin!(INTO builtins INSERT
        // As
        fn "pt" (x: Real, y: Real) -> Pt { Pt {x, y} }
        fn "line" (p1: Pt, p2: Pt) -> Line { Line {p1, p2} }
        fn "circ" (o: Pt, r: Real) -> Circ { Circ {o, r} }
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cexpr::eval::eval;

    #[test]
    fn pt() {
        assert_eq!(eval("pt 1.0 2.0"), Pt { x: 1.0, y: 2.0 }.into());
    }

    #[test]
    fn line() {
        assert_eq!(
            eval("line (pt 1.0 2.0) (pt 3.0 4.0)"),
            Line {
                p1: Pt { x: 1.0, y: 2.0 },
                p2: Pt { x: 3.0, y: 4.0 }
            }
            .into()
        );
    }

    #[test]
    fn circ() {
        assert_eq!(
            eval("circ (pt 1.0 2.0) 3.0"),
            Circ {
                o: Pt { x: 1.0, y: 2.0 },
                r: 3.0
            }
            .into()
        );
    }
}
