use types::core::{Circ, Line, Pt, Value};

/// Shift and scale transformation.
///
/// Works like that on the following points:
///
/// * (0, 0) -> (offset.x, offset.y)
/// * (0, 1) -> (offset.x, offset.y + zoom)
///
/// Transformation is applied to convert **real** coordinates
/// into **screen** coordinates.
///
/// Suffixes `_real` and `_screen` in the rest of the code means real coordinates
/// and screen coordinates, respectively.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Transformation {
    pub(super) offset: Pt,
    pub(super) zoom: f64,
}

impl Transformation {
    /// Creates a [Transformation] that transform rectangle
    ///     (-1, -1),
    ///     ( 1, -1),
    ///     ( 1,  1),
    ///     (-1,  1)
    /// into rectangle that exactly fits into this rectangle
    ///     (origin.x,          origin.y),
    ///     (origin.x + size.x, origin.y),
    ///     (origin.x,          origin.y + size.y),
    ///     (origin.x + size.x, origin.y + size.y)
    /// so that their centers are the same
    pub(super) fn from_bounds(origin: Pt, size: Pt) -> Self {
        Self {
            offset: Pt {
                x: origin.x + size.x / 2.0,
                y: origin.y + size.y / 2.0,
            },
            zoom: size.y.min(size.x) / 2.0,
        }
    }

    pub(super) fn identity() -> Self {
        Self {
            offset: Pt { x: 0.0, y: 0.0 },
            zoom: 1.0,
        }
    }

    pub(super) fn inverse(&self) -> Self {
        Self {
            offset: -self.offset / self.zoom,
            zoom: 1.0 / self.zoom,
        }
    }

    pub(super) fn chain(&self, other: &Self) -> Self {
        Self {
            offset: self.offset * other.zoom + other.offset,
            zoom: self.zoom * other.zoom,
        }
    }

    pub(super) fn transform_value(&self, value: &Value) -> Option<Value> {
        match value {
            Value::Pt(Some(pt)) => Some(self.transform_pt(*pt).into()),
            Value::Line(Some(line)) => Some(self.transform_line(*line).into()),
            Value::Circ(Some(circ)) => Some(self.transform_circ(*circ).into()),
            _ => None,
        }
    }

    pub(super) fn transform_pt(&self, p: Pt) -> Pt {
        p * self.zoom + self.offset
    }

    pub(super) fn transform_line(&self, l: Line) -> Line {
        let Line { mut p1, mut p2 } = l;

        p1 = self.transform_pt(p1);
        p2 = self.transform_pt(p2);

        Line { p1, p2 }
    }

    pub(super) fn transform_circ(&self, c: Circ) -> Circ {
        let Circ { mut o, mut r } = c;

        o = self.transform_pt(o);
        r *= self.zoom;

        Circ { o, r }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transform_origin() {
        let offset = Pt { x: 123.0, y: 456.0 };

        let t = Transformation { offset, zoom: 42.0 };

        let pt = Pt { x: 0.0, y: 0.0 };

        assert_eq!(t.transform_pt(pt), offset);
    }

    #[test]
    fn transform_zoom() {
        let t = Transformation {
            offset: Pt { x: 0.0, y: 0.0 },
            zoom: 42.0,
        };

        let pt = Pt { x: 1.0, y: 2.0 };

        assert_eq!(t.transform_pt(pt), Pt { x: 42.0, y: 84.0 });
    }

    #[test]
    fn transform_unit() {
        let offset = Pt { x: 123.0, y: 456.0 };
        let zoom = 42.0;
        let t = Transformation { offset, zoom };

        let pt = Pt { x: 0.0, y: 1.0 };

        assert_eq!(
            t.transform_pt(pt),
            Pt {
                x: offset.x,
                y: offset.y + zoom
            }
        );
    }

    #[test]
    fn transform_complex() {
        let t = Transformation {
            offset: Pt { x: 4.0, y: 5.0 },
            zoom: 6.0,
        };

        let pt = Pt { x: 7.0, y: 8.0 };

        assert_eq!(
            t.transform_pt(pt),
            Pt {
                x: 7.0 * 6.0 + 4.0,
                y: 8.0 * 6.0 + 5.0,
            }
        );
    }

    #[test]
    fn from_bounds_horizontal_fit() {
        let t = Transformation::from_bounds(Pt { x: 100.0, y: 200.0 }, Pt { x: 100.0, y: 500.0 });

        assert_eq!(
            t,
            Transformation {
                offset: Pt { x: 150.0, y: 450.0 },
                zoom: 50.0
            }
        )
    }

    #[test]
    fn from_bounds_vertical_fit() {
        let t = Transformation::from_bounds(Pt { x: 200.0, y: 100.0 }, Pt { x: 500.0, y: 100.0 });

        assert_eq!(
            t,
            Transformation {
                offset: Pt { x: 450.0, y: 150.0 },
                zoom: 50.0
            }
        )
    }

    #[test]
    fn inverse() {
        let t = Transformation {
            offset: Pt { x: 150.0, y: 450.0 },
            zoom: 50.0,
        };
        let t_ = t.inverse();

        let p_orig = Pt { x: 100.0, y: 200.0 };
        let p_trans = t.transform_pt(p_orig);
        let p_back = t_.transform_pt(p_trans);

        assert!(p_orig.dist(p_back) < 1e-6);
    }

    #[test]
    fn chain() {
        let t1 = Transformation {
            offset: Pt { x: 123.0, y: 671.0 },
            zoom: 10.0,
        };
        let t2 = Transformation {
            offset: Pt { x: 71.0, y: 171.0 },
            zoom: 0.5,
        };

        let p = Pt { x: 3.0, y: 9.0 };
        let p1 = t2.transform_pt(t1.transform_pt(p));
        let p2 = t1.chain(&t2).transform_pt(p);

        assert!(p1.dist(p2) < 1e-6);
    }
}
