use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::core::{Circ, Line, Pt};

impl Add for Pt {
    type Output = Pt;

    fn add(self, rhs: Self) -> Self {
        let Pt { x: x1, y: y1 } = self;
        let Pt { x: x2, y: y2 } = rhs;

        Pt {
            x: x1 + x2,
            y: y1 + y2,
        }
    }
}

impl Sub for Pt {
    type Output = Pt;

    fn sub(self, rhs: Self) -> Self {
        let Pt { x: x1, y: y1 } = self;
        let Pt { x: x2, y: y2 } = rhs;

        Pt {
            x: x1 - x2,
            y: y1 - y2,
        }
    }
}

impl Mul<f64> for Pt {
    type Output = Pt;

    fn mul(self, rhs: f64) -> Self {
        let Pt { x, y } = self;

        Pt {
            x: x * rhs,
            y: y * rhs,
        }
    }
}

impl Mul<Pt> for f64 {
    type Output = Pt;

    fn mul(self, rhs: Pt) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Pt {
    type Output = Pt;

    fn div(self, rhs: f64) -> Self {
        let Pt { x, y } = self;

        Pt {
            x: x / rhs,
            y: y / rhs,
        }
    }
}

impl Neg for Pt {
    type Output = Pt;

    fn neg(self) -> Self {
        let Pt { x, y } = self;

        Pt { x: -x, y: -y }
    }
}

impl Pt {
    pub fn len(self) -> f64 {
        let Pt { x, y } = self;

        (x * x + y * y).sqrt()
    }

    pub fn dist(self, rhs: Pt) -> f64 {
        (self - rhs).len()
    }

    pub fn cross(self, rhs: Pt) -> f64 {
        let Pt { x: x1, y: y1 } = self;
        let Pt { x: x2, y: y2 } = rhs;

        x1 * y2 - y1 * x2
    }
}

impl Line {
    pub fn dist(self, p: Pt) -> f64 {
        let Line { p1, p2 } = self;
        let p1p2 = p2 - p1;
        let p1p = p - p1;

        p1p2.cross(p1p).abs() / p1p2.len()
    }
}

impl Circ {
    pub fn dist(self, p: Pt) -> f64 {
        let Circ { o, r } = self;

        (o.dist(p) - r).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            Pt { x: 1.0, y: 2.0 } + Pt { x: 5.0, y: 6.0 },
            Pt { x: 6.0, y: 8.0 }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Pt { x: 1.0, y: 2.0 } - Pt { x: 5.0, y: 3.0 },
            Pt { x: -4.0, y: -1.0 }
        );
    }

    #[test]
    fn mul() {
        assert_eq!(Pt { x: 3.0, y: 4.0 } * 5.0, Pt { x: 15.0, y: 20.0 });
        assert_eq!(5.0 * Pt { x: 3.0, y: 4.0 }, Pt { x: 15.0, y: 20.0 });
    }

    #[test]
    fn div() {
        assert_eq!(Pt { x: 15.0, y: 20.0 } / 5.0, Pt { x: 3.0, y: 4.0 });
    }

    #[test]
    fn neg() {
        assert_eq!(-Pt { x: 3.0, y: 1.0 }, Pt { x: -3.0, y: -1.0 });
    }

    #[test]
    fn len() {
        assert_eq!(Pt { x: 3.0, y: 4.0 }.len(), 5.0);
    }

    #[test]
    fn cross() {
        assert_eq!(Pt { x: 2.0, y: 3.0 }.cross(Pt { x: 4.0, y: 5.0 }), -2.0);
    }

    #[test]
    fn dist_pt_pt() {
        assert_eq!(Pt { x: 7.0, y: 9.0 }.dist(Pt { x: 12.0, y: 21.0 }), 13.0);
    }

    #[test]
    fn dist_line_pt() {
        let actual = Line {
            p1: Pt { x: 1.0, y: 0.0 },
            p2: Pt { x: 0.0, y: 1.0 },
        }
        .dist(Pt { x: 1.0, y: 1.0 });

        let expected = 2f64.sqrt() / 2.0;

        assert!((actual - expected).abs() < 1e-8);
    }

    #[test]
    fn dist_circ_pt() {
        assert_eq!(
            Circ {
                o: Pt { x: 1.0, y: 1.0 },
                r: 1.0
            }
            .dist(Pt { x: 2.0, y: 2.0 }),
            2f64.sqrt() - 1.0
        )
    }
}
