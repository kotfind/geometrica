use std::time::Duration;

use iced::{
    border::Radius,
    widget::{container, text},
    Border, Color, Element,
    Length::Fill,
    Task, Theme,
};

#[derive(Debug, Clone)]
pub struct StatusMessage {
    kind: StatusMessageKind,
    text: String,
}

impl StatusMessage {
    const MESSAGE_DURATION: Duration = Duration::from_secs(20);

    pub fn new(kind: StatusMessageKind, text: impl ToString) -> Self {
        Self {
            kind,
            text: text.to_string(),
        }
    }

    pub fn error(text: impl ToString) -> Self {
        Self::new(StatusMessageKind::Error, text)
    }

    pub fn warn(text: impl ToString) -> Self {
        Self::new(StatusMessageKind::Warn, text)
    }

    pub fn info(text: impl ToString) -> Self {
        Self::new(StatusMessageKind::Info, text)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum StatusMessageKind {
    Error = 0,
    Warn = 1,
    Info = 2,
}

impl StatusMessageKind {
    fn to_color(&self) -> Color {
        match self {
            StatusMessageKind::Error => Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },

            StatusMessageKind::Warn => Color {
                r: 1.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            StatusMessageKind::Info => Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    SetMessage(StatusMessage),
    ClearMessage,
}

#[derive(Debug, Default)]
pub struct State {
    message: Option<StatusMessage>,
}

impl State {
    pub fn view(&self) -> Element<Msg> {
        let text = if let Some(message) = &self.message {
            text(message.text.clone()).style(|_theme: &Theme| text::Style {
                color: Some(message.kind.to_color()),
            })
        } else {
            text("")
        }
        .width(Fill);

        container(text)
            .width(Fill)
            .padding(1)
            .style(|theme| container::Style {
                border: Border {
                    color: theme.palette().primary,
                    width: 0.5,
                    radius: Radius::new(0.0),
                },
                ..Default::default()
            })
            .into()
    }

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::SetMessage(message) => {
                self.message = Some(message);
                Task::future(Self::clear_message_in(StatusMessage::MESSAGE_DURATION))
            }
            Msg::ClearMessage => {
                self.message = None;
                Task::none()
            }
        }
    }

    async fn clear_message_in(duration: Duration) -> Msg {
        tokio::time::sleep(duration).await;
        Msg::ClearMessage
    }
}
