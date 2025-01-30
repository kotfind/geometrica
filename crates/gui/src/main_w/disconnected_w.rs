use client::Client;
use iced::{widget::text, Element};

#[derive(Debug, Default)]
pub struct State {}

#[derive(Debug, Clone)]
pub enum Msg {
    Connected(Client),
}

impl State {
    pub fn view(&self) -> Element<Msg> {
        text("Not connected").into()
    }

    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Connected(_) => {
                unreachable!("This message should have been processed in a parent widget, and not forwarded here");
            }
        }
    }
}
