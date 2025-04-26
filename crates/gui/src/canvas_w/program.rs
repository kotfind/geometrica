use std::collections::HashMap;

use super::draw::{draw_value, LINE_WIDTH, POINT_WIDTH};
use super::helpers::point_to_pt;
use super::widget::Msg;
use iced::Point;
use iced::{
    mouse::{self, Cursor},
    widget::canvas,
    Rectangle, Renderer, Theme,
};
use itertools::Itertools;
use types::core::{Ident, Value, ValueType};

use super::helpers::new_point_name;
use crate::mode_selector_w::Mode;

#[derive(Debug)]
pub(super) struct Program<'a> {
    pub(super) vars: &'a HashMap<Ident, Value>,
    pub(super) mode: &'a Mode,
}

#[derive(Debug, Default)]
pub(super) struct ProgramState {
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

                let cursor_pt = point_to_pt(&cursor_pos);
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
                        Mode::Function => {
                            /* TODO: highlight items */
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

static POINT_CLICK_AREA_WIDTH: f64 = POINT_WIDTH as f64 * 2.0;
static LINE_CLICK_AREA_WIDTH: f64 = LINE_WIDTH as f64 * 2.0;

fn is_under_cursor(cursor_pos: Point, value: &Value) -> bool {
    let cursor_pos = point_to_pt(&cursor_pos);
    // XXX: transformation
    match value {
        Value::Pt(Some(pt)) => pt.dist(cursor_pos) < POINT_CLICK_AREA_WIDTH,
        Value::Line(Some(line)) => line.dist(cursor_pos) < LINE_CLICK_AREA_WIDTH,
        Value::Circ(Some(circ)) => circ.dist(cursor_pos) < LINE_CLICK_AREA_WIDTH,
        _ => false,
    }
}
