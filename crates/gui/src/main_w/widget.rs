use iced::{widget::column, Element, Length::Fill, Task};

use crate::status_bar_w::{self, StatusMessage};

use super::{connected_w, disconnected_w};

#[derive(Debug)]
pub struct State {
    status_bar_w: status_bar_w::State,
    kind: StateKind,
}

impl Default for State {
    fn default() -> Self {
        Self {
            status_bar_w: Default::default(),
            kind: StateKind::Disconnected(Default::default()),
        }
    }
}

#[derive(Debug)]
pub enum StateKind {
    Connected(connected_w::State),
    Disconnected(disconnected_w::State),
}

#[derive(Debug, Clone)]
pub enum Msg {
    ConnectedMsg(connected_w::Msg),
    DisconnectedMsg(disconnected_w::Msg),
    StatusBarWMsg(status_bar_w::Msg),
}

impl State {
    pub const TITLE: &str = "Geometrica GUI";

    pub fn view(&self) -> Element<Msg> {
        let master = match &self.kind {
            StateKind::Connected(state) => state.view().map(Msg::ConnectedMsg),
            StateKind::Disconnected(state) => state.view().map(Msg::DisconnectedMsg),
        };

        column![master, self.status_bar_w.view().map(Msg::StatusBarWMsg)]
            .width(Fill)
            .into()
    }

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        match (&mut self.kind, msg) {
            (StateKind::Connected(state), Msg::ConnectedMsg(msg)) => match msg {
                connected_w::Msg::SetStatusMessage(message) => self.set_status_message(message),
                connected_w::Msg::Disconnected => {
                    self.kind = StateKind::Disconnected(Default::default());
                    Task::none()
                }
                _ => state.update(msg).map(Msg::ConnectedMsg),
            },
            (StateKind::Disconnected(state), Msg::DisconnectedMsg(msg)) => match msg {
                disconnected_w::Msg::Connected(client) => {
                    let (connected, task) = connected_w::State::run_with(client);
                    self.kind = StateKind::Connected(connected);
                    task.map(Msg::ConnectedMsg)
                }
                disconnected_w::Msg::SetStatusMessage(message) => self.set_status_message(message),
                _ => state.update(msg).map(Msg::DisconnectedMsg),
            },
            (_, Msg::StatusBarWMsg(msg)) => self.status_bar_w.update(msg).map(Msg::StatusBarWMsg),
            _ => {
                eprintln!("WARN: Unexpected message type for current state");
                Task::none()
            }
        }
    }

    fn set_status_message(&mut self, message: StatusMessage) -> Task<Msg> {
        eprintln!("{message}");

        self.status_bar_w
            .update(status_bar_w::Msg::SetMessage(message))
            .map(Msg::StatusBarWMsg)
    }
}
