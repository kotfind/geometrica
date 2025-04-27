use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use client::Client;
use itertools::Itertools;
use types::{
    core::{Ident, Value, ValueType},
    lang::{Definition, Expr, FuncCallExpr, FunctionSignature, ValueDefinition},
};

use crate::helpers::new_object_name_with_prefix;

#[derive(Debug, Clone, Default)]
pub enum Mode {
    #[default]
    Modify,
    Transform,
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
