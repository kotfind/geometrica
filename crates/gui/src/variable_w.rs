use std::collections::HashMap;

use client::Client;
use iced::{
    font::Weight,
    widget::{column, container, mouse_area, text, text_input},
    Background, Color, Element, Font,
    Length::{Fill, Shrink},
    Task,
};
use iced_aw::{grid, grid_row, GridRow};
use itertools::Itertools;
use types::core::{Ident, Value};

use crate::{mode_selector_w::Mode, status_bar_w::StatusMessage};

#[derive(Debug)]
pub struct State {
    new_def_text: String,
    mouse_on_item: Option<Ident>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SetStatusMessage(StatusMessage),
    None,

    NewDefTextChanged(String),
    MouseOnItemChanged(Option<Ident>),

    DefineNew,
    Remove(Ident),
}

impl State {
    pub fn new() -> Self {
        Self {
            new_def_text: "".to_string(),
            mouse_on_item: None,
        }
    }

    pub fn view<'a>(&'a self, vars: &'a HashMap<Ident, Value>, mode: &'a Mode) -> Element<'a, Msg> {
        column![self.view_variables(vars, mode), self.view_new_def()]
            .spacing(5)
            .padding(5)
            .width(Fill)
            .into()
    }

    fn view_variables<'a>(
        &'a self,
        vars: &'a HashMap<Ident, Value>,
        mode: &'a Mode,
    ) -> Element<'a, Msg> {
        let header = ["Name", "Value"]
            .into_iter()
            .map(|txt| {
                text(txt).font(Font {
                    weight: Weight::Bold,
                    ..Default::default()
                })
            })
            .collect_vec();

        let header = grid_row(header);

        let body = vars
            .iter()
            .sorted_by(|(var_name_1, _), (var_name_2, _)| Ord::cmp(&var_name_1.0, &var_name_2.0))
            .map(|(var_name, var_value)| self.view_grid_row(var_name, var_value, mode));

        let rows = std::iter::once(header).chain(body).collect_vec();

        mouse_area(
            grid(rows)
                .column_widths(&[Shrink, Fill])
                .width(Fill)
                .spacing(5),
        )
        .on_exit(Msg::MouseOnItemChanged(None))
        .into()
    }

    fn view_grid_row<'a>(
        &'a self,
        var_name: &'a Ident,
        var_value: &'a Value,
        mode: &'a Mode,
    ) -> GridRow<'a, Msg> {
        fn cell<'a>(
            mouse_on_item: &'a Option<Ident>,
            var_name: &'a Ident,
            txt: String,
            mode: &'a Mode,
        ) -> Element<'a, Msg> {
            let ans = text(txt).width(Fill);
            let ans = container(ans).style(move |_theme| {
                if !mouse_on_item.as_ref().is_some_and(|item| item == var_name) {
                    return Default::default();
                }

                let color = match mode {
                    Mode::Modify => Color {
                        r: 1.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    },
                    Mode::Delete => Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                    Mode::Function => Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    },
                    _ => Default::default(),
                };

                container::Style {
                    background: Some(Background::Color(color)),
                    ..Default::default()
                }
            });

            mouse_area(ans)
                .on_press(match mode {
                    Mode::Modify => {
                        /* TODO */
                        Msg::None
                    }
                    Mode::Delete => Msg::Remove(var_name.clone()),
                    Mode::Function => {
                        /* TODO */
                        Msg::None
                    }
                    _ => Msg::None,
                })
                .on_enter(Msg::MouseOnItemChanged(Some(var_name.clone())))
                .into()
        }

        grid_row![
            cell(&self.mouse_on_item, var_name, var_name.to_string(), mode),
            cell(&self.mouse_on_item, var_name, var_value.to_string(), mode)
        ]
    }

    fn view_new_def(&self) -> Element<Msg> {
        text_input("new_var = 1.0", &self.new_def_text)
            .on_input(Msg::NewDefTextChanged)
            .on_submit(Msg::DefineNew)
            .width(Fill)
            .into()
    }

    pub fn update(&mut self, msg: Msg, client: Client) -> Task<Msg> {
        match msg {
            Msg::NewDefTextChanged(text) => {
                self.new_def_text = text;
                Task::none()
            }
            Msg::DefineNew => {
                let task =
                    Task::perform(Self::define_new(client, self.new_def_text.clone()), |res| {
                        res.map_or_else(
                            |e| Msg::SetStatusMessage(StatusMessage::error(format!("{e:#}"))),
                            |_| Msg::None,
                        )
                    });
                self.new_def_text = "".to_string();
                task
            }
            Msg::SetStatusMessage(_) => {
                unreachable!("should have been processed in parent widget")
            }
            Msg::None => Task::none(),
            Msg::Remove(var_name) => Task::perform(Self::remove(client, var_name), |res| {
                res.map_or_else(
                    |e| Msg::SetStatusMessage(StatusMessage::error(format!("{e:#}"))),
                    |_| Msg::None,
                )
            }),
            Msg::MouseOnItemChanged(ident) => {
                self.mouse_on_item = ident;
                Task::none()
            }
        }
    }

    async fn define_new(client: Client, def: String) -> anyhow::Result<()> {
        // FIXME: this method technically allows defining functions,
        // which is not the intended behaviour
        client.define_one(def.trim()).await
    }

    async fn remove(client: Client, var_name: Ident) -> anyhow::Result<()> {
        client.rm(var_name).await
    }
}
