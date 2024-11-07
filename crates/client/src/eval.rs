use crate::Connection;
use types::{api, core::Value};

impl Connection {
    pub async fn eval(&self, expr: impl ToString) -> anyhow::Result<Value> {
        let resp = self
            .req(api::eval::Request {
                exprs: vec![api::eval::RequestExpr {
                    expr: expr.to_string(),
                    vars: [].into(),
                }],
            })
            .await?;
        assert_eq!(resp.values.len(), 1);
        let res = resp.values.into_iter().next().unwrap()?;
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn simple() {
        let con = Connection::new_test().await.unwrap();
        assert_eq!(con.eval("1 + 1").await.unwrap(), 2.into());
    }
}
