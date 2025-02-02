use anyhow::Context;
use parser::ParseInto;
use types::lang::Statement;

use crate::{Client, ScriptResult};

impl Client {
    /// Parses and executes script. Returns a table-result for each command.
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

    /// Parses and executes a statement. Returns a table-result if statement was a command.
    pub async fn exec_one(&self, script: impl ParseInto<Statement>) -> ScriptResult {
        let stmt = match script.parse_into().context("failed to parse script") {
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
}

// TODO: add tests
