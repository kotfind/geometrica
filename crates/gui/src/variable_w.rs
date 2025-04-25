use std::collections::HashMap;

use client::Client;
use iced::{
    font::Weight,
    widget::{column, text, text_input},
    Element, Font,
    Length::Fill,
    Task,
};
use iced_aw::{grid, grid_row};
use itertools::Itertools;
use types::core::{Ident, Value};

use crate::status_bar_w::StatusMessage;

#[derive(Debug)]
pub struct State {
    new_def_text: String,
}

#[derive(Debug, Clone)]
pub enum Msg {
    NewDefTextChanged(String),
    DefineNew,
    SetStatusMessage(StatusMessage),
    None,
}

impl State {
    pub fn new() -> Self {
        Self {
            new_def_text: "".to_string(),
        }
    }

    pub fn view<'a>(&'a self, vars: &'a HashMap<Ident, Value>) -> Element<'a, Msg> {
        column![self.view_variables(vars), self.view_new_def()]
            .spacing(5)
            .padding(5)
            .width(Fill)
            .into()
    }

    fn view_variables<'a>(&'a self, vars: &'a HashMap<Ident, Value>) -> Element<'a, Msg> {
        let header = ["Name", "Value"]
            .into_iter()
            .map(|txt| {
                text(txt).font(Font {
                    weight: Weight::Bold,
                    ..Default::default()
                })
            })
            .collect_vec();

        let header = grid_row(header);

        let body = vars
            .iter()
            .sorted_by(|(var_name_1, _), (var_name_2, _)| Ord::cmp(&var_name_1.0, &var_name_2.0))
            .map(|(var_name, var_val)| grid_row![text(&var_name.0), text(var_val.to_string())]);

        let rows = std::iter::once(header).chain(body).collect_vec();

        grid(rows).column_width(Fill).width(Fill).spacing(5).into()
    }

    fn view_new_def(&self) -> Element<Msg> {
        text_input("new_var = 1.0", &self.new_def_text)
            .on_input(Msg::NewDefTextChanged)
            .on_submit(Msg::DefineNew)
            .width(Fill)
            .into()
    }

    pub fn update(&mut self, msg: Msg, client: Client) -> Task<Msg> {
        match msg {
            Msg::NewDefTextChanged(text) => {
                self.new_def_text = text;
                Task::none()
            }
            Msg::DefineNew => {
                let task =
                    Task::perform(Self::define_new(client, self.new_def_text.clone()), |res| {
                        res.map_or_else(
                            |e| Msg::SetStatusMessage(StatusMessage::error(format!("{e:#}"))),
                            |_| Msg::None,
                        )
                    });
                self.new_def_text = "".to_string();
                task
            }
            Msg::SetStatusMessage(_) => {
                unreachable!("should have been processed in parent widget")
            }
            Msg::None => Task::none(),
        }
    }

    async fn define_new(client: Client, def: String) -> anyhow::Result<()> {
        // FIXME: this method technically allows defining functions,
        // which is not the intended behaviour
        client.define_one(def.trim()).await
    }
}
