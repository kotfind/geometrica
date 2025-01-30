use std::collections::HashMap;

use client::Client;
use iced::{widget::row, Element, Length::Fill, Task};
use types::core::Value;

mod command_w;
mod variable_w;

#[derive(Debug, Default)]
struct State {
    command_w: command_w::State,
    vars: HashMap<String, Value>,

    // Would be None if not connected
    client: Option<Client>,
}

#[derive(Debug, Clone)]
enum Msg {
    CommandWMsg(command_w::Msg),
    Connected(Client),
}

impl State {
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
            Msg::Connected(client) => {
                self.client = Some(client);
            }
        }
    }

    async fn connect() -> Client {
        // TODO: settings
        // TODO: fix unwrap
        Client::from(Default::default()).await.unwrap()
    }
}

fn main() -> anyhow::Result<()> {
    iced::application("Geometrica Gui", State::update, State::view).run_with(|| {
        (
            Default::default(),
            Task::perform(State::connect(), Msg::Connected),
        )
    })?;

    Ok(())
}
