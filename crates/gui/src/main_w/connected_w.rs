use std::collections::HashMap;

use anyhow::Context;
use client::Client;
use iced::{
    font::Weight,
    widget::{button, container, pane_grid, text},
    Border, Element, Font,
    Length::Fill,
    Task, Theme,
};
use types::core::{Ident, Value};

use crate::{canvas_w, command_w, status_bar_w::StatusMessage, variable_w};

#[derive(Debug)]
pub struct State {
    client: Client,
    vars: HashMap<Ident, Value>,
    panes: pane_grid::State<Pane>,

    command_w: command_w::State,
    variable_w: variable_w::State,
}

#[derive(Debug, Clone)]
pub enum Msg {
    PaneDrag(pane_grid::DragEvent),
    PaneResize(pane_grid::ResizeEvent),
    PaneClose(pane_grid::Pane),

    SetStatusMessage(StatusMessage),
    GotVars(HashMap<Ident, Value>),

    CanvasWMsg(canvas_w::Msg),
    CommandWMsg(command_w::Msg),
    VariableWMsg(variable_w::Msg),
}

// The numbers are explicitly specified, so that they persist across refactoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pane {
    CanvasW = 0,
    CommandW = 1,
    VariableW = 2,
}

static LEFT_PANE_RATIO: f32 = 0.2;
static RIGHT_PANE_RATIO: f32 = (1.0 - 2.0 * LEFT_PANE_RATIO) / (1.0 - LEFT_PANE_RATIO);

impl State {
    pub fn run_with(client: Client) -> (Self, Task<Msg>) {
        use pane_grid::{Axis::*, Configuration as Cfg};

        let panes = pane_grid::State::with_configuration(Cfg::Split {
            axis: Vertical,
            ratio: LEFT_PANE_RATIO,
            a: Box::new(Cfg::Pane(Pane::VariableW)),
            b: Box::new(Cfg::Split {
                axis: Vertical,
                ratio: RIGHT_PANE_RATIO,
                a: Box::new(Cfg::Pane(Pane::CanvasW)),
                b: Box::new(Cfg::Pane(Pane::CommandW)),
            }),
        });

        (
            Self {
                client: client.clone(),
                vars: Default::default(),
                panes,
                command_w: command_w::State::new(),
                variable_w: variable_w::State::new(),
            },
            Task::future(Self::fetch_vars_msg(client)),
        )
    }

    pub fn view(&self) -> Element<Msg> {
        self.view_master_area()
    }

    fn view_master_area(&self) -> Element<Msg> {
        pane_grid::PaneGrid::new(&self.panes, |pane, state, _| {
            let (title, body) = match state {
                Pane::CanvasW => ("", canvas_w::view(&self.vars).map(Msg::CanvasWMsg)),
                Pane::CommandW => ("Command Line", self.command_w.view().map(Msg::CommandWMsg)),
                Pane::VariableW => (
                    "Variables",
                    self.variable_w.view(&self.vars).map(Msg::VariableWMsg),
                ),
            };

            let mut content = pane_grid::Content::new(body);

            if state != &Pane::CanvasW {
                let title = container(text(title).font(Font {
                    weight: Weight::Bold,
                    ..Default::default()
                }))
                .padding(5);

                let controls = pane_grid::Controls::new(button("X").on_press(Msg::PaneClose(pane)));

                let title_bar = pane_grid::TitleBar::new(title)
                    .controls(controls)
                    .style(Self::title_bar_style);

                content = content.title_bar(title_bar).style(Self::pane_style);
            }

            content
        })
        .width(Fill)
        .height(Fill)
        .on_drag(Msg::PaneDrag)
        .on_resize(10, Msg::PaneResize)
        .into()
    }

    fn title_bar_style(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();
        container::Style {
            text_color: Some(palette.primary.strong.text),
            background: Some(palette.primary.strong.color.into()),
            ..Default::default()
        }
    }

    fn pane_style(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();
        container::Style {
            border: Border {
                color: palette.primary.strong.color,
                width: 2.,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::PaneDrag(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
                Task::none()
            }
            Msg::PaneDrag(_) => Task::none(),
            Msg::PaneResize(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
                Task::none()
            }
            Msg::PaneClose(pane) => {
                self.panes.close(pane);
                Task::none()
            }
            Msg::GotVars(vars) => {
                self.vars = vars;
                Task::future(Self::fetch_vars_msg(self.client.clone()))
            }
            Msg::CanvasWMsg(_msg) => Task::none(),
            Msg::CommandWMsg(msg) => self
                .command_w
                .update(msg, self.client.clone())
                .map(Msg::CommandWMsg),
            Msg::SetStatusMessage(_) => {
                unreachable!("should have been processed in parent widget")
            }
            Msg::VariableWMsg(msg) => match msg {
                variable_w::Msg::SetStatusMessage(message) => {
                    Task::done(Msg::SetStatusMessage(message))
                }
                _ => self
                    .variable_w
                    .update(msg, self.client.clone())
                    .map(Msg::VariableWMsg),
            },
        }
    }

    async fn fetch_vars_msg(client: Client) -> Msg {
        Self::fetch_vars(client).await.map_or_else(
            |e| Msg::SetStatusMessage(StatusMessage::error(format!("{e:#}"))),
            Msg::GotVars,
        )
    }

    async fn fetch_vars(client: Client) -> anyhow::Result<HashMap<Ident, Value>> {
        // FiXME: polling w/o timeout
        client.get_all_items().await.context("failed to fetch vars")
    }
}
