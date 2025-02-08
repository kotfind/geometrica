use crate::Client;
use anyhow::Context;
use parser::ParseInto;
use types::{api, core::Value, lang::Expr};

impl Client {
    pub async fn eval_one(&self, expr: impl ParseInto<Expr>) -> anyhow::Result<Value> {
        let expr = expr.parse_into().context("failed to parse expr")?;

        let resp = self.req(api::eval::Request { exprs: vec![expr] }).await?;
        assert_eq!(resp.values.len(), 1);
        let res = resp.values.into_iter().next().unwrap()?;
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
    use crate::test_utils::TestClient;

    #[tokio::test]
    async fn eval() {
        let con = TestClient::new().await;
        assert_eq!(con.eval_one("1 + 1").await.unwrap(), 2.into());
    }

    #[tokio::test]
    async fn eval_multi() {
        let con = TestClient::new().await;
        let mut res = con
            .eval(["1 + 1", "2 * 2", "x + 1"])
            .await
            .unwrap()
            .into_iter();

        assert_eq!(res.next().unwrap().unwrap(), 2.into());
        assert_eq!(res.next().unwrap().unwrap(), 4.into());
        assert!(res.next().unwrap().is_err());
        assert!(res.next().is_none());
    }
}
