use anyhow::Context;
use std::collections::HashMap;

use super::program::Program;
use client::Client;
use iced::{widget::canvas, Element, Length::Fill, Task};
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

pub fn view<'a>(vars: &'a HashMap<Ident, Value>, mode: &'a Mode) -> Element<'a, Msg> {
    canvas::Canvas::new(Program { vars, mode })
        .width(Fill)
        .height(Fill)
        .into()
}

pub fn update<'a>(
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
