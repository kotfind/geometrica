use std::{collections::HashMap, path::Path};

use anyhow::Context;
use parser::ParseInto;
use reqwest::Url;
use types::{
    api::{self, FunctionList},
    core::{Ident, Value},
    lang::{Definition, Expr, Statement},
};

use crate::ScriptResult;

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) server_url: Url,
    pub(crate) client: reqwest::Client,
}

impl Client {
    pub async fn clear(&self) -> anyhow::Result<()> {
        self.req(api::clear::Request {})
            .await
            .context("failed to clear")?;

        Ok(())
    }

    pub async fn define_one(&self, def: impl ParseInto<Definition>) -> anyhow::Result<()> {
        self.req(api::exec::Request {
            defs: vec![def.parse_into().context("failed to parse definition")?],
        })
        .await
        .context("define failed")?;

        Ok(())
    }

    pub async fn define(&self, defs: impl ParseInto<Vec<Definition>>) -> anyhow::Result<()> {
        self.req(api::exec::Request {
            defs: defs.parse_into().context("failed to parse definitions")?,
        })
        .await
        .context("define failed")?;

        Ok(())
    }

    pub async fn eval_one(&self, expr: impl ParseInto<Expr>) -> anyhow::Result<Value> {
        let expr = expr.parse_into().context("failed to parse expr")?;

        let resp = self
            .req(api::eval::Request { exprs: vec![expr] })
            .await
            .context("failed to eval expr")?;
        assert_eq!(resp.values.len(), 1);
        let res = resp
            .values
            .into_iter()
            .next()
            .unwrap()
            .context("evaluation failed")?;
        Ok(res)
    }

    pub async fn eval(
        &self,
        exprs: impl IntoIterator<Item = impl ParseInto<Expr>>,
    ) -> anyhow::Result<Vec<anyhow::Result<Value>>> {
        let resp = self
            .req(api::eval::Request {
                exprs: exprs
                    .into_iter()
                    .map(|expr| expr.parse_into().context("failed to parse expr"))
                    .collect::<Result<_, _>>()?,
            })
            .await
            .context("failed to eval exprs")?;
        let res = resp
            .values
            .into_iter()
            .map(|value| value.map_err(anyhow::Error::from))
            .collect();
        Ok(res)
    }

    /// Parses and executes the script.
    pub async fn exec(&self, script: impl ParseInto<Vec<Statement>>) -> ScriptResult {
        let script = match script.parse_into().context("failed to parse script") {
            Ok(x) => x,
            Err(e) => return ScriptResult::error(e),
        };

        let mut ans = Vec::new();

        for stmt in script {
            let res = self.exec_one(stmt).await;
            ans.extend(res.results);

            if let Some(err) = res.error {
                return ScriptResult::partail_error(ans, err);
            }
        }

        ScriptResult::ok(ans)
    }

    /// Parses and executes the statement.
    pub async fn exec_one(&self, stmt: impl ParseInto<Statement>) -> ScriptResult {
        let stmt = match stmt.parse_into().context("failed to parse script") {
            Ok(x) => x,
            Err(e) => return ScriptResult::error(e),
        };

        match stmt {
            Statement::Definition(def) => {
                match self.define_one(def).await.context("define failed") {
                    Ok(()) => ScriptResult::ok_none(),
                    Err(err) => ScriptResult::error(err),
                }
            }

            Statement::Command(cmd) => self.command(cmd).await.context("failed to execute command"),
        }
    }

    pub async fn list_funcs(&self) -> anyhow::Result<FunctionList> {
        let resp = self
            .req(api::func::list::Request {})
            .await
            .context("failed to get functions")?;
        Ok(resp.func_list)
    }

    pub async fn get_all_items(&self) -> anyhow::Result<HashMap<Ident, Value>> {
        let resp = self
            .req(api::items::get_all::Request {})
            .await
            .context("failed to get all items")?;

        Ok(resp.items)
    }

    pub async fn get_item(&self, name: impl Into<Ident>) -> anyhow::Result<Value> {
        let name = name.into();
        let resp = self
            .req(api::items::get::Request { name: name.clone() })
            .await
            .context(format!("failed to get '{name}'"))?;

        Ok(resp.value)
    }

    pub async fn rm(&self, name: impl Into<Ident>) -> anyhow::Result<()> {
        let name = name.into();
        self.req(api::rm::Request { name: name.clone() })
            .await
            .context(format!("failed to rm '{name}'"))?;

        Ok(())
    }

    pub async fn set(
        &self,
        name: impl Into<Ident>,
        expr: impl ParseInto<Expr>,
    ) -> anyhow::Result<()> {
        let name = name.into();
        let expr = expr.parse_into().context("failed to parse expr")?;
        self.req(api::set::Request {
            name: name.clone(),
            expr: expr.clone(),
        })
        .await
        .context(format!("failed to set '{name}' to '{expr}'"))?;

        Ok(())
    }

    pub async fn load_json(&self, json: impl ToString) -> anyhow::Result<()> {
        self.req(api::json::load::Request {
            json: json.to_string(),
        })
        .await
        .context("failed to load from json")?;

        Ok(())
    }

    pub async fn dump_json(&self) -> anyhow::Result<String> {
        let resp = self
            .req(api::json::dump::Request {})
            .await
            .context("failed to dump to json")?;
        Ok(resp.json)
    }

    pub async fn save(&self, file: &Path) -> anyhow::Result<()> {
        let json = self.dump_json().await.context("dump_json failed")?;
        tokio::fs::write(file, json.as_bytes())
            .await
            .context("failed to write to file")?;
        Ok(())
    }

    pub async fn load(&self, file: &Path) -> anyhow::Result<()> {
        let json = tokio::fs::read_to_string(file)
            .await
            .context("failed to read file")?;
        self.load_json(json).await.context("load_json failed")?;
        Ok(())
    }

    pub async fn dump_svg(&self) -> anyhow::Result<String> {
        let resp = self
            .req(api::svg::dump::Request {})
            .await
            .context("failed to dump to svg")?;
        Ok(resp.svg)
    }

    pub async fn save_svg(&self, file: &Path) -> anyhow::Result<()> {
        let svg = self.dump_svg().await.context("dump_svg failed")?;
        tokio::fs::write(file, svg.as_bytes())
            .await
            .context("failed to write to file")?;
        Ok(())
    }
}
