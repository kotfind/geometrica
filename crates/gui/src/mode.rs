use std::fmt::{self, Display};

use types::lang::FunctionSignature;

#[derive(Debug, Clone, Default)]
pub enum Mode {
    #[default]
    Modify,
    Transform,
    CreatePoint,
    Delete,
    Function(FunctionMode),
}

#[derive(Debug, Clone)]
pub struct FunctionMode {
    pub sign: FunctionSignature,
}

impl Mode {
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
