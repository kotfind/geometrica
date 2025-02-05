use std::collections::HashMap;

use client::Client;
use iced::{
    font::Weight,
    widget::{button, container, pane_grid, text},
    Border, Element, Font,
    Length::Fill,
    Task, Theme,
};
use types::core::{Ident, Value};

use crate::{canvas_w, command_w, variable_w};

#[derive(Debug)]
pub struct State {
    command_w: command_w::State,
    vars: HashMap<Ident, Value>,
    client: Client,
    panes: pane_grid::State<Pane>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    CanvasWMsg(canvas_w::Msg),
    CommandWMsg(command_w::Msg),
    GotVars(HashMap<Ident, Value>),
    PaneDrag(pane_grid::DragEvent),
    PaneResize(pane_grid::ResizeEvent),
    PaneClose(pane_grid::Pane),
}

// The numbers are explicitly specified, so that they persist across refactoring.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pane {
    CANVAS_W = 0,
    COMMAND_W = 1,
    VARIABLE_W = 2,
}

static LEFT_PANE_RATIO: f32 = 0.2;
static RIGHT_PANE_RATIO: f32 = (1.0 - 2.0 * LEFT_PANE_RATIO) / (1.0 - LEFT_PANE_RATIO);

impl State {
    pub fn run_with(client: Client) -> (Self, Task<Msg>) {
        use pane_grid::{Axis::*, Configuration as Cfg};

        let panes = pane_grid::State::with_configuration(Cfg::Split {
            axis: Vertical,
            ratio: LEFT_PANE_RATIO,
            a: Box::new(Cfg::Pane(Pane::VARIABLE_W)),
            b: Box::new(Cfg::Split {
                axis: Vertical,
                ratio: RIGHT_PANE_RATIO,
                a: Box::new(Cfg::Pane(Pane::CANVAS_W)),
                b: Box::new(Cfg::Pane(Pane::COMMAND_W)),
            }),
        });

        (
            Self {
                command_w: command_w::State::new(client.clone()),
                vars: Default::default(),
                client: client.clone(),
                panes,
            },
            Task::perform(Self::fetch_vars(client), Msg::GotVars),
        )
    }

    pub fn view(&self) -> Element<Msg> {
        pane_grid::PaneGrid::new(&self.panes, |pane, state, _| {
            let (title, body) = match state {
                Pane::CANVAS_W => ("", canvas_w::view(&self.vars).map(Msg::CanvasWMsg)),
                Pane::COMMAND_W => ("Command Line", self.command_w.view().map(Msg::CommandWMsg)),
                Pane::VARIABLE_W => ("Variables", variable_w::view(&self.vars)),
            };

            let mut content = pane_grid::Content::new(body);

            if state != &Pane::CANVAS_W {
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
            Msg::CanvasWMsg(_msg) => {
                // XXX: not used yet
            }
            Msg::CommandWMsg(msg) => {
                return self.command_w.update(msg).map(Msg::CommandWMsg);
            }
            Msg::GotVars(vars) => {
                self.vars = vars;
                return Task::perform(Self::fetch_vars(self.client.clone()), Msg::GotVars);
            }
            Msg::PaneDrag(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            Msg::PaneDrag(_) => {}
            Msg::PaneResize(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Msg::PaneClose(pane) => {
                self.panes.close(pane);
            }
        }

        Task::none()
    }

    async fn fetch_vars(client: Client) -> HashMap<Ident, Value> {
        // FiXME: polling w/o timeout
        // FIXME: unwrap
        client.get_all_items().await.unwrap()
    }
}
