use crate::Connection;
use anyhow::Context;
use types::{api, core::Value};

impl Connection {
    pub async fn eval(&self, expr: impl ToString) -> anyhow::Result<Value> {
        let expr = parser::expr(&expr.to_string()).context("failed to parse expr")?;

        let resp = self.req(api::eval::Request { exprs: vec![expr] }).await?;
        assert_eq!(resp.values.len(), 1);
        let res = resp.values.into_iter().next().unwrap()?;
        Ok(res)
    }

    pub async fn eval_multi(
        &self,
        exprs: impl IntoIterator<Item = impl ToString>,
    ) -> anyhow::Result<Vec<anyhow::Result<Value>>> {
        let resp = self
            .req(api::eval::Request {
                exprs: exprs
                    .into_iter()
                    .map(|expr| parser::expr(&expr.to_string()))
                    .collect::<Result<_, _>>()
                    .context("failed to parse expr")?,
            })
            .await?;
        let res = resp
            .values
            .into_iter()
            .map(|value| value.map_err(anyhow::Error::from))
            .collect();
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn eval() {
        let con = Connection::new_test().await.unwrap();
        assert_eq!(con.eval("1 + 1").await.unwrap(), 2.into());
    }

    #[tokio::test]
    async fn eval_multi() {
        let con = Connection::new_test().await.unwrap();
        let mut res = con
            .eval_multi(["1 + 1", "2 * 2", "x + 1"])
            .await
            .unwrap()
            .into_iter();

        assert_eq!(res.next().unwrap().unwrap(), 2.into());
        assert_eq!(res.next().unwrap().unwrap(), 4.into());
        assert!(res.next().unwrap().is_err());
        assert!(res.next().is_none());
    }
}
