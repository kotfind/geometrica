use anyhow::Context;
use parser::ParseInto;
use types::lang::Statement;

use crate::{table::Table, Client};

impl Client {
    /// Parses and executes script. Returns a table-result for each command.
    pub async fn exec(&self, script: impl ParseInto<Vec<Statement>>) -> anyhow::Result<Vec<Table>> {
        let script = script.parse_into().context("failed to parse script")?;
        let mut ans = Vec::new();

        for stmt in script {
            if let Some(res) = self.exec_one(stmt).await? {
                ans.push(res);
            }
        }

        Ok(ans)
    }

    /// Parses and executes a statement. Returns a table-result if statement was a command.
    pub async fn exec_one(
        &self,
        script: impl ParseInto<Statement>,
    ) -> anyhow::Result<Option<Table>> {
        let stmt = script.parse_into().context("failed to parse script")?;

        Ok(match stmt {
            Statement::Definition(def) => {
                self.define_one(def).await.context("define failed")?;
                None
            }

            Statement::Command(cmd) => {
                let res = self
                    .command(cmd)
                    .await
                    .context("failed to execute command")?;
                Some(res)
            }
        })
    }
}
