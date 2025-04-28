use indoc::indoc;
use std::{path::PathBuf, str::FromStr};

use anyhow::{anyhow, bail, Context};
use enum_iterator::Sequence;
use itertools::Itertools;
use parser::ParseInto;
use types::{
    api::FunctionList,
    core::Value,
    lang::{Command, CommandArg, Expr},
};

use crate::{table::Table, Client, ScriptResult};

impl Client {
    pub async fn command(&self, cmd: impl ParseInto<Command>) -> ScriptResult {
        let cmd = match cmd.parse_into().context("failed to parse command") {
            Ok(cmd) => cmd,
            Err(err) => return ScriptResult::error(err),
        };

        let cmd_type = match CommandType::from_str(&cmd.name.0) {
            Ok(cmd_type) => cmd_type,
            Err(err) => return ScriptResult::error(err),
        };

        cmd_type.apply(self, cmd.args).await
    }

    pub fn list_cmd() -> Vec<CommandType> {
        enum_iterator::all().collect()
    }
}

#[derive(Debug, Sequence)]
pub enum CommandType {
    Clear,
    Eval,
    Get,
    GetAll,
    ListCmd,
    ListFunc,
    Rm,
    Set,
    Save,
    Load,
    SaveSvg,
}

impl FromStr for CommandType {
    type Err = anyhow::Error;

    fn from_str(cmd_name: &str) -> Result<Self, Self::Err> {
        let mut ans = None;
        for cmd in enum_iterator::all::<CommandType>() {
            if cmd.name() == cmd_name {
                ans = Some(cmd);
                break;
            }
        }

        ans.ok_or(anyhow!("undefined command: {}", cmd_name))
    }
}

impl CommandType {
    // Returns a tuple of (name, signature, description)
    //
    // This function should only be used to implement `name`, `sign` and `desc`.
    // In all other cases use these function rather that this.
    fn _info(&self) -> (&str, &str, &str) {
        match self {
            CommandType::Get => ("get", "ident+", "get item's value by it's name"),
            CommandType::GetAll => ("get_all", "-", "get all items' values"),
            CommandType::Eval => ("eval", "expr+", "evaluate some expressions"),
            CommandType::Set => ("set", "ident expr", "set item's value to expr's value"),
            CommandType::Rm => ("rm", "ident+", "remove some items"),
            CommandType::ListFunc => ("list_func", "-", "list all functions"),
            CommandType::ListCmd => ("list_cmd", "-", "list all commands"),
            CommandType::Clear => ("clear", "-", "clear all items and user-defined functions"),
            CommandType::Save => (
                "save",
                "expr",
                indoc!(
                    "writes the state to a file
                    expr should evaluate to a file path (str)"
                ),
            ),
            CommandType::Load => (
                "load",
                "expr",
                indoc!(
                    "reads state from file
                    expr should evaluate to a file path (str)"
                ),
            ),
            CommandType::SaveSvg => (
                "save_svg",
                "expr",
                indoc!(
                    "writes svg image to a file
                    expr should evaluate to a file path (str)
                    note: can NOT be imported"
                ),
            ),
        }
    }

    pub fn name(&self) -> &str {
        self._info().0
    }

    pub fn sign(&self) -> &str {
        self._info().1
    }

    pub fn desc(&self) -> &str {
        self._info().2
    }

    async fn apply(&self, client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        match self {
            CommandType::Get => Self::get_cmd(client, args).await,
            CommandType::GetAll => Self::get_all_cmd(client, args).await,
            CommandType::Eval => Self::eval_cmd(client, args).await,
            CommandType::Set => Self::set_cmd(client, args).await,
            CommandType::Rm => Self::rm_cmd(client, args).await,
            CommandType::ListFunc => Self::list_func_cmd(client, args).await,
            CommandType::ListCmd => Self::list_cmd_cmd(args),
            CommandType::Clear => Self::clear_cmd(client, args).await,
            CommandType::Save => Self::save_cmd(client, args).await,
            CommandType::Load => Self::load_cmd(client, args).await,
            CommandType::SaveSvg => Self::save_svg_cmd(client, args).await,
        }
    }

    async fn get_cmd(client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(IDENT name FROM args);
        unwrap_cmd_arg!(END FROM args);

        let item = match client.get_item(name.clone()).await {
            Ok(item) => item,
            Err(err) => return ScriptResult::error(err.context("get_item failed")),
        };

        ScriptResult::ok_one(Table::new_with_rows(
            ["Name", "Value"],
            [[name.to_string(), item.to_string()]],
        ))
    }

    async fn get_all_cmd(client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(END FROM args);

        let items = match client.get_all_items().await {
            Ok(items) => items,
            Err(err) => return ScriptResult::error(err.context("get_all_items failed")),
        };

        ScriptResult::ok_one(Table::new_with_rows(
            ["Name", "Value"],
            items
                .into_iter()
                .map(|(name, value)| [name.to_string(), value.to_string()])
                .sorted(),
        ))
    }

    async fn eval_cmd(client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        let mut exprs = Vec::with_capacity(args.len());
        let mut args = args.into_iter().peekable();
        while args.peek().is_some() {
            unwrap_cmd_arg!(EXPR expr FROM args);
            exprs.push(expr);
        }

        let values = match client.eval(exprs.clone()).await {
            Ok(values) => values,
            Err(err) => return ScriptResult::error(err.context("eval failed")),
        };

        assert_eq!(exprs.len(), values.len());

        fn fit(text: impl ToString) -> String {
            fit_to_len(text, 100)
        }

        ScriptResult::ok_one(Table::new_with_rows(
            ["Name", "Value"],
            exprs
                .into_iter()
                .zip(values.into_iter())
                .map(|(expr, value)| {
                    [
                        fit(expr),
                        match value {
                            Ok(value) => fit(value),
                            Err(err) => fit(err),
                        },
                    ]
                }),
        ))
    }

    async fn set_cmd(client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(IDENT name FROM args);
        unwrap_cmd_arg!(EXPR expr FROM args);
        unwrap_cmd_arg!(END FROM args);

        if let Err(err) = client.set(name, expr).await {
            return ScriptResult::error(err.context("set failed"));
        }

        ScriptResult::ok_none()
    }

    async fn rm_cmd(client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(IDENT name FROM args);
        unwrap_cmd_arg!(END FROM args);

        if let Err(err) = client.rm(name).await {
            return ScriptResult::error(err.context("rm failed"));
        }

        ScriptResult::ok_none()
    }

    async fn list_func_cmd(client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(END FROM args);

        let FunctionList {
            operators,
            normal_builtins,
            user_defined,
        } = match client.list_funcs().await {
            Ok(funcs) => funcs,
            Err(err) => return ScriptResult::error(err.context("list_funcs failed")),
        };

        let funcs = [
            (operators, "operator"),
            (normal_builtins, "builtin"),
            (user_defined, "user-defined"),
        ]
        .into_iter()
        .flat_map(|(list, type_name)| {
            list.into_iter()
                .map(|sign| sign.to_string())
                .sorted()
                .map(|sign| [sign, type_name.to_string()])
        });

        // let operators = operators
        //     .into_iter()
        //     .map(|sign| [sign.to_string(), "operator".to_string()]);
        //
        // let normal_builtins = normal_builtins
        //     .into_iter()
        //     .map(|sign| [sign.to_string(), "builtin".to_string()]);
        //
        // let user_defined = user_defined
        //     .into_iter()
        //     .map(|sign| [sign.to_string(), "user-defined".to_string()]);
        //
        // let funcs = iter::empty()
        //     .chain(operators)
        //     .chain(normal_builtins)
        //     .chain(user_defined)
        //     .sorted_by(|lhs, rhs| (&lhs[1], &lhs[0]).cmp(&(&rhs[1], &rhs[0])));

        ScriptResult::ok_one(Table::new_with_rows(["Signature", "Type"], funcs))
    }

    fn list_cmd_cmd(args: Vec<CommandArg>) -> ScriptResult {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(END FROM args);

        let mut rows = Vec::new();
        for cmd in enum_iterator::all::<CommandType>() {
            let name = cmd.name().to_string() + "!";
            let sign = cmd.sign().to_string();
            let desc = cmd.desc().to_string();
            rows.push([name, sign, desc]);
        }

        ScriptResult::ok_one(Table::new_with_rows(["Name", "Sign", "Description"], rows))
    }

    async fn clear_cmd(client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(END FROM args);

        if let Err(err) = client.clear().await {
            return ScriptResult::error(err.context("clear failed"));
        }

        ScriptResult::ok_none()
    }

    async fn eval_file_path(client: &Client, expr: Expr) -> anyhow::Result<PathBuf> {
        Ok(match client.eval_one(expr).await {
            Ok(Value::Str(Some(path))) => PathBuf::from(path),
            Ok(Value::Str(None)) => {
                bail!("file path should not be none")
            }

            Ok(v) => {
                bail!("file path must be a str, got {}", v.value_type())
            }
            Err(err) => return Err(err.context("failed to evaluate file path expr")),
        })
    }

    async fn save_cmd(client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(EXPR expr FROM args);
        unwrap_cmd_arg!(END FROM args);

        let path = match Self::eval_file_path(client, expr).await {
            Ok(path) => path,
            Err(err) => return ScriptResult::error(err.context("eval_file_path failed")),
        };

        if let Err(err) = client.save(&path).await {
            return ScriptResult::error(err.context("save failed"));
        }

        ScriptResult::ok_none()
    }

    async fn load_cmd(client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(EXPR expr FROM args);
        unwrap_cmd_arg!(END FROM args);

        let path = match Self::eval_file_path(client, expr).await {
            Ok(path) => path,
            Err(err) => return ScriptResult::error(err.context("eval_file_path failed")),
        };

        if let Err(err) = client.load(&path).await {
            return ScriptResult::error(err.context("load failed"));
        }

        ScriptResult::ok_none()
    }

    async fn save_svg_cmd(client: &Client, args: Vec<CommandArg>) -> ScriptResult {
        let mut args = args.into_iter();
        unwrap_cmd_arg!(EXPR expr FROM args);
        unwrap_cmd_arg!(END FROM args);

        let path = match Self::eval_file_path(client, expr).await {
            Ok(path) => path,
            Err(err) => return ScriptResult::error(err.context("eval_file_path failed")),
        };

        if let Err(err) = client.save_svg(&path).await {
            return ScriptResult::error(err.context("save_svg failed"));
        }

        ScriptResult::ok_none()
    }
}

fn fit_to_len(text: impl ToString, len: usize) -> String {
    const SUFFIX: &str = "...";

    if len < SUFFIX.len() {
        panic!("len may not be less than {}, got {}", SUFFIX.len(), len);
    }

    let text = text.to_string();

    if text.len() <= len {
        return text;
    }

    text.chars()
        .take(len - SUFFIX.len())
        .chain(SUFFIX.chars())
        .collect()
}

macro_rules! unwrap_cmd_arg {
    (END FROM $args:ident) => {
        if $args.next().is_some() {
            return ScriptResult::error(anyhow!("got arg, expected none"));
        }
    };

    (IDENT $name:ident FROM $args:ident) => {
        let $name = match $args.next() {
            Some(CommandArg::Ident(ident)) => ident,
            Some(CommandArg::Expr(_)) => {
                return ScriptResult::error(anyhow!("got expr, ident expected"))
            }
            None => return ScriptResult::error(anyhow!("expected ident, got nothing")),
        };
    };

    (EXPR $name:ident FROM $args:ident) => {
        let $name = match $args.next() {
            Some(CommandArg::Expr(expr)) => expr,
            Some(CommandArg::Ident(_)) => {
                return ScriptResult::error(anyhow!("got ident, expr expected"))
            }
            None => return ScriptResult::error(anyhow!("expected expr, got nothing")),
        };
    };
}
use unwrap_cmd_arg;
