use std::cmp::Ordering;
use std::collections::HashMap;

use crate::helpers::new_object_name_with_type;
use crate::mode::Mode;

use super::draw::draw_value;
use super::helpers::point_to_pt;
use super::transform::Transformation;
use super::widget::Msg;
use iced::mouse::ScrollDelta;
use iced::Color;
use iced::{
    mouse::{self, Cursor},
    widget::canvas,
    Rectangle, Renderer, Theme,
};
use itertools::Itertools;
use types::core::{Ident, Pt, Value, ValueType};

static ZOOM_SPEED: f64 = 0.5;

#[derive(Debug)]
pub(super) struct Program<'a> {
    pub(super) vars: &'a HashMap<Ident, Value>,
    pub(super) mode: &'a Mode,

    /// A transformation to abstract from screen size.
    ///
    /// Maps square
    ///     (-1, -1),
    ///     ( 1, -1),
    ///     ( 1,  1),
    ///     (-1,  1)
    /// to canvas screen.
    ///
    /// Is applied AFTER [Self::custom_transformation].
    pub(super) unify_transformation: Transformation,

    /// A transformation, defined by user.
    ///
    /// It is applied BEFORE [Self::unify_transformation].
    pub(super) custom_transformation: Transformation,
}

#[derive(Debug, Default)]
pub(super) struct ProgramState {
    highlighted_item: Option<Ident>,

    /// For [Mode::Modify] only
    picked_pt: Option<Ident>,

    /// For [Mode::Transform] only
    ///
    /// Is Some if and only if Left mouse button is pressed.
    last_cursor_pos_screen: Option<Pt>,
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
        let t = self.complete_transformation();

        for (var_name, var_value_real) in self.vars {
            let Some(var_value_screen) = t.transform_value(var_value_real) else {
                continue;
            };

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

            draw_value(&var_value_screen, &mut frame, color);
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
            Event::{ButtonPressed, ButtonReleased, CursorLeft, WheelScrolled},
        };

        let Some(cursor_pos) = cursor.position_in(bounds) else {
            return (Ignored, None);
        };

        let cursor_pos_screen = point_to_pt(&cursor_pos);

        let cursor_pos_real = self
            .complete_transformation()
            .inverse()
            .transform_pt(cursor_pos_screen);

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
                            cursor_pos_real,
                        )),
                    )
                } else {
                    (Ignored, None)
                }
            }

            Mode::Modify => {
                let cursor_item =
                    self.get_cursor_item(cursor_pos_screen, |v| v.value_type() == ValueType::Pt);
                let cursor_item_name = cursor_item.map(|(name, _value)| name);
                state.highlighted_item = state.picked_pt.clone().or(cursor_item_name.clone());

                if let (ButtonPressed(Left), Some(cursor_item_name)) =
                    (mouse_event, cursor_item_name)
                {
                    state.picked_pt = Some(cursor_item_name);
                }

                if let Some(picked_pt) = &state.picked_pt {
                    return (
                        Captured,
                        Some(Msg::MovePoint(picked_pt.clone(), cursor_pos_real)),
                    );
                }

                (Ignored, None)
            }

            Mode::Delete => {
                let cursor_item = self.get_cursor_item(cursor_pos_screen, |_| true);
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
                let cursor_item = self.get_cursor_item(cursor_pos_screen, |v| {
                    v.value_type() == func_mode.next_arg_type()
                });
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

            Mode::Transform => {
                let mut status = Ignored;

                let t_orig = self.custom_transformation;
                let mut t = t_orig;

                match mouse_event {
                    CursorLeft | ButtonReleased(_) => {
                        status = Captured;
                        state.last_cursor_pos_screen = None;
                    }
                    ButtonPressed(Left) => {
                        status = Captured;
                        state.last_cursor_pos_screen = Some(cursor_pos_screen);
                    }
                    WheelScrolled {
                        delta: ScrollDelta::Lines { y: y_delta, .. },
                    } => {
                        status = Captured;
                        t.zoom *= 2f64.powf(y_delta as f64 * ZOOM_SPEED);
                    }
                    // TODO: WheelScrolled { delta: ScrollDelta::Pixels { .. } }
                    _ => {}
                };

                if let Some(last_cursor_pos_screen) = state.last_cursor_pos_screen {
                    let uni_inv = self.unify_transformation.inverse();
                    let delta = uni_inv.transform_pt(cursor_pos_screen)
                        - uni_inv.transform_pt(last_cursor_pos_screen);
                    t.offset = t.offset + delta;
                    state.last_cursor_pos_screen = Some(cursor_pos_screen);
                }

                (
                    status,
                    (t != t_orig).then_some(Msg::SetCustomTransformation(t)),
                )
            }
        }
    }

    fn get_cursor_item(
        &self,
        cursor_pos_screen: Pt,
        cond: impl Fn(&Value) -> bool,
    ) -> Option<(Ident, Value)> {
        static CLICK_DIST: f64 = 10.0;

        let t = self.complete_transformation();

        struct WithDist<'a> {
            dist: f64,
            name: &'a Ident,
            value: Value,
        }

        self.vars
            .iter()
            .filter_map(|(name, value_real)| Some((name, t.transform_value(value_real)?)))
            .filter(|(_, value)| cond(value))
            .filter_map(|(name, value)| {
                Some(WithDist {
                    dist: match value {
                        Value::Pt(Some(pt)) => pt.dist(cursor_pos_screen),
                        Value::Line(Some(line)) => line.dist(cursor_pos_screen),
                        Value::Circ(Some(circ)) => circ.dist(cursor_pos_screen),
                        _ => return None,
                    },
                    name,
                    value,
                })
            })
            .filter(|item| item.dist < CLICK_DIST)
            .sorted_by(|lhs, rhs| match (&lhs.value, &rhs.value) {
                (Value::Pt(_), Value::Line(_)) | (Value::Pt(_), Value::Circ(_)) => Ordering::Less,

                _ => lhs.dist.total_cmp(&rhs.dist),
            })
            .next()
            .map(|item| (item.name.clone(), item.value.clone()))
    }

    /// Converts **real** coordinates into **screen** coordinates.
    fn complete_transformation(&self) -> Transformation {
        self.custom_transformation.chain(&self.unify_transformation)
    }
}
