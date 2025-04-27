use std::{
    fmt::{self, Display},
    time::Duration,
};

use client::Client;
use iced::{
    widget::{column, container, mouse_area, pick_list, scrollable, text, text_input, Column},
    Element,
    Length::Fill,
    Task, Theme,
};
use itertools::Itertools;
use types::{api::FunctionList, lang::FunctionSignature};

use crate::{
    helpers::perform_or_status,
    mode::{FunctionMode, Mode},
    my_colors,
    status_bar_w::StatusMessage,
};

static FUNCTION_FETCH_SLEEP_TIME: Duration = Duration::from_millis(50);

#[derive(Debug)]
pub struct State {
    func_list: FunctionList,

    func_type_filter: FunctionTypeFilter,
    func_name_filter: String,

    hovered_mode: Option<Mode>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SetStatusMessage(StatusMessage),

    ModeSelected(Mode),
    FunctionTypeFilterPicked(FunctionTypeFilter),
    FunctionNameFilterChanged(String),
    GotFunctionList(FunctionList),
    ModeHovered(Option<Mode>),
}

impl State {
    pub fn run_with(client: Client) -> (Self, Task<Msg>) {
        (
            Self {
                func_list: FunctionList {
                    operators: Vec::new(),
                    normal_builtins: Vec::new(),
                    user_defined: Vec::new(),
                },
                func_type_filter: FunctionTypeFilter::Builtins,
                func_name_filter: "".to_string(),
                hovered_mode: None,
            },
            perform_or_status!(
                async move { client.list_funcs().await },
                Msg::GotFunctionList
            ),
        )
    }

    pub fn view<'a>(&'a self, mode: &'a Mode) -> Element<'a, Msg> {
        let ans = column![
            self.view_basic_mode_selector(mode),
            self.view_function_mode_selector(mode)
        ]
        .padding(5)
        .spacing(5);

        let ans = mouse_area(ans).on_exit(Msg::ModeHovered(None));

        ans.into()
    }

    fn view_basic_mode_selector<'a>(&'a self, current_mode: &'a Mode) -> Element<'a, Msg> {
        let mut column = Column::new().spacing(5);

        let basic_modes = [
            Mode::Transform,
            Mode::Modify,
            Mode::CreatePoint,
            Mode::Delete,
        ];

        for mode in basic_modes {
            column = column.push(self.view_basic_mode_item(mode, current_mode));
        }

        column.into()
    }

    fn view_basic_mode_item<'a>(&'a self, mode: Mode, current_mode: &'a Mode) -> Element<'a, Msg> {
        let ans = text!("{mode}");

        let mode_cloned = mode.clone();
        let ans = container(ans)
            .style(move |theme: &Theme| {
                let col = match (&mode, &self.hovered_mode) {
                    (mode, _) if mode.holds_same_variant(current_mode) => {
                        my_colors::ITEM_BG_SELECTED(theme)
                    }
                    (_, Some(hovered_mode)) if hovered_mode.holds_same_variant(&mode) => {
                        my_colors::ITEM_BG_HOVERED(theme)
                    }
                    _ => my_colors::ITEM_BG_NORMAL(theme),
                };

                container::Style {
                    background: col,
                    ..Default::default()
                }
            })
            .width(Fill)
            .padding(2);

        let ans = mouse_area(ans)
            .on_enter(Msg::ModeHovered(Some(mode_cloned.clone())))
            .on_press(Msg::ModeSelected(mode_cloned.clone()));

        ans.into()
    }

    fn view_function_mode_selector<'a>(&'a self, mode: &'a Mode) -> Element<'a, Msg> {
        let type_filter = pick_list(
            FunctionTypeFilter::ALL,
            Some(self.func_type_filter.clone()),
            Msg::FunctionTypeFilterPicked,
        )
        .width(Fill);

        let name_filter = text_input("Function Name", &self.func_name_filter)
            .on_input(Msg::FunctionNameFilterChanged)
            .width(Fill);

        column![type_filter, name_filter, self.view_function_list(mode)]
            .spacing(5)
            .width(Fill)
            .into()
    }

    fn view_function_list<'a>(&'a self, mode: &'a Mode) -> Element<'a, Msg> {
        let rows = self
            .filter_functions()
            .map(|(type_str, sign)| self.view_function_list_item(mode, sign, type_str));

        let ans = Column::with_children(rows).width(Fill);

        let ans = scrollable(ans).width(Fill);

        ans.into()
    }

    fn view_function_list_item<'a>(
        &'a self,
        mode: &'a Mode,
        sign: &'a FunctionSignature,
        type_str: &'a str,
    ) -> Element<'a, Msg> {
        let ans = text!("{sign} ({type_str})");

        let ans = container(ans)
            .style(move |theme: &Theme| {
                let col = match (mode, &self.hovered_mode) {
                    (
                        Mode::Function(FunctionMode {
                            sign: mode_sign, ..
                        }),
                        _,
                    ) if mode_sign == sign => my_colors::ITEM_BG_SELECTED(theme),

                    (
                        _,
                        Some(Mode::Function(FunctionMode {
                            sign: hovered_sign, ..
                        })),
                    ) if hovered_sign == sign => my_colors::ITEM_BG_HOVERED(theme),

                    _ => my_colors::ITEM_BG_NORMAL(theme),
                };

                container::Style {
                    background: col,
                    ..Default::default()
                }
            })
            .padding(2.5)
            .width(Fill);

        let ans = mouse_area(ans)
            .on_enter(Msg::ModeHovered(Some(Mode::Function(FunctionMode::new(
                sign.clone(),
            )))))
            .on_press(Msg::ModeSelected(Mode::Function(FunctionMode::new(
                sign.clone(),
            ))));

        ans.into()
    }

    /// Return Value:
    /// - First argument of the tuple is a type_str, one of:
    ///     - "U" for user-defined functions
    ///     - "B" for builtin functions
    ///     - "O" for operators
    /// - Second argument is the function signature
    fn filter_functions(&self) -> impl Iterator<Item = (&str, &FunctionSignature)> {
        let FunctionList {
            operators,
            normal_builtins,
            user_defined,
        } = &self.func_list;

        let type_filter = &self.func_type_filter;
        let any_type = type_filter == &FunctionTypeFilter::All;

        [
            (
                "O",
                operators,
                any_type || type_filter == &FunctionTypeFilter::Operators,
            ),
            (
                "B",
                normal_builtins,
                any_type || type_filter == &FunctionTypeFilter::Builtins,
            ),
            (
                "U",
                user_defined,
                any_type || type_filter == &FunctionTypeFilter::UserDefined,
            ),
        ]
        .into_iter()
        .filter(|&(_, _, type_cond)| type_cond)
        .flat_map(|(type_str, list, _)| list.iter().map(move |item| (type_str, item)))
        .filter(|(_, sign)| sign.name.0.contains(&self.func_name_filter))
        .sorted_by(|(_, lhs_sign), (_, rhs_sign)| lhs_sign.to_string().cmp(&rhs_sign.to_string()))
    }

    pub fn update(&mut self, msg: Msg, client: Client) -> Task<Msg> {
        match msg {
            Msg::ModeSelected(_) | Msg::SetStatusMessage(_) => {
                unreachable!("should have been processed in parent widget")
            }
            Msg::FunctionTypeFilterPicked(type_filter) => {
                self.func_type_filter = type_filter;
                Task::none()
            }
            Msg::FunctionNameFilterChanged(name_filter) => {
                self.func_name_filter = name_filter;
                Task::none()
            }
            Msg::GotFunctionList(func_list) => {
                self.func_list = func_list;

                perform_or_status!(
                    async move {
                        tokio::time::sleep(FUNCTION_FETCH_SLEEP_TIME).await;
                        client.list_funcs().await
                    },
                    Msg::GotFunctionList
                )
            }
            Msg::ModeHovered(hovered_mode) => {
                self.hovered_mode = hovered_mode;
                Task::none()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionTypeFilter {
    All,
    Operators,
    Builtins,
    UserDefined,
}

impl FunctionTypeFilter {
    const ALL: &'static [FunctionTypeFilter] = &[
        FunctionTypeFilter::All,
        FunctionTypeFilter::Operators,
        FunctionTypeFilter::Builtins,
        FunctionTypeFilter::UserDefined,
    ];
}

impl Display for FunctionTypeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FunctionTypeFilter::All => "All",
            FunctionTypeFilter::Operators => "Operators (O)",
            FunctionTypeFilter::Builtins => "Builtins (B)",
            FunctionTypeFilter::UserDefined => "User Defined (U)",
        };
        write!(f, "{s}")
    }
}
