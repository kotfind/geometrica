use std::collections::HashMap;

use client::Client;
use iced::{
    font::Weight,
    widget::{button, column, container, pane_grid, text},
    Border, Element, Font,
    Length::Fill,
    Task, Theme,
};
use types::core::{Ident, Value};

use crate::{
    args_w, canvas_w, command_w,
    helpers::perform_or_status,
    mode::Mode,
    mode_selector_w::{self},
    status_bar_w::StatusMessage,
    top_bar_w, variable_w,
};

#[derive(Debug)]
pub struct State {
    client: Client,
    vars: HashMap<Ident, Value>,
    panes: pane_grid::State<Pane>,

    mode: Mode,

    command_w: command_w::State,
    variable_w: variable_w::State,
    mode_selector_w: mode_selector_w::State,
    canvas_w: canvas_w::State,
}

#[derive(Debug, Clone)]
pub enum Msg {
    PaneDrag(pane_grid::DragEvent),
    PaneResize(pane_grid::ResizeEvent),
    PaneClose(pane_grid::Pane),

    SetStatusMessage(StatusMessage),
    GotVars(HashMap<Ident, Value>),
    Disconnected,

    CanvasWMsg(canvas_w::Msg),
    CommandWMsg(command_w::Msg),
    VariableWMsg(variable_w::Msg),
    ModeSelectorW(mode_selector_w::Msg),
    TopBarWMsg(top_bar_w::Msg),
    ArgsWMsg(args_w::Msg),
}

// The numbers are explicitly specified, so that they persist across refactoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pane {
    CanvasW = 0,
    CommandW = 1,
    VariableW = 2,
    ModeSelectorW = 3,
    ArgsW = 4,
}

static LEFT_PANE_RATIO: f32 = 0.2;
static RIGHT_PANE_RATIO: f32 = (1.0 - 2.0 * LEFT_PANE_RATIO) / (1.0 - LEFT_PANE_RATIO);

impl State {
    pub fn run_with(client: Client) -> (Self, Task<Msg>) {
        use pane_grid::{Axis::*, Configuration as Cfg};

        let panes = pane_grid::State::with_configuration(Cfg::Split {
            axis: Vertical,
            ratio: LEFT_PANE_RATIO,
            a: Box::new(Cfg::Split {
                axis: Horizontal,
                ratio: 0.5,
                a: Box::new(Cfg::Pane(Pane::VariableW)),
                b: Box::new(Cfg::Pane(Pane::CommandW)),
            }),
            b: Box::new(Cfg::Split {
                axis: Vertical,
                ratio: RIGHT_PANE_RATIO,
                a: Box::new(Cfg::Pane(Pane::CanvasW)),
                b: Box::new(Cfg::Split {
                    axis: Horizontal,
                    ratio: 0.5,
                    a: Box::new(Cfg::Pane(Pane::ModeSelectorW)),
                    b: Box::new(Cfg::Pane(Pane::ArgsW)),
                }),
            }),
        });

        let (mode_selector_w, mode_selector_w_task) =
            mode_selector_w::State::run_with(client.clone());
        let mode_selector_w_task = mode_selector_w_task.map(Msg::ModeSelectorW);

        let fetch_vars_task = perform_or_status!(
            {
                let client = client.clone();
                async move { client.get_all_items().await }
            },
            Msg::GotVars
        );

        (
            Self {
                client: client.clone(),
                vars: Default::default(),
                panes,

                mode: Default::default(),

                command_w: command_w::State::new(),
                variable_w: variable_w::State::new(),
                mode_selector_w,
                canvas_w: canvas_w::State::new(),
            },
            Task::batch([mode_selector_w_task, fetch_vars_task]),
        )
    }

    pub fn view(&self) -> Element<Msg> {
        column![
            top_bar_w::view().map(Msg::TopBarWMsg),
            self.view_master_area()
        ]
        .width(Fill)
        .into()
    }

    fn view_master_area(&self) -> Element<Msg> {
        pane_grid::PaneGrid::new(&self.panes, |pane, state, _| {
            let (title, body) = match state {
                Pane::CanvasW => (
                    "",
                    self.canvas_w
                        .view(&self.vars, &self.mode)
                        .map(Msg::CanvasWMsg),
                ),
                Pane::CommandW => ("Command Line", self.command_w.view().map(Msg::CommandWMsg)),
                Pane::VariableW => (
                    "Variables",
                    self.variable_w
                        .view(&self.vars, &self.mode)
                        .map(Msg::VariableWMsg),
                ),
                Pane::ModeSelectorW => (
                    "Mode Selector",
                    self.mode_selector_w
                        .view(&self.mode)
                        .map(Msg::ModeSelectorW),
                ),
                Pane::ArgsW => ("Arguments", args_w::view(&self.mode).map(Msg::ArgsWMsg)),
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
            Msg::SetStatusMessage(_) | Msg::Disconnected => {
                unreachable!("should have been processed in parent widget")
            }

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
                // TODO?: sleep before next request?

                self.vars = vars;

                perform_or_status!(
                    {
                        let client = self.client.clone();
                        async move { client.get_all_items().await }
                    },
                    Msg::GotVars
                )
            }
            Msg::CanvasWMsg(msg) => match msg {
                canvas_w::Msg::SetStatusMessage(message) => {
                    Task::done(Msg::SetStatusMessage(message))
                }
                canvas_w::Msg::SetMode(mode) => {
                    self.mode = mode;
                    Task::none()
                }
                _ => self
                    .canvas_w
                    .update(msg, self.client.clone(), &self.mode, &self.vars)
                    .map(Msg::CanvasWMsg),
            },
            Msg::CommandWMsg(msg) => self
                .command_w
                .update(msg, self.client.clone())
                .map(Msg::CommandWMsg),
            Msg::VariableWMsg(msg) => match msg {
                variable_w::Msg::SetStatusMessage(message) => {
                    Task::done(Msg::SetStatusMessage(message))
                }
                variable_w::Msg::SetMode(mode) => {
                    self.mode = mode;
                    Task::none()
                }
                _ => self
                    .variable_w
                    .update(msg, self.client.clone(), &self.mode, &self.vars)
                    .map(Msg::VariableWMsg),
            },
            Msg::ModeSelectorW(msg) => match msg {
                mode_selector_w::Msg::ModeSelected(mode) => {
                    self.mode = mode;
                    Task::none()
                }
                mode_selector_w::Msg::SetStatusMessage(message) => {
                    Task::done(Msg::SetStatusMessage(message))
                }
                _ => self
                    .mode_selector_w
                    .update(msg, self.client.clone())
                    .map(Msg::ModeSelectorW),
            },
            Msg::TopBarWMsg(msg) => match msg {
                top_bar_w::Msg::SetStatusMessage(message) => {
                    Task::done(Msg::SetStatusMessage(message))
                }
                top_bar_w::Msg::Disconnect => Task::done(Msg::Disconnected),
                top_bar_w::Msg::SetFitAllTransformation => {
                    self.canvas_w.set_fit_all_transformation(&self.vars);
                    Task::none()
                }
                top_bar_w::Msg::SetIdentityTransformation => {
                    self.canvas_w.set_identity_transformation();
                    Task::none()
                }
                _ => top_bar_w::update(msg, self.client.clone()).map(Msg::TopBarWMsg),
            },
            Msg::ArgsWMsg(msg) => args_w::update(msg).map(Msg::ArgsWMsg),
        }
    }
}
