use std::{iter, sync::Arc};

use client::{Client, ScriptResult, Table};
use iced::{
    alignment::Vertical,
    font::Weight,
    widget::{button, column, row, scrollable, text, text_input, Column, Text},
    Alignment::Center,
    Element, Font,
    Length::{Fill, Shrink},
    Task, Theme,
};
use iced_aw::{grid_row, Grid, GridRow};
use itertools::Itertools;

#[derive(Debug)]
pub struct State {
    scripts_and_results: Vec<ScriptOrResult>,
    script_input: String,
}

#[derive(Debug, Clone)]
pub enum Msg {
    ScriptInputChanged(String),
    SendScript,
    // Arc-ing ScriptResult to make in Clonable
    GotScriptResult(Arc<ScriptResult>),
}

impl State {
    pub fn new() -> Self {
        Self {
            scripts_and_results: Default::default(),
            script_input: Default::default(),
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

            scrollable(grd).width(Fill).height(Fill)
        };

        let script_input = text_input("Script", &self.script_input)
            .on_input(Msg::ScriptInputChanged)
            .on_submit(Msg::SendScript)
            .width(Fill);

        let submit_button = button(">").on_press(Msg::SendScript);

        column![
            scripts_and_results,
            row![script_input, submit_button].padding(5).spacing(5)
        ]
        .padding(5)
        .spacing(5)
        .width(Fill)
        .height(Fill)
        .align_x(Center)
        .into()
    }

    pub fn update(&mut self, msg: Msg, client: Client) -> Task<Msg> {
        match msg {
            Msg::ScriptInputChanged(script) => self.script_input = script,

            Msg::SendScript => {
                let script = self.script_input.clone();
                if script.is_empty() {
                    return Task::none();
                }
                self.script_input.clear();
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
