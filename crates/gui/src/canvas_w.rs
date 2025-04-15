use std::collections::HashMap;

use iced::{
    mouse::Cursor,
    widget::canvas::{self, Path},
    Element,
    Length::Fill,
    Point, Rectangle, Renderer, Theme,
};
use types::core::{Circ, Ident, Line, Pt, Value};

#[derive(Debug, Clone)]
pub enum Msg {}

pub fn view(vars: &HashMap<Ident, Value>) -> Element<Msg> {
    canvas::Canvas::new(Program { vars })
        .width(Fill)
        .height(Fill)
        .into()
}

#[derive(Debug)]
pub struct Program<'a> {
    vars: &'a HashMap<Ident, Value>,
}

#[derive(Debug, Default)]
pub struct ProgramState {}

impl canvas::Program<Msg> for Program<'_> {
    type State = ProgramState;

    fn draw(
        &self,
        _state: &ProgramState,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        for val in self.vars.values() {
            val.draw(&mut frame);
        }

        vec![frame.into_geometry()]
    }
}

static POINT_WIDTH: f32 = 5.;
static LINE_WIDTH: f32 = 1.;

/// Converts [`Pt`] to [`Point`] with the same coordinates.
fn p(pt: &Pt) -> Point {
    Point {
        x: pt.x as f32,
        y: pt.y as f32,
    }
}

trait Draw {
    fn draw(&self, frame: &mut canvas::Frame);
}

impl Draw for Pt {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::circle(p(self), POINT_WIDTH);
        frame.fill(&path, canvas::Fill::default());
    }
}

impl Draw for Line {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::line(p(&self.p1), p(&self.p2));
        frame.stroke(&path, canvas::Stroke::default().with_width(LINE_WIDTH));
    }
}

impl Draw for Circ {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::circle(p(&self.o), self.r as f32);
        frame.stroke(&path, canvas::Stroke::default().with_width(LINE_WIDTH));
    }
}

impl Draw for Value {
    fn draw(&self, frame: &mut canvas::Frame) {
        match self {
            Value::Pt(pt) => {
                if let Some(pt) = pt {
                    pt.draw(frame);
                }
            }
            Value::Line(line) => {
                if let Some(line) = line {
                    line.draw(frame);
                }
            }
            Value::Circ(circ) => {
                if let Some(circ) = circ {
                    circ.draw(frame);
                }
            }
            Value::Bool(_) | Value::Int(_) | Value::Real(_) | Value::Str(_) => {}
        }
    }
}
