use std::collections::HashMap;

use super::program::Program;
use client::Client;
use iced::{widget::canvas, Element, Length::Fill, Task};
use types::{
    core::{Ident, Pt, Value, ValueType},
    lang::{Definition, Expr, ValueDefinition},
};

use crate::{helpers::perform_or_status, mode_selector_w::Mode, status_bar_w::StatusMessage};

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
        Msg::CreatePoint(name, pt) => {
            perform_or_status!({
                let client = client.clone();
                async move {
                    client
                        .define_one(Definition::ValueDefinition(ValueDefinition {
                            name,
                            value_type: Some(ValueType::Pt),
                            body: Expr::Value(pt.into()),
                        }))
                        .await
                }
            })
        }
        Msg::MovePoint(name, pt) => perform_or_status!({
            let client = client.clone();
            async move { client.set(name, Expr::Value(pt.into())).await }
        }),
        Msg::Delete(name) => perform_or_status!({
            let client = client.clone();
            async move { client.rm(name).await }
        }),
        Msg::SetStatusMessage(_) => {
            unreachable!("should have been processed in parent widget")
        }
        Msg::None => Task::none(),
    }
}
