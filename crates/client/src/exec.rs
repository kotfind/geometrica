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
            match stmt {
                Statement::Definition(def) => {
                    self.define(def).await.context("define failed")?;
                }

                Statement::Command(cmd) => {
                    let res = self
                        .command(cmd)
                        .await
                        .context("failed to execute command")?;
                    ans.push(res);
                }
            }
        }

        Ok(ans)
    }
}
