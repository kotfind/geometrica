//! This module implements exporting ExecScope to svg.

use core::f64;

use indoc::indoc;
use svg::{
    node::element::{Circle as SvgCircle, Line as SvgLine, Style},
    Document,
};
use types::core::{Circ, Line, Pt, Value};

use crate::exec::ExecScope;

const STROKE_WIDTH: f64 = 1.0;
const PT_RADIUS: f64 = 4.0;
const VIEWBOX_PADDING: f64 = 10.0;

const PT_CLASS: &str = "pt";
const LINE_CLASS: &str = "line";
const CIRC_CLASS: &str = "circ";

impl ExecScope {
    pub fn to_svg(&self) -> String {
        let (bounds, scale) = self.get_bounds_and_scale();
        let doc = self.build_doc(bounds, scale);
        doc.to_string()
    }

    /// Svg does not seem to like float numbers,
    /// so just scaling everything.
    fn get_bounds_and_scale(&self) -> ([f64; 4], f64) {
        let mut bounds = None;

        for value in self.get_all_items().values() {
            bounds = value.update_bounds(bounds);
        }

        let [min_x, min_y, max_x, max_y] = bounds.unwrap_or([0.0; 4]);
        let wid = max_x - min_x;
        let hei = max_y - min_y;
        let res = [wid, hei]
            .into_iter()
            .filter(|v| *v != 0.0)
            .min_by(|x, y| x.total_cmp(y))
            .unwrap_or(1.0);
        let scale = 1000.0 / res;

        (
            [
                scale * min_x - VIEWBOX_PADDING,
                scale * min_y - VIEWBOX_PADDING,
                scale * wid + 2.0 * VIEWBOX_PADDING,
                scale * hei + 2.0 * VIEWBOX_PADDING,
            ],
            scale,
        )
    }

    fn build_doc(&self, bounds: [f64; 4], scale: f64) -> Document {
        let mut doc = Document::new()
            .add(svg_style())
            .set("viewBox", <(f64, f64, f64, f64)>::from(bounds));

        for value in self.get_all_items().values() {
            doc = value.populate_doc(doc, scale);
        }

        doc
    }
}

fn svg_style() -> Style {
    Style::new(format!(
        indoc!(
            r"
            .{line} {{
                fill: black;
                stroke: black;
                stroke-width: {stroke_width};
                stroke-linecap: round;
            }}

            .{pt} {{
                fill: black;
                stroke: black;
                stroke-width: {pt_radius};
                stroke-linecap: round;
            }}

            .{circ} {{
                fill: none;
                stroke: black;
                stroke-width: {stroke_width};
            }}
        "
        ),
        line = LINE_CLASS,
        pt = PT_CLASS,
        circ = CIRC_CLASS,
        pt_radius = PT_RADIUS,
        stroke_width = STROKE_WIDTH,
    ))
}

type Bounds = Option<[f64; 4]>;

trait ToSvg {
    // [bounds]: (min_x, min_y, max_x, max_y)
    fn update_bounds(&self, bounds: Bounds) -> Bounds;

    fn populate_doc(&self, doc: Document, scale: f64) -> Document;
}

impl ToSvg for Value {
    fn update_bounds(&self, bounds: Bounds) -> Bounds {
        match self {
            Value::Pt(Some(pt)) => pt.update_bounds(bounds),
            Value::Line(Some(line)) => line.update_bounds(bounds),
            Value::Circ(Some(circ)) => circ.update_bounds(bounds),
            _ => bounds,
        }
    }

    fn populate_doc(&self, doc: Document, scale: f64) -> Document {
        match self {
            Value::Pt(Some(pt)) => pt.populate_doc(doc, scale),
            Value::Line(Some(line)) => line.populate_doc(doc, scale),
            Value::Circ(Some(circ)) => circ.populate_doc(doc, scale),
            _ => doc,
        }
    }
}

impl ToSvg for Pt {
    fn update_bounds(&self, bounds: Bounds) -> Bounds {
        let Pt { x, y } = *self;
        Some(match bounds {
            Some([min_x, min_y, max_x, max_y]) => {
                [min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y)]
            }
            None => [x, y, x, y],
        })
    }

    fn populate_doc(&self, doc: Document, scale: f64) -> Document {
        let Pt { mut x, mut y } = self;
        x *= scale;
        y *= scale;

        let node = SvgLine::new()
            .set("class", PT_CLASS)
            .set("x1", x)
            .set("y1", y)
            .set("x2", x)
            .set("y2", y);

        doc.add(node)
    }
}

impl ToSvg for Line {
    fn update_bounds(&self, bounds: Bounds) -> Bounds {
        let bounds = self.p1.update_bounds(bounds);
        self.p2.update_bounds(bounds)
    }

    fn populate_doc(&self, doc: Document, scale: f64) -> Document {
        let Line {
            p1: Pt { x: x1, y: y1 },
            p2: Pt { x: x2, y: y2 },
        } = self;

        let node = SvgLine::new()
            .set("class", LINE_CLASS)
            .set("x1", scale * x1)
            .set("y1", scale * y1)
            .set("x2", scale * x2)
            .set("y2", scale * y2);

        doc.add(node)
    }
}

impl ToSvg for Circ {
    fn update_bounds(&self, mut bounds: Bounds) -> Bounds {
        let Circ { o, r } = *self;

        let pts = [
            o,
            o + Pt { x: r, y: 0.0 },
            o + Pt { x: -r, y: 0.0 },
            o + Pt { x: 0.0, y: r },
            o + Pt { x: 0.0, y: -r },
        ]
        .into_iter();

        for pt in pts {
            bounds = pt.update_bounds(bounds);
        }

        bounds
    }

    fn populate_doc(&self, doc: Document, scale: f64) -> Document {
        let Circ { o: Pt { x, y }, r } = self;
        let node = SvgCircle::new()
            .set("class", CIRC_CLASS)
            .set("cx", scale * x)
            .set("cy", scale * y)
            .set("r", scale * r);

        doc.add(node)
    }
}
