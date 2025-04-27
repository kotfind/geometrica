use std::cmp::Ordering;
use std::collections::HashMap;

use crate::helpers::new_object_name_with_type;
use crate::mode::Mode;

use super::draw::draw_value;
use super::helpers::point_to_pt;
use super::widget::Msg;
use iced::Color;
use iced::{
    mouse::{self, Cursor},
    widget::canvas,
    Rectangle, Renderer, Theme,
};
use itertools::Itertools;
use types::core::{Ident, Pt, Value, ValueType};

#[derive(Debug)]
pub(super) struct Program<'a> {
    pub(super) vars: &'a HashMap<Ident, Value>,
    pub(super) mode: &'a Mode,
}

#[derive(Debug, Default)]
pub(super) struct ProgramState {
    picked_pt: Option<Ident>,
    highlighted_item: Option<Ident>,
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
            let color = match &self.mode {
                _ if state
                    .highlighted_item
                    .as_ref()
                    .is_some_and(|highlighted| highlighted == var_name) =>
                {
                    Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }
                }

                Mode::Function(func_mode)
                    if func_mode
                        .selected_args()
                        .iter()
                        .any(|name| name == var_name) =>
                {
                    Color {
                        r: 0.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }
                }

                _ => Color::BLACK,
            };

            draw_value(var_value, &mut frame, color);
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
        match event {
            canvas::Event::Mouse(mouse_event) => {
                Self::handle_mouse_event(self, state, mouse_event, bounds, cursor)
            }
            _ => (canvas::event::Status::Ignored, None),
        }
    }
}

impl Program<'_> {
    fn handle_mouse_event(
        &self,
        state: &mut <Self as canvas::Program<Msg>>::State,
        mouse_event: mouse::Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (canvas::event::Status, Option<Msg>) {
        use canvas::event::Status::{Captured, Ignored};
        use mouse::{
            Button::Left,
            Event::{ButtonPressed, ButtonReleased},
        };

        let Some(cursor_pt) = cursor.position_in(bounds) else {
            return (Ignored, None);
        };

        let cursor_pt = point_to_pt(&cursor_pt);

        if let ButtonReleased(_) = &mouse_event {
            state.picked_pt = None;
        }

        match &self.mode {
            Mode::CreatePoint => {
                state.highlighted_item = None;

                if let ButtonPressed(Left) = mouse_event {
                    (
                        Captured,
                        Some(Msg::CreatePoint(
                            new_object_name_with_type(Some(ValueType::Pt), self.vars.keys()),
                            cursor_pt,
                        )),
                    )
                } else {
                    (Ignored, None)
                }
            }

            Mode::Modify => {
                let cursor_item =
                    self.get_cursor_item(cursor_pt, |v| v.value_type() == ValueType::Pt);
                let cursor_item_name = cursor_item.map(|(name, _value)| name);
                state.highlighted_item = state.picked_pt.clone().or(cursor_item_name.clone());

                if let (ButtonPressed(Left), Some(cursor_item_name)) =
                    (mouse_event, cursor_item_name)
                {
                    state.picked_pt = Some(cursor_item_name);
                }

                if let Some(picked_pt) = &state.picked_pt {
                    return (Captured, Some(Msg::MovePoint(picked_pt.clone(), cursor_pt)));
                }

                (Ignored, None)
            }

            Mode::Delete => {
                let cursor_item = self.get_cursor_item(cursor_pt, |_| true);
                let cursor_item_name = cursor_item.map(|(name, _value)| name);
                state.highlighted_item = cursor_item_name.clone();

                if let (ButtonPressed(Left), Some(cursor_item_name)) =
                    (mouse_event, cursor_item_name)
                {
                    (Captured, Some(Msg::Delete(cursor_item_name)))
                } else {
                    (Ignored, None)
                }
            }

            Mode::Function(func_mode) => {
                let cursor_item = self
                    .get_cursor_item(cursor_pt, |v| v.value_type() == func_mode.next_arg_type());
                let cursor_item_name = cursor_item.map(|(name, _value)| name);
                state.highlighted_item = cursor_item_name.clone();

                if let (ButtonPressed(Left), Some(cursor_item_name)) =
                    (mouse_event, cursor_item_name)
                {
                    (Captured, Some(Msg::PickFunctionArg(cursor_item_name)))
                } else {
                    (Ignored, None)
                }
            }

            Mode::Transform => todo!(),
        }
    }

    fn get_cursor_item(
        &self,
        cursor_pos: Pt,
        cond: impl Fn(&Value) -> bool,
    ) -> Option<(Ident, Value)> {
        static CLICK_DIST: f64 = 10.0;

        struct WithDist<'a> {
            dist: f64,
            name: &'a Ident,
            value: &'a Value,
        }

        self.vars
            .iter()
            .filter(|(_, value)| cond(value))
            .filter_map(|(name, value)| {
                Some(WithDist {
                    dist: match value {
                        Value::Pt(Some(pt)) => pt.dist(cursor_pos),
                        Value::Line(Some(line)) => line.dist(cursor_pos),
                        Value::Circ(Some(circ)) => circ.dist(cursor_pos),
                        _ => return None,
                    },
                    name,
                    value,
                })
            })
            .filter(|item| item.dist < CLICK_DIST)
            .sorted_by(|lhs, rhs| match (lhs.value, rhs.value) {
                (Value::Pt(_), Value::Line(_)) | (Value::Pt(_), Value::Circ(_)) => Ordering::Less,

                _ => lhs.dist.total_cmp(&rhs.dist),
            })
            .next()
            .map(|item| (item.name.clone(), item.value.clone()))
    }
}
