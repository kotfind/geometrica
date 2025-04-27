use std::{
    fmt::{self, Display},
    time::Duration,
};

use client::Client;
use iced::{
    widget::{
        button, column, container, mouse_area, pick_list, scrollable, text, text_input, Column,
    },
    Background, Element,
    Length::Fill,
    Task, Theme,
};
use itertools::Itertools;
use types::{api::FunctionList, lang::FunctionSignature};

use crate::{helpers::perform_or_status, status_bar_w::StatusMessage};

static FUNCTION_FETCH_SLEEP_TIME: Duration = Duration::from_millis(50);

#[derive(Debug, Clone, Default)]
pub enum Mode {
    #[default]
    CreatePoint,
    Modify,
    Transform,
    Delete,
    Function(FunctionMode),
}

#[derive(Debug, Clone)]
pub struct FunctionMode {
    sign: FunctionSignature,
}

impl Mode {
    /// Returns list of modes, that have default values
    fn basic_modes() -> &'static [Mode] {
        static BASIC_MODES: &[Mode] = &[
            Mode::CreatePoint,
            Mode::Modify,
            Mode::Transform,
            Mode::Delete,
        ];

        BASIC_MODES
    }

    pub fn holds_same_variant(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Mode::CreatePoint, Mode::CreatePoint)
                | (Mode::Modify, Mode::Modify)
                | (Mode::Transform, Mode::Transform)
                | (Mode::Delete, Mode::Delete)
                | (Mode::Function { .. }, Mode::Function { .. })
        )
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::CreatePoint => write!(f, "Create Point"),
            Mode::Modify => write!(f, "Modify"),
            Mode::Transform => write!(f, "Transform"),
            Mode::Delete => write!(f, "Delete"),
            Mode::Function { .. } => write!(f, "Function"),
        }
    }
}

#[derive(Debug)]
pub struct State {
    func_list: FunctionList,
    func_type_filter: FunctionTypeFilter,
    func_name_filter: String,
    hovered_func_sign: Option<FunctionSignature>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SetStatusMessage(StatusMessage),

    ModeSelected(Mode),
    FunctionTypeFilterPicked(FunctionTypeFilter),
    FunctionNameFilterChanged(String),
    GotFunctionList(FunctionList),
    FunctionHovered(Option<FunctionSignature>),
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
                func_type_filter: FunctionTypeFilter::All,
                func_name_filter: "".to_string(),
                hovered_func_sign: None,
            },
            perform_or_status!(
                async move { client.list_funcs().await },
                Msg::GotFunctionList
            ),
        )
    }

    pub fn view<'a>(&'a self, mode: &'a Mode) -> Element<'a, Msg> {
        column![
            self.view_basic_mode_selector(mode),
            self.view_function_mode_selector(mode)
        ]
        .padding(5)
        .spacing(5)
        .into()
    }

    fn view_basic_mode_selector(&self, current_mode: &Mode) -> Element<Msg> {
        let mut column = Column::new().spacing(5);

        for mode in Mode::basic_modes() {
            let btn = button(text(mode.to_string())).width(Fill).on_press_maybe(
                if !mode.holds_same_variant(current_mode) {
                    Some(Msg::ModeSelected(mode.clone()))
                } else {
                    None
                },
            );

            column = column.push(btn);
        }

        column.into()
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

        let ans = mouse_area(ans).on_exit(Msg::FunctionHovered(None));

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
                let bg = theme.extended_palette().background;

                let col = match (mode, &self.hovered_func_sign) {
                    (
                        Mode::Function(FunctionMode {
                            sign: mode_sign, ..
                        }),
                        _,
                    ) if mode_sign == sign => Some(bg.strong.color),

                    (_, Some(hovered_sign)) if hovered_sign == sign => Some(bg.weak.color),

                    _ => None,
                }
                .map(Background::Color);

                container::Style {
                    background: col,
                    ..Default::default()
                }
            })
            .padding(2.5)
            .width(Fill);

        let ans = mouse_area(ans)
            .on_enter(Msg::FunctionHovered(Some(sign.clone())))
            .on_press(Msg::ModeSelected(Mode::Function(FunctionMode {
                sign: sign.clone(),
            })));

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
            Msg::FunctionHovered(sign) => {
                self.hovered_func_sign = sign;
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
