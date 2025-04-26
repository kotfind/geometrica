use std::collections::{HashMap, HashSet};

use client::Client;
use iced::{
    advanced::mouse,
    mouse::Cursor,
    widget::canvas::{self, Path},
    Color, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Task, Theme,
};
use itertools::Itertools;
use types::{
    core::{Circ, Ident, Line, Pt, Value, ValueType},
    lang::{Definition, Expr, ValueDefinition},
};

use crate::{mode_selector_w::Mode, status_bar_w::StatusMessage};

#[derive(Debug, Clone)]
pub enum Msg {
    SetStatusMessage(StatusMessage),
    None,

    CreatePoint(Ident, Pt),
    MovePoint(Ident, Pt),
    Delete(Ident),
}

pub fn view<'a>(vars: &'a HashMap<Ident, Value>, mode: &'a Mode) -> Element<'a, Msg> {
    canvas::Canvas::new(Program { vars, mode })
        .width(Fill)
        .height(Fill)
        .into()
}

pub fn update(msg: Msg, client: Client) -> Task<Msg> {
    match msg {
        Msg::CreatePoint(name, pt) => Task::perform(create_point(client, name, pt), |res| {
            res.map_or_else(
                |e| Msg::SetStatusMessage(StatusMessage::error(format!("{e:#}"))),
                |_| Msg::None,
            )
        }),
        Msg::MovePoint(name, pt) => Task::perform(move_point(client, name, pt), |res| {
            res.map_or_else(
                |e| Msg::SetStatusMessage(StatusMessage::error(format!("{e:#}"))),
                |_| Msg::None,
            )
        }),
        Msg::Delete(name) => Task::perform(delete(client, name), |res| {
            res.map_or_else(
                |e| Msg::SetStatusMessage(StatusMessage::error(format!("{e:#}"))),
                |_| Msg::None,
            )
        }),
        Msg::SetStatusMessage(_) => {
            unreachable!("should have been processed in parent widget")
        }
        Msg::None => Task::none(),
    }
}

#[derive(Debug)]
pub struct Program<'a> {
    vars: &'a HashMap<Ident, Value>,
    mode: &'a Mode,
}

#[derive(Debug, Default)]
pub struct ProgramState {
    picked_pt: Option<Ident>,
}

impl canvas::Program<Msg> for Program<'_> {
    type State = ProgramState;

    fn draw(
        &self,
        state: &ProgramState,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        for (var_name, var_value) in self.vars {
            let is_picked = state
                .picked_pt
                .as_ref()
                .is_some_and(|picked| picked == var_name);

            draw_value(var_value, &mut frame, is_picked);
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (canvas::event::Status, Option<Msg>) {
        use canvas::{
            event::Status::{Captured, Ignored},
            Event::Mouse,
        };
        use mouse::{
            Button::Left,
            Event::{ButtonPressed, ButtonReleased, CursorMoved},
        };

        match event {
            Mouse(mouse_event) => {
                let Some(cursor_pos) = cursor.position_in(bounds) else {
                    return (Ignored, None);
                };

                let cursor_pt = p_(&cursor_pos);
                let cursor_items = self
                    .vars
                    .iter()
                    .filter(|(_name, value)| is_under_cursor(cursor_pos, value))
                    .map(|(name, value)| (name.clone(), value.clone()))
                    .collect_vec();

                match mouse_event {
                    ButtonPressed(Left) => match &self.mode {
                        Mode::CreatePoint => (
                            Captured,
                            Some(Msg::CreatePoint(
                                new_point_name(self.vars.keys()),
                                cursor_pt,
                            )),
                        ),
                        Mode::Modify => {
                            if let Some((name, _value)) = cursor_items
                                .into_iter()
                                .find(|(_, value)| value.value_type() == ValueType::Pt)
                            {
                                state.picked_pt = Some(name);
                            }
                            (Captured, None)
                        }
                        Mode::Transform => todo!(),
                        Mode::Delete => (
                            Captured,
                            cursor_items
                                .into_iter()
                                .next()
                                .map(|(name, _value)| Msg::Delete(name)),
                        ),
                        Mode::Function => todo!(),
                    },
                    CursorMoved { position: _ } => match &self.mode {
                        Mode::Modify => {
                            if let Some(picked_pt) = &state.picked_pt {
                                (Captured, Some(Msg::MovePoint(picked_pt.clone(), cursor_pt)))
                            } else {
                                (Ignored, None)
                            }
                        }
                        Mode::Transform => todo!(),
                        Mode::Function =>
                        /* TODO: highlight items */
                        {
                            todo!()
                        }
                        _ => (Ignored, None),
                    },

                    ButtonReleased(_) => {
                        state.picked_pt = None;

                        (Captured, None)
                    }

                    _ => (Ignored, None),
                }
            }
            _ => (Ignored, None),
        }
    }
}

async fn move_point(client: Client, name: Ident, pt: Pt) -> anyhow::Result<()> {
    client.set(name, Expr::Value(pt.into())).await
}

async fn create_point(client: Client, name: Ident, pt: Pt) -> anyhow::Result<()> {
    client
        .define_one(Definition::ValueDefinition(ValueDefinition {
            name,
            value_type: Some(ValueType::Pt),
            body: Expr::Value(pt.into()),
        }))
        .await
}

async fn delete(client: Client, name: Ident) -> anyhow::Result<()> {
    client.rm(name).await
}

fn new_point_name<'a>(names: impl Iterator<Item = &'a Ident>) -> Ident {
    let names: HashSet<_> = names.collect();

    let mut cntr = 1;
    loop {
        let name_candidate = Ident(format!("p{cntr}"));
        if names.contains(&name_candidate) {
            cntr += 1;
        } else {
            return name_candidate;
        }
    }
}

static POINT_CLICK_AREA_WIDTH: f64 = POINT_WIDTH as f64 * 2.0;
static LINE_CLICK_AREA_WIDTH: f64 = POINT_WIDTH as f64 * 2.0;

fn is_under_cursor(cursor_pos: Point, value: &Value) -> bool {
    let cursor_pos = p_(&cursor_pos);
    // XXX: transformation
    match value {
        Value::Pt(Some(pt)) => pt.dist(cursor_pos) < POINT_CLICK_AREA_WIDTH,
        Value::Line(Some(line)) => line.dist(cursor_pos) < LINE_CLICK_AREA_WIDTH,
        Value::Circ(Some(circ)) => circ.dist(cursor_pos) < LINE_CLICK_AREA_WIDTH,
        _ => false,
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

/// Converts [`Point`] to [`Pt`] with the same coordinates.
fn p_(pt: &Point) -> Pt {
    Pt {
        x: pt.x as f64,
        y: pt.y as f64,
    }
}

fn draw_value(value: &Value, frame: &mut canvas::Frame, is_picked: bool) {
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
    let path = Path::circle(p(pt), POINT_WIDTH);
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
    let path = Path::line(p(p1), p(p2));
    frame.stroke(&path, canvas::Stroke::default().with_width(LINE_WIDTH));
}

fn draw_circ(Circ { o, r }: &Circ, frame: &mut canvas::Frame) {
    let path = Path::circle(p(o), *r as f32);
    frame.stroke(&path, canvas::Stroke::default().with_width(LINE_WIDTH));
}
