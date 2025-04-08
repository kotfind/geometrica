use anyhow::{anyhow, bail, Context};
use parser::ParseInto;
use types::lang::{Command, CommandArg};

use crate::{table::Table, Client, ScriptResult};

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
    pub async fn command(&self, cmd: impl ParseInto<Command>) -> ScriptResult {
        let cmd = match cmd.parse_into().context("failed to parse command") {
            Ok(cmd) => cmd,
            Err(e) => return ScriptResult::error(e),
        };

        match &cmd.name.0 as &str {
            "get" => self.get_cmd(cmd.args).await.map(ScriptResult::ok_one),

            "get_all" => self.get_all_cmd(cmd.args).await.map(ScriptResult::ok_one),

            "eval" => self.eval_cmd(cmd.args).await.map(ScriptResult::ok_one),

            "set" => self
                .set_cmd(cmd.args)
                .await
                .map(|_| ScriptResult::ok_none()),

            "delete" => self
                .delete_cmd(cmd.args)
                .await
                .map(|_| ScriptResult::ok_none()),

            _ => Err(anyhow!("undefined command: {}", cmd.name)),
        }
        .unwrap_or_else(ScriptResult::error)
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
        Ok(Table::new_with_rows(
            ["Name", "Value"],
            exprs
                .into_iter()
                .zip(values.into_iter())
                .map(|(expr, value)| {
                    [
                        format!("{expr}"),
                        match value {
                            Ok(value) => value.to_string(),
                            Err(_) => "error".to_string(),
                        },
                    ]
                }),
        ))
    }

    async fn set_cmd(&self, args: Vec<CommandArg>) -> anyhow::Result<()> {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(IDENT name FROM args);
        unwrap_cmd_arg!(EXPR expr FROM args);
        unwrap_cmd_arg!(END FROM args);

        self.set(name, expr).await.context("failed to set value")?;

        Ok(())
    }

    async fn delete_cmd(&self, args: Vec<CommandArg>) -> anyhow::Result<()> {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(IDENT name FROM args);
        unwrap_cmd_arg!(END FROM args);

        self.delete(name).await.context("failed to delete")?;

        Ok(())
    }
}

// TODO: add tests
