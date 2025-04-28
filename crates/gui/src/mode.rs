use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use client::Client;
use iced::{mouse, Color};
use itertools::Itertools;
use types::{
    core::{Ident, Value, ValueType},
    lang::{Definition, Expr, FuncCallExpr, FunctionSignature, ValueDefinition},
};

use crate::{helpers::new_object_name_with_prefix, my_colors};

#[derive(Debug, Clone, Default)]
pub enum Mode {
    #[default]
    Transform,
    Modify,
    CreatePoint,
    Delete,
    Function(FunctionMode),
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

    pub fn to_item_color_and_interaction(
        &self,
        item_name: &Ident,
        item_value_type: &ValueType,
        picked: &Option<Ident>,
        hovered: &Option<Ident>,
        default_color: Color,
    ) -> (Color, mouse::Interaction) {
        let is_hovered = hovered.as_ref().is_some_and(|hovered| hovered == item_name);

        match &self {
            Mode::Modify if picked.as_ref().is_some_and(|picked| picked == item_name) => {
                (my_colors::ITEM_MODIFY_PICKED, mouse::Interaction::Move)
            }

            Mode::Function(func_mode)
                if func_mode
                    .selected_args()
                    .iter()
                    .any(|name| name == item_name) =>
            {
                (my_colors::ITEM_FUNCTION_PICKED, mouse::Interaction::None)
            }

            Mode::Function(func_mode)
                if is_hovered && &func_mode.next_arg_type() == item_value_type =>
            {
                (
                    my_colors::ITEM_FUNCTION_HOVERED,
                    mouse::Interaction::Pointer,
                )
            }

            Mode::Modify if is_hovered && item_value_type == &ValueType::Pt => {
                (my_colors::ITEM_MODIFY_HOVERED, mouse::Interaction::Pointer)
            }
            Mode::Delete if is_hovered => (
                my_colors::ITEM_DELETE_HOVERED,
                mouse::Interaction::NotAllowed,
            ),

            _ => (default_color, mouse::Interaction::None),
        }
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

#[derive(Debug, Clone)]
pub struct FunctionMode {
    pub sign: FunctionSignature,
    selected_args: Vec<Ident>,
}

impl FunctionMode {
    pub fn new(sign: FunctionSignature) -> Self {
        Self {
            sign,
            selected_args: Vec::new(),
        }
    }

    pub fn next_arg_type(&self) -> ValueType {
        self.sign.arg_types[self.selected_args.len()].clone()
    }

    pub fn selected_args(&self) -> &Vec<Ident> {
        &self.selected_args
    }

    /// Adds argument to [Self::selected_args] list. If all args
    /// are selected, function is executed and new object is created.
    pub async fn add_arg(
        &mut self,
        arg: Ident,
        client: Client,
        vars: &HashMap<Ident, Value>,
    ) -> anyhow::Result<()> {
        assert!(self.next_arg_type() == vars[&arg].value_type());

        self.selected_args.push(arg);

        let selected_num = self.selected_args.len();
        let args_num = self.sign.arg_types.len();

        match () {
            _ if selected_num > args_num => {
                panic!("selected more args, than possibly could")
            }
            _ if selected_num < args_num => {
                // NONE
            }
            _ if selected_num == args_num => {
                self.create_new_object(client, vars).await?;
                self.selected_args.clear();
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    async fn create_new_object(
        &self,
        client: Client,
        vars: &HashMap<Ident, Value>,
    ) -> anyhow::Result<()> {
        let name = new_object_name_with_prefix(
            &self
                .sign
                .name
                .0
                .chars()
                // Remove `#`
                .filter(|c| c.is_ascii_alphanumeric())
                .collect::<String>(),
            vars.keys(),
        );

        client
            .define_one(Definition::ValueDefinition(ValueDefinition {
                name,
                value_type: None, /* FIXME */
                body: Expr::FuncCall(FuncCallExpr {
                    name: self.sign.name.clone(),
                    args: self
                        .selected_args
                        .iter()
                        .map(|a| Box::new(Expr::Variable(a.clone())))
                        .collect_vec(),
                }),
            }))
            .await
    }
}
