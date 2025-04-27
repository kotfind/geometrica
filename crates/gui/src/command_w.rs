use std::{iter, sync::Arc};

use client::{Client, ScriptResult, Table};
use iced::{
    alignment::Vertical,
    font::Weight,
    keyboard::{key, Key, Modifiers},
    widget::{
        button, column, row, scrollable, text, text_editor, text_editor::Binding, Column,
        Scrollable, Text,
    },
    Alignment::Center,
    Element, Font,
    Length::{Fill, Shrink},
    Task, Theme,
};
use iced_aw::{grid_row, Grid, GridRow};
use itertools::Itertools;

use crate::helpers::my_tooltip;

#[derive(Debug)]
pub struct State {
    scripts_and_results: Vec<ScriptOrResult>,
    script_editor_content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Msg {
    ScriptEditorAction(text_editor::Action),
    SendScript,
    // Arc-ing ScriptResult to make in Clonable
    GotScriptResult(Arc<ScriptResult>),
}

impl State {
    pub fn new() -> Self {
        Self {
            scripts_and_results: Default::default(),
            script_editor_content: Default::default(),
        }
    }

    pub fn view(&self) -> Element<Msg> {
        let scripts_and_results = {
            let mut grd = Grid::new();

            for script_or_result in &self.scripts_and_results {
                grd = script_or_result.push_to_grid(grd);
            }

            grd = grd
                .row_height(Shrink)
                .vertical_alignment(Vertical::Top)
                .column_spacing(5)
                .row_spacing(10);

            Scrollable::with_direction(
                grd,
                scrollable::Direction::Both {
                    vertical: Default::default(),
                    horizontal: Default::default(),
                },
            )
            .width(Fill)
            .height(Fill)
        };

        let script_editor = text_editor(&self.script_editor_content)
            .placeholder("list_cmd!")
            .on_action(Msg::ScriptEditorAction)
            .key_binding(|key| match key.key {
                Key::Named(key::Named::Enter) => {
                    if key.modifiers.contains(Modifiers::CTRL)
                        || self.script_editor_content.line_count() > 1
                    {
                        Some(Binding::Insert('\n'))
                    } else {
                        Some(Binding::Custom(Msg::SendScript))
                    }
                }
                Key::Named(key::Named::Tab) => {
                    let s = " ".repeat(4);
                    let ans = s.chars().map(Binding::Insert);
                    Some(Binding::Sequence(ans.collect_vec()))
                }
                _ => Binding::from_key_press(key),
            });

        let submit_button = {
            let ans = button(">").on_press(Msg::SendScript);
            my_tooltip(ans, "Enter: send\nCtrl + Enter: new line\nTab: indent")
        };

        let editor_row = row![script_editor, submit_button].spacing(5);

        column![scripts_and_results, editor_row]
            .padding(5)
            .spacing(5)
            .width(Fill)
            .height(Fill)
            .align_x(Center)
            .into()
    }

    pub fn update(&mut self, msg: Msg, client: Client) -> Task<Msg> {
        match msg {
            Msg::SendScript => {
                let script = self.script_editor_content.text();
                if script.is_empty() {
                    return Task::none();
                }
                self.script_editor_content = text_editor::Content::new();
                self.scripts_and_results
                    .push(ScriptOrResult::Script(script.clone()));
                return Task::perform(
                    {
                        let client = client.clone();
                        async move { client.exec(script).await }
                    },
                    |res| Msg::GotScriptResult(Arc::new(res)),
                );
            }
            Msg::GotScriptResult(script_result) => {
                self.scripts_and_results
                    .push(ScriptOrResult::Result(script_result));
            }
            Msg::ScriptEditorAction(action) => self.script_editor_content.perform(action),
        }

        Task::none()
    }
}

#[derive(Clone, Debug)]
enum ScriptOrResult {
    Script(String),

    // Note: Arc-ing ScriptResult to make it Clonable
    Result(Arc<ScriptResult>),
}

impl ScriptOrResult {
    fn push_to_grid<'a, MSG: 'static>(&'a self, grd: Grid<'a, MSG>) -> Grid<'a, MSG> {
        let row = match self {
            ScriptOrResult::Script(script) => {
                let sender: Text<Theme> = text("ME:");

                let body: Text<Theme> = text(script);

                grid_row![sender, body]
            }
            ScriptOrResult::Result(res) => {
                let sender: Text<Theme> = text("SERVER:");

                let mut body = Column::new();
                let mut printed_something = false;

                for table in &res.results {
                    body = body.push(Self::table_to_grid(table));
                    printed_something = true;
                }

                if let Some(err) = &res.error {
                    body = body.push(text!("{:?}", err));
                    printed_something = true;
                }

                if !printed_something {
                    body = body.push(text("Ok"));
                }

                grid_row![sender, body]
            }
        };

        grd.push(row)
    }

    fn table_to_grid<MSG: 'static>(table: &Table) -> Grid<'_, MSG> {
        let header = table
            .header()
            .iter()
            .map(|txt| {
                text(txt).font(Font {
                    weight: Weight::Bold,
                    ..Default::default()
                })
            })
            .collect_vec();

        let body = table
            .rows()
            .iter()
            .map(|row| row.iter().map(text).collect_vec());

        let rows = iter::once(header)
            .chain(body)
            .map(|row| GridRow::with_elements(row));

        Grid::with_rows(rows.collect_vec()).spacing(5)
    }
}
