use anyhow::Context;
use std::collections::HashMap;

use super::{program::Program, transform::Transformation};
use client::Client;
use iced::{
    widget::{
        canvas::{self},
        responsive,
    },
    Element,
    Length::Fill,
    Task,
};
use types::{
    core::{Circ, Ident, Line, Pt, Value, ValueType},
    lang::{Definition, Expr, ValueDefinition},
};

use crate::{helpers::perform_or_status, mode::Mode, status_bar_w::StatusMessage};

#[derive(Debug, Clone)]
pub enum Msg {
    SetStatusMessage(StatusMessage),
    None,

    SetMode(Mode),
    SetCustomTransformation(Transformation),

    CreatePoint(Ident, Pt),
    MovePoint(Ident, Pt),
    Delete(Ident),
    PickFunctionArg(Ident),
}

#[derive(Debug)]
pub struct State {
    custom_transformation: Transformation,
}

impl State {
    pub fn new() -> Self {
        Self {
            custom_transformation: Transformation::identity(),
        }
    }

    pub fn view<'a>(&'a self, vars: &'a HashMap<Ident, Value>, mode: &'a Mode) -> Element<'a, Msg> {
        responsive(|size| {
            let w = size.width as f64;
            let h = size.height as f64;
            let unify_transformation =
                Transformation::from_bounds(Pt { x: 0.0, y: 0.0 }, Pt { x: w, y: h }, true);

            canvas::Canvas::new(Program {
                vars,
                mode,
                unify_transformation,
                custom_transformation: self.custom_transformation,
            })
            .width(Fill)
            .height(Fill)
            .into()
        })
        .into()
    }

    pub fn update<'a>(
        &mut self,
        msg: Msg,
        client: Client,
        mode: &'a Mode,
        vars: &'a HashMap<Ident, Value>,
    ) -> Task<Msg> {
        match msg {
            Msg::SetStatusMessage(_) | Msg::SetMode(_) => {
                unreachable!("should have been processed in parent widget")
            }
            Msg::None => Task::none(),
            Msg::CreatePoint(name, pt) => {
                perform_or_status!(async move {
                    client
                        .define_one(Definition::ValueDefinition(ValueDefinition {
                            name,
                            value_type: Some(ValueType::Pt),
                            body: Expr::Value(pt.into()),
                        }))
                        .await
                })
            }
            Msg::MovePoint(name, pt) => {
                perform_or_status!(async move { client.set(name, Expr::Value(pt.into())).await })
            }
            Msg::Delete(name) => perform_or_status!({
                let client = client.clone();
                async move { client.rm(name).await }
            }),
            Msg::PickFunctionArg(arg) => {
                let Mode::Function(func_mode) = mode else {
                    println!("WARN: expected function mode");
                    return Task::none();
                };
                let mut func_mode = func_mode.clone();
                let vars = vars.clone();

                perform_or_status!(
                    async move {
                        func_mode
                            .add_arg(arg, client, &vars)
                            .await
                            .context("add_arg failed")?;
                        Ok(Mode::Function(func_mode))
                    },
                    Msg::SetMode
                )
            }
            Msg::SetCustomTransformation(custom_transformation) => {
                self.custom_transformation = custom_transformation;
                Task::none()
            }
        }
    }

    /// Sets [Self::custom_transformation] to [Transformation::identity()]
    pub fn set_identity_transformation(&mut self) {
        self.custom_transformation = Transformation::identity();
    }

    /// Resets custom transformation, so that all objects fit.
    ///
    /// Notes:
    ///
    /// * Does nothing if there are no drawable objects.
    /// * Fixes offset and sets zoom to 1 if only available object is a single point.
    pub fn set_fit_all_transformation(&mut self, vars: &HashMap<Ident, Value>) {
        // The bounds are: (min_x, min_y, max_x, max_y)
        let bounds = vars
            .values()
            .filter_map(|value| match value {
                Value::Pt(Some(Pt { x, y })) => Some((*x, *y, *x, *y)),
                Value::Line(Some(Line {
                    p1: Pt { x: x1, y: y1 },
                    p2: Pt { x: x2, y: y2 },
                })) => Some((x1.min(*x2), y1.min(*y2), x1.max(*x2), y1.max(*y2))),
                Value::Circ(Some(Circ { o: Pt { x, y }, r })) => Some((x - r, y - r, x + r, x + r)),
                _ => None,
            })
            .reduce(
                |(min_x_1, min_y_1, max_x_1, max_y_1), (min_x_2, min_y_2, max_x_2, max_y_2)| {
                    (
                        min_x_1.min(min_x_2),
                        min_y_1.min(min_y_2),
                        max_x_1.max(max_x_2),
                        max_y_1.max(max_y_2),
                    )
                },
            );

        let Some((min_x, min_y, max_x, max_y)) = bounds else {
            return;
        };

        let mut t = Transformation::from_bounds(
            Pt { x: min_x, y: min_y },
            Pt { x: max_x, y: max_y },
            false,
        )
        .inverse();

        // Some padding
        t.zoom /= 1.1;

        self.custom_transformation = t;
    }
}
