use iced::{
    widget::canvas::{self, Path},
    Color,
};
use types::core::{Circ, Line, Pt, Value};

use super::helpers::pt_to_point;

pub(super) static POINT_WIDTH: f32 = 5.;
pub(super) static LINE_WIDTH: f32 = 1.;

pub(super) fn draw_value(value: &Value, frame: &mut canvas::Frame, color: Color) {
    match value {
        Value::Pt(Some(pt)) => {
            draw_pt(pt, frame, color);
        }
        Value::Line(Some(line)) => {
            draw_line(line, frame, color);
        }
        Value::Circ(Some(circ)) => {
            draw_circ(circ, frame, color);
        }
        _ => {}
    }
}

fn draw_pt(pt: &Pt, frame: &mut canvas::Frame, color: Color) {
    let path = Path::circle(pt_to_point(pt), POINT_WIDTH);
    frame.fill(
        &path,
        canvas::Fill {
            style: canvas::Style::Solid(color),
            ..Default::default()
        },
    );
}

fn draw_line(Line { p1, p2 }: &Line, frame: &mut canvas::Frame, color: Color) {
    let path = Path::line(pt_to_point(p1), pt_to_point(p2));
    frame.stroke(
        &path,
        canvas::Stroke {
            style: canvas::Style::Solid(color),
            width: LINE_WIDTH,
            ..Default::default()
        },
    );
}

fn draw_circ(Circ { o, r }: &Circ, frame: &mut canvas::Frame, color: Color) {
    let path = Path::circle(pt_to_point(o), *r as f32);
    frame.stroke(
        &path,
        canvas::Stroke {
            style: canvas::Style::Solid(color),
            width: LINE_WIDTH,
            ..Default::default()
        },
    );
}
