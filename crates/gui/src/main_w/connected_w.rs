use std::collections::HashMap;

use client::Client;
use iced::{widget::row, Element, Length::Fill, Task};
use types::core::{Ident, Value};

use crate::{command_w, variable_w};

#[derive(Debug)]
pub struct State {
    command_w: command_w::State,
    vars: HashMap<Ident, Value>,
    client: Client,
}

#[derive(Debug, Clone)]
pub enum Msg {
    CommandWMsg(command_w::Msg),
    GotVars(HashMap<Ident, Value>),
}

impl State {
    pub fn run_with(client: Client) -> (Self, Task<Msg>) {
        (
            Self {
                command_w: command_w::State::new(client.clone()),
                vars: Default::default(),
                client: client.clone(),
            },
            Task::perform(Self::fetch_vars(client), Msg::GotVars),
        )
    }

    pub fn view(&self) -> Element<Msg> {
        row![
            variable_w::view(&self.vars),
            self.command_w.view().map(Msg::CommandWMsg)
        ]
        .width(Fill)
        .height(Fill)
        .into()
    }

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::CommandWMsg(msg) => self.command_w.update(msg).map(Msg::CommandWMsg),
            Msg::GotVars(vars) => {
                self.vars = vars;
                Task::perform(Self::fetch_vars(self.client.clone()), Msg::GotVars)
            }
        }
    }

    async fn fetch_vars(client: Client) -> HashMap<Ident, Value> {
        // FiXME: polling w/o timeout
        // FIXME: unwrap
        client.get_all_items().await.unwrap()
    }
}
