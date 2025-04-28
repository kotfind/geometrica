use anyhow::Context;
use client::{Client, ClientSettings};
use iced::{
    font::Weight,
    widget::{button, checkbox, column, container, text, text_input},
    Alignment::Center,
    Element, Font,
    Length::Fill,
    Task,
};

use crate::{helpers::perform_or_status, status_bar_w::StatusMessage};

#[derive(Debug)]
pub struct State {
    server_url_input: String,
    try_spawn_server: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            server_url_input: ClientSettings::DEFAULT_URL
                .parse()
                .expect("DEFAULT_URL can be parsed to url"),
            try_spawn_server: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    SetStatusMessage(StatusMessage),
    Connected(Client),
    Connect,

    ServerUrlInputChanged(String),
    SetTrySpawnServer(bool),
}

impl State {
    pub fn view(&self) -> Element<Msg> {
        let title = text("Connection Options").font(Font {
            weight: Weight::Bold,
            ..Default::default()
        });

        let server_url_input =
            text_input("Server Url", &self.server_url_input).on_input(Msg::ServerUrlInputChanged);

        let try_spawn_server_check =
            checkbox("Try Spawn Server", self.try_spawn_server).on_toggle(Msg::SetTrySpawnServer);

        let submit_btn = button("Connect").on_press(Msg::Connect);

        let ans = column![title, server_url_input, try_spawn_server_check, submit_btn]
            .spacing(10)
            .align_x(Center);

        let ans = container(ans).max_width(300);

        let ans = container(ans)
            .width(Fill)
            .height(Fill)
            .align_x(Center)
            .align_y(Center);

        ans.into()
    }

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::Connected(_) | Msg::SetStatusMessage(_) => {
                unreachable!("should have been processed in a parent widget");
            }
            Msg::ServerUrlInputChanged(url) => {
                self.server_url_input = url;
                Task::none()
            }
            Msg::Connect => {
                perform_or_status!(
                    Self::connect(self.server_url_input.clone(), self.try_spawn_server),
                    Msg::Connected
                )
            }
            Msg::SetTrySpawnServer(try_spawn_server) => {
                self.try_spawn_server = try_spawn_server;
                Task::none()
            }
        }
    }

    async fn connect(server_url: String, try_spawn_server: bool) -> anyhow::Result<Client> {
        let client = Client::from(ClientSettings {
            server_url: server_url.parse().context("failed to parse server url")?,
            try_spawn_server,
        })
        .await
        .context("failed to connect")?;

        Ok(client)
    }
}
