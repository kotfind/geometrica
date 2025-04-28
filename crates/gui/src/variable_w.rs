use anyhow::Context;
use client::Client;
use iced::mouse;
use iced::widget::text;
use iced::{
    widget::{column, container, mouse_area, scrollable, text_input},
    Background, Element,
    Length::{Fill, Fixed},
    Task,
};
use iced_aw::{grid, grid_row, GridRow};
use itertools::Itertools;
use std::collections::HashMap;
use types::core::{Ident, Value};

use crate::{helpers::perform_or_status, mode::Mode, status_bar_w::StatusMessage};

#[derive(Debug)]
pub struct State {
    new_def_text: String,
    hovered_item: Option<Ident>,
    currently_editing: Option<(/* var_name: */ Ident, /* var_value: */ String)>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SetMode(Mode),
    SetStatusMessage(StatusMessage),
    None,

    NewDefTextChanged(String),
    HoveredItemChanged(Option<Ident>),
    CurrentlyEditingChanged(Option<(Ident, String)>),
    ApplyCurrentlyEditing,
    PickFunctionArg(Ident),

    DefineNew,
    Remove(Ident),
}

impl State {
    pub fn new() -> Self {
        Self {
            new_def_text: "".to_string(),
            hovered_item: None,
            currently_editing: None,
        }
    }

    pub fn view<'a>(&'a self, vars: &'a HashMap<Ident, Value>, mode: &'a Mode) -> Element<'a, Msg> {
        column![self.view_new_def(), self.view_variables(vars, mode)]
            .spacing(5)
            .padding(5)
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn view_variables<'a>(
        &'a self,
        vars: &'a HashMap<Ident, Value>,
        mode: &'a Mode,
    ) -> Element<'a, Msg> {
        let rows = vars
            .iter()
            .sorted_by(|(var_name_1, _), (var_name_2, _)| Ord::cmp(&var_name_1.0, &var_name_2.0))
            .map(|(var_name, var_value)| self.view_grid_row(var_name, var_value, mode));

        let ans = grid(rows.collect_vec())
            .column_widths(&[Fixed(100.0), Fill])
            .width(Fill);

        let ans = scrollable(ans);

        let ans = mouse_area(ans).on_exit(Msg::HoveredItemChanged(None));

        ans.into()
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
        let ans = container(inner)
            .width(Fill)
            .padding(2.5)
            .style(move |_theme| {
                let color = mode
                    .to_item_color_and_interaction(
                        var_name,
                        &var_value.value_type(),
                        &None,
                        &self.hovered_item,
                        Default::default(),
                    )
                    .0;

                container::Style {
                    background: Some(Background::Color(color)),
                    ..Default::default()
                }
            });

        let mut ans = mouse_area(ans)
            .on_press(match mode {
                _ if self.currently_editing.is_some() => Msg::ApplyCurrentlyEditing,

                Mode::Modify => {
                    Msg::CurrentlyEditingChanged(Some((var_name.clone(), var_value.to_string())))
                }
                Mode::Delete => Msg::Remove(var_name.clone()),
                Mode::Function(func_mode)
                    if func_mode.next_arg_type() == var_value.value_type() =>
                {
                    Msg::PickFunctionArg(var_name.clone())
                }
                _ => Msg::None,
            })
            .interaction({
                match mode {
                    Mode::Modify => mouse::Interaction::Text,
                    _ => {
                        mode.to_item_color_and_interaction(
                            var_name,
                            &var_value.value_type(),
                            &None,
                            &self.hovered_item,
                            Default::default(),
                        )
                        .1
                    }
                }
            });

        if !self
            .hovered_item
            .as_ref()
            .is_some_and(|item| item == var_name)
        {
            let msg = Msg::HoveredItemChanged(Some(var_name.clone()));
            ans = ans.on_enter(msg.clone()).on_move(move |_| msg.clone());
        }

        ans.into()
    }

    fn view_new_def(&self) -> Element<Msg> {
        text_input("new_var = 1.0", &self.new_def_text)
            .on_input(Msg::NewDefTextChanged)
            .on_submit(Msg::DefineNew)
            .width(Fill)
            .into()
    }

    pub fn update<'a>(
        &'a mut self,
        msg: Msg,
        client: Client,
        mode: &'a Mode,
        vars: &'a HashMap<Ident, Value>,
    ) -> Task<Msg> {
        match msg {
            Msg::SetStatusMessage(_) | Msg::SetMode(_) => {
                unreachable!("should have been processed in parent widget")
            }
            Msg::None => Task::none(),
            Msg::NewDefTextChanged(text) => {
                self.new_def_text = text;
                Task::none()
            }
            Msg::DefineNew => {
                let task = perform_or_status!({
                    let def = self.new_def_text.trim().to_string();

                    // FIXME: this method technically allows defining functions,
                    // which is not the intended behaviour
                    async move { client.define_one(def.clone()).await }
                });
                self.new_def_text = "".to_string();
                task
            }
            Msg::Remove(var_name) => perform_or_status!(async move { client.rm(var_name).await }),
            Msg::HoveredItemChanged(ident) => {
                self.hovered_item = ident;
                Task::none()
            }
            Msg::CurrentlyEditingChanged(currently_editing) => {
                self.currently_editing = currently_editing;
                Task::none()
            }
            Msg::ApplyCurrentlyEditing => match self.currently_editing.take() {
                Some((var_name, var_value)) if !var_value.is_empty() => {
                    perform_or_status!(async move { client.set(var_name, var_value).await })
                }
                _ => Task::none(),
            },
            Msg::PickFunctionArg(arg) => {
                let Mode::Function(func_mode) = mode else {
                    eprintln!("WARN: expected function mode");
                    return Task::none();
                };
                let mut func_mode = func_mode.clone();
                let vars = vars.clone();

                perform_or_status!(
                    async move {
                        func_mode
                            .add_arg(arg, client, &vars)
                            .await
                            .context("add_arg failed")?;
                        Ok(Mode::Function(func_mode))
                    },
                    Msg::SetMode
                )
            }
        }
    }
}
