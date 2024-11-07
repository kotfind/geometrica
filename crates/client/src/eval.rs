use crate::Connection;
use types::{
    api,
    core::{Ident, Value},
};

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

    pub async fn eval_with_vars(
        &self,
        expr: impl ToString,
        vars: impl IntoIterator<Item = (impl Into<Ident>, impl Into<Value>)>,
    ) -> anyhow::Result<Value> {
        let resp = self
            .req(api::eval::Request {
                exprs: vec![api::eval::RequestExpr {
                    expr: expr.to_string(),
                    vars: vars
                        .into_iter()
                        .map(|(name, value)| (name.into(), value.into()))
                        .collect(),
                }],
            })
            .await?;
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
                    .map(|expr| api::eval::RequestExpr {
                        expr: expr.to_string(),
                        vars: [].into(),
                    })
                    .collect(),
            })
            .await?;
        let res = resp
            .values
            .into_iter()
            .map(|value| value.map_err(anyhow::Error::from))
            .collect();
        Ok(res)
    }

    pub async fn eval_multi_with_vars(
        &self,
        exprs: impl IntoIterator<
            Item = (
                impl ToString,
                impl IntoIterator<Item = (impl Into<Ident>, impl Into<Value>)>,
            ),
        >,
    ) -> anyhow::Result<Vec<anyhow::Result<Value>>> {
        let resp = self
            .req(api::eval::Request {
                exprs: exprs
                    .into_iter()
                    .map(|(expr, vars)| api::eval::RequestExpr {
                        expr: expr.to_string(),
                        vars: vars
                            .into_iter()
                            .map(|(name, value)| (name.into(), value.into()))
                            .collect(),
                    })
                    .collect(),
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
    async fn eval_with_vars() {
        let con = Connection::new_test().await.unwrap();
        assert_eq!(
            con.eval_with_vars("x + 1", [("x", 2)]).await.unwrap(),
            3.into()
        );
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

    #[tokio::test]
    async fn eval_multi_with_vars() {
        let con = Connection::new_test().await.unwrap();
        let mut res = con
            .eval_multi_with_vars([
                ("x + 1", vec![("x", 1)]),
                ("x * y", vec![("x", 7), ("y", 6)]),
                ("x + y + z", vec![("x", 1), ("y", 2), ("z", 3)]),
            ])
            .await
            .unwrap()
            .into_iter();

        assert_eq!(res.next().unwrap().unwrap(), 2.into());
        assert_eq!(res.next().unwrap().unwrap(), 42.into());
        assert_eq!(res.next().unwrap().unwrap(), 6.into());
        assert!(res.next().is_none());
    }
}
