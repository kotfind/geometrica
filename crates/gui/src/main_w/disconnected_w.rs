use anyhow::Context;
use client::{Client, ClientSettings};
use iced::{
    font::Weight,
    widget::{button, column, container, text, text_input},
    Alignment::Center,
    Element, Font,
    Length::Fill,
    Task,
};

use crate::{helpers::perform_or_status, status_bar_w::StatusMessage};

#[derive(Debug)]
pub struct State {
    server_url_input: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            server_url_input: ClientSettings::DEFAULT_URL
                .parse()
                .expect("DEFAULT_URL can be parsed to url"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    Connected(Client),
    ServerUrlInputChanged(String),
    Connect,
    SetStatusMessage(StatusMessage),
}

impl State {
    pub fn view(&self) -> Element<Msg> {
        let title = text("Connection Options").font(Font {
            weight: Weight::Bold,
            ..Default::default()
        });
        let input =
            text_input("Server Url", &self.server_url_input).on_input(Msg::ServerUrlInputChanged);

        let btn = button("Connect").on_press(Msg::Connect);

        let content = column![title, input, btn].spacing(5).align_x(Center);

        let inner = container(content).max_width(300);

        container(inner)
            .width(Fill)
            .height(Fill)
            .align_x(Center)
            .align_y(Center)
            .into()
    }

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::Connected(_) => {
                unreachable!("This message should have been processed in a parent widget, and not forwarded here");
            }
            Msg::ServerUrlInputChanged(url) => {
                self.server_url_input = url;
                Task::none()
            }
            Msg::Connect => {
                perform_or_status!(Self::connect(self.server_url_input.clone()), Msg::Connected)
            }
            Msg::SetStatusMessage(_) => {
                unreachable!("this message should have been processed in a parent widget");
            }
        }
    }

    async fn connect(server_url: String) -> anyhow::Result<Client> {
        // TODO: more settings
        let client = Client::from(ClientSettings {
            server_url: server_url.parse().context("failed to parse server url")?,
            ..Default::default()
        })
        .await
        .context("failed to connect")?;

        Ok(client)
    }
}
