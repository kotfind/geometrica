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
    core::{Ident, Pt, Value, ValueType},
    lang::{Definition, Expr, ValueDefinition},
};

use crate::{helpers::perform_or_status, mode::Mode, status_bar_w::StatusMessage};

#[derive(Debug, Clone)]
pub enum Msg {
    SetStatusMessage(StatusMessage),
    None,

    SetMode(Mode),

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
            canvas::Canvas::new(Program {
                vars,
                mode,
                unify_transformation: Transformation::from_bounds(
                    Pt { x: 0.0, y: 0.0 },
                    Pt {
                        x: size.width as f64,
                        y: size.height as f64,
                    },
                ),
                custom_transformation: self.custom_transformation,
            })
            .width(Fill)
            .height(Fill)
            .into()
        })
        .into()
    }

    pub fn update<'a>(
        &self,
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
        }
    }
}
