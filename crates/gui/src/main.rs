use std::collections::HashMap;

use iced::{widget::row, Element, Length::Fill};
use types::core::{Pt, Value};

mod command_w;
mod variable_w;

#[derive(Debug)]
struct State {
    command_w: command_w::State,
    vars: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
enum Msg {
    CommandWMsg(command_w::Msg),
}

impl State {
    fn new() -> Self {
        Self {
            command_w: Default::default(),
            vars: HashMap::from([
                // FIXME: Dummy
                ("x".to_string(), 1.0.into()),
                ("y".to_string(), 2.0.into()),
                ("z".to_string(), Pt { x: 3.0, y: 4.0 }.into()),
            ]),
        }
    }

    fn view(&self) -> Element<Msg> {
        row![
            variable_w::view(&self.vars),
            self.command_w.view().map(Msg::CommandWMsg)
        ]
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::CommandWMsg(msg) => self.command_w.update(msg),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

fn main() -> anyhow::Result<()> {
    iced::run("Geometrica Gui", State::update, State::view)?;
    Ok(())
}
