use std::collections::HashMap;

use client::Client;
use iced::{widget::row, Element, Length::Fill, Task};
use types::core::Value;

use crate::{command_w, variable_w};

#[derive(Debug)]
pub struct State {
    command_w: command_w::State,
    vars: HashMap<String, Value>,
}

impl State {
    pub fn new(client: Client) -> Self {
        Self {
            command_w: command_w::State::new(client),
            vars: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    CommandWMsg(command_w::Msg),
}

impl State {
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
        }
    }
}
