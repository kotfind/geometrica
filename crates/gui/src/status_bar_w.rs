use std::{
    fmt::{self, Display},
    time::Duration,
};

use iced::{
    widget::{container, mouse_area, text},
    Color, Element,
    Length::Fill,
    Task, Theme,
};

use crate::{helpers::my_tooltip, my_colors};

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

impl Display for StatusMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.text)
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
            StatusMessageKind::Error => my_colors::STATUS_ERROR,
            StatusMessageKind::Warn => my_colors::STATUS_WARN,
            StatusMessageKind::Info => my_colors::STATUS_INFO,
        }
    }
}

impl Display for StatusMessageKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StatusMessageKind::Error => "ERR",
            StatusMessageKind::Warn => "WARN",
            StatusMessageKind::Info => "INFO",
        };
        write!(f, "{s}")
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
        let ans = if let Some(message) = &self.message {
            text(message.text.clone()).style(|_theme: &Theme| text::Style {
                color: Some(message.kind.to_color()),
            })
        } else {
            text("")
        }
        .width(Fill);

        let ans = container(ans)
            .width(Fill)
            .padding(1)
            .style(|theme| container::Style {
                background: my_colors::BAR_BG(theme),
                ..Default::default()
            });

        let ans = mouse_area(ans).on_press(Msg::ClearMessage);

        my_tooltip(ans, "press to hide status message")
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
