use client::Client;
use iced::{Element, Task};

mod connected_w;
mod disconnected_w;

#[derive(Debug)]
pub enum State {
    Connected(connected_w::State),
    Disconnected(disconnected_w::State),
}

impl Default for State {
    fn default() -> Self {
        Self::Disconnected(Default::default())
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    ConnectedMsg(connected_w::Msg),
    DisconnectedMsg(disconnected_w::Msg),
}

impl State {
    pub const TITLE: &str = "Geometrica GUI";

    pub fn run_with() -> (Self, Task<Msg>) {
        (
            Default::default(),
            Task::perform(State::connect(), |client| {
                Msg::DisconnectedMsg(disconnected_w::Msg::Connected(client))
            } /* FIXME */),
        )
    }

    pub fn view(&self) -> Element<Msg> {
        match self {
            State::Connected(state) => state.view().map(Msg::ConnectedMsg),
            State::Disconnected(state) => state.view().map(Msg::DisconnectedMsg),
        }
    }

    pub fn update(&mut self, msg: Msg) {
        match (&mut *self, msg) {
            (State::Connected(state), Msg::ConnectedMsg(msg)) => {
                state.update(msg);
            }
            (State::Disconnected(state), Msg::DisconnectedMsg(msg)) =>
            {
                #[allow(irrefutable_let_patterns)]
                if let disconnected_w::Msg::Connected(client) = msg {
                    *self = State::Connected(connected_w::State::new(client));
                } else {
                    state.update(msg);
                }
            }
            _ => {
                println!("WARN: Unexpected message type for current state");
            }
        }
    }

    async fn connect() -> Client {
        // TODO: settings
        // TODO: fix unwrap
        Client::from(Default::default()).await.unwrap()
    }
}
