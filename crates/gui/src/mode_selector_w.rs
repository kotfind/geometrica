use std::fmt::{self, Display};

use iced::{
    widget::{button, text, Column},
    Element,
    Length::Fill,
};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum Mode {
    #[default]
    CreatePoint,
    Modify,
    Transform,
    Delete,
    Function,
}

impl Mode {
    /// Returns list of modes, that doesn't hold any data.
    fn basic_modes() -> &'static [Mode] {
        static BASIC_MODES: &[Mode] = &[
            Mode::CreatePoint,
            Mode::Modify,
            Mode::Transform,
            Mode::Delete,
        ];

        BASIC_MODES
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
        let btn =
            button(text(mode.to_string()))
                .width(Fill)
                .on_press_maybe(if mode != current_mode {
                    Some(Msg::ModeSelected(mode.clone()))
                } else {
                    None
                });

        column = column.push(btn);
    }

    column.into()
}
