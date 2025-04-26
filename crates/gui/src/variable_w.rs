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

use crate::{helpers::perform_or_status, mode_selector_w::Mode, status_bar_w::StatusMessage};

#[derive(Debug)]
pub struct State {
    new_def_text: String,
    mouse_on_item: Option<Ident>,
    currently_editing: Option<(/* var_name: */ Ident, /* var_value: */ String)>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SetStatusMessage(StatusMessage),
    None,

    NewDefTextChanged(String),
    MouseOnItemChanged(Option<Ident>),
    CurrentlyEditingChanged(Option<(Ident, String)>),
    ApplyCurrentlyEditing,

    DefineNew,
    Remove(Ident),
}

impl State {
    pub fn new() -> Self {
        Self {
            new_def_text: "".to_string(),
            mouse_on_item: None,
            currently_editing: None,
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
        let name_cell_inner = text!("{var_name}").into();

        let value_cell_inner = match &self.currently_editing {
            Some((editing_var_name, editing_var_value)) if editing_var_name == var_name => {
                text_input(&var_value.to_string(), editing_var_value)
                    .on_input(|new_value| {
                        Msg::CurrentlyEditingChanged(Some((editing_var_name.clone(), new_value)))
                    })
                    .on_submit(Msg::ApplyCurrentlyEditing)
                    .into()
            }
            _ => text!("{var_value}").into(),
        };

        grid_row![
            self.view_cell(mode, var_name, var_value, name_cell_inner),
            self.view_cell(mode, var_name, var_value, value_cell_inner)
        ]
    }

    fn view_cell<'a>(
        &'a self,
        mode: &'a Mode,
        var_name: &'a Ident,
        var_value: &'a Value,
        inner: Element<'a, Msg>,
    ) -> Element<'a, Msg> {
        let ans = container(inner).width(Fill).style(move |_theme| {
            if !self
                .mouse_on_item
                .as_ref()
                .is_some_and(|item| item == var_name)
            {
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
                _ if self.currently_editing.is_some() => Msg::ApplyCurrentlyEditing,

                Mode::Modify => {
                    Msg::CurrentlyEditingChanged(Some((var_name.clone(), var_value.to_string())))
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
                let task = perform_or_status!({
                    let client = client.clone();
                    let def = self.new_def_text.trim().to_string();

                    // FIXME: this method technically allows defining functions,
                    // which is not the intended behaviour
                    async move { client.define_one(def.clone()).await }
                });
                self.new_def_text = "".to_string();
                task
            }
            Msg::SetStatusMessage(_) => {
                unreachable!("should have been processed in parent widget")
            }
            Msg::None => Task::none(),
            Msg::Remove(var_name) => perform_or_status!({
                let client = client.clone();
                async move { client.rm(var_name).await }
            }),
            Msg::MouseOnItemChanged(ident) => {
                self.mouse_on_item = ident;
                Task::none()
            }
            Msg::CurrentlyEditingChanged(currently_editing) => {
                self.currently_editing = currently_editing;
                Task::none()
            }
            Msg::ApplyCurrentlyEditing => match self.currently_editing.take() {
                Some((var_name, var_value)) if !var_value.is_empty() => {
                    perform_or_status!({
                        let client = client.clone();
                        async move { client.set(var_name, var_value).await }
                    })
                }
                _ => Task::none(),
            },
        }
    }
}
