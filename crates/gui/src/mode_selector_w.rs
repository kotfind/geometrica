use std::fmt::{self, Display};

use iced::{
    widget::{button, text, Column},
    Element,
    Length::Fill,
};

#[derive(Debug, Clone, Default)]
pub enum Mode {
    #[default]
    CreatePoint,
    Modify,
    Transform,
    Delete,
    Function,
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
                | (Mode::Function, Mode::Function)
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
            Mode::Function => write!(f, "Function"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    ModeSelected(Mode),
}

pub fn view(current_mode: &Mode) -> Element<Msg> {
    let mut column = Column::new().padding(5).spacing(5);

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
