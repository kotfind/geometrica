use iced::{
    widget::canvas::{self, Path},
    Color,
};
use types::core::{Circ, Line, Pt, Value};

use super::helpers::pt_to_point;

pub(super) static POINT_WIDTH: f32 = 5.;
pub(super) static LINE_WIDTH: f32 = 1.;

pub(super) fn draw_value(value: &Value, frame: &mut canvas::Frame, is_picked: bool) {
    match value {
        Value::Pt(Some(pt)) => {
            draw_pt(pt, frame, is_picked);
        }
        Value::Line(Some(line)) => {
            draw_line(line, frame);
        }
        Value::Circ(Some(circ)) => {
            draw_circ(circ, frame);
        }
        _ => {}
    }
}

fn draw_pt(pt: &Pt, frame: &mut canvas::Frame, is_picked: bool) {
    let path = Path::circle(pt_to_point(pt), POINT_WIDTH);
    frame.fill(
        &path,
        canvas::Fill {
            style: canvas::Style::Solid(if is_picked {
                Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }
            } else {
                Color::BLACK
            }),
            ..Default::default()
        },
    );
}

fn draw_line(Line { p1, p2 }: &Line, frame: &mut canvas::Frame) {
    let path = Path::line(pt_to_point(p1), pt_to_point(p2));
    frame.stroke(&path, canvas::Stroke::default().with_width(LINE_WIDTH));
}

fn draw_circ(Circ { o, r }: &Circ, frame: &mut canvas::Frame) {
    let path = Path::circle(pt_to_point(o), *r as f32);
    frame.stroke(&path, canvas::Stroke::default().with_width(LINE_WIDTH));
}
