use anyhow::{anyhow, bail, Context};
use parser::ParseInto;
use types::lang::{Command, CommandArg};

use crate::{table::Table, Client};

macro_rules! unwrap_cmd_arg {
    (END FROM $args:ident) => {
        if $args.next().is_some() {
            bail!("unexpected command argument");
        }
    };

    (IDENT $name:ident FROM $args:ident) => {
        let CommandArg::Ident($name) =
            $args.next().ok_or(anyhow!("unexpected command argument"))?
        else {
            bail!("unexpected command argument")
        };
    };

    (EXPR $name:ident FROM $args:ident) => {
        let CommandArg::Expr($name) = $args.next().ok_or(anyhow!("unexpected command argument"))?
        else {
            bail!("unexpected command argument")
        };
    };
}

impl Client {
    /// Parses and executes `cmd`. Returns output of command in a form of a table of strings.
    /// Parsing output table is not recommended as it may change. Use output for printing only.
    pub async fn command(&self, cmd: impl ParseInto<Command>) -> anyhow::Result<Table> {
        let cmd = cmd.parse_into().context("failed to parse command")?;

        match &cmd.name.0 as &str {
            "get" => self.get_cmd(cmd.args).await,

            "get_all" => self.get_all_cmd(cmd.args).await,

            "eval" => self.eval_cmd(cmd.args).await,

            _ => bail!("undefined command: {}", cmd.name),
        }
    }

    async fn get_cmd(&self, args: Vec<CommandArg>) -> anyhow::Result<Table> {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(IDENT name FROM args);
        unwrap_cmd_arg!(END FROM args);

        let item = self.get_item(name.clone()).await?;

        Ok(Table::new_with_rows(
            ["Name", "Value"],
            [[name.to_string(), item.to_string()]],
        ))
    }

    async fn get_all_cmd(&self, _args: Vec<CommandArg>) -> anyhow::Result<Table> {
        let items = self.get_all_items().await?;

        Ok(Table::new_with_rows(
            ["Name", "Value"],
            items
                .into_iter()
                .map(|(name, value)| [name.to_string(), value.to_string()]),
        ))
    }

    async fn eval_cmd(&self, args: Vec<CommandArg>) -> anyhow::Result<Table> {
        let mut exprs = Vec::with_capacity(args.len());
        let mut args = args.into_iter().peekable();
        while args.peek().is_some() {
            unwrap_cmd_arg!(EXPR expr FROM args);
            exprs.push(expr);
        }

        let values = self.eval(exprs.clone()).await.context("failed to eval")?;

        assert_eq!(exprs.len(), values.len());

        // TODO: shorten value if too long
        // TODO: print expr with Display, not Debug
        Ok(Table::new_with_rows(
            ["Name", "Value"],
            exprs
                .into_iter()
                .zip(values.into_iter())
                .map(|(expr, value)| {
                    [
                        format!("{expr:?}"),
                        match value {
                            Ok(value) => value.to_string(),
                            Err(_) => "error".to_string(),
                        },
                    ]
                }),
        ))
    }
}
