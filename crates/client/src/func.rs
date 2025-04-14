use anyhow::Context;
use types::{api, lang::FunctionSignature};

use crate::Client;

impl Client {
    // Returns a result of a pair of two `Vec<FunctionSignature>`.
    // The first one contains built-in functions,
    // the second one contains user-defined functions.
    pub async fn list_funcs(
        &self,
    ) -> anyhow::Result<(Vec<FunctionSignature>, Vec<FunctionSignature>)> {
        let resp = self
            .req(api::func::list::Request {})
            .await
            .context("failed to get functions")?;
        Ok((resp.builtins, resp.user_defined))
    }
}
