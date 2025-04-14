use std::collections::HashMap;

use anyhow::Context;
use parser::ParseInto;
use reqwest::Url;
use types::{
    api,
    core::{Ident, Value},
    lang::{Definition, Expr, FunctionSignature, Statement},
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
}

#[cfg(test)]
mod test {
    mod define {
        use types::core::Ident;

        use crate::test_utils::TestClient;

        #[tokio::test]
        async fn simple() {
            let con = TestClient::new().await;

            con.define(
                r#"
            x = 1
            y = 2
            z = x + y
        "#,
            )
            .await
            .unwrap();

            let items = con.get_all_items().await.unwrap();

            assert!(items.len() == 3);
            assert!(items[&Ident::from("x")] == 1.into());
            assert!(items[&Ident::from("y")] == 2.into());
            assert!(items[&Ident::from("z")] == 3.into());
        }

        #[tokio::test]
        async fn with_funcs() {
            let con = TestClient::new().await;

            con.define(
                r#"
                sq x:int -> int = x^2
                sq x:real -> real = x^2.0
                sum x:int y:int -> int = x + y
                a = 1
                b = 2
                c = sum (sq a) (sq b)
        "#,
            )
            .await
            .unwrap();

            let items = con.get_all_items().await.unwrap();

            assert!(items.len() == 3);
            assert!(items[&Ident::from("a")] == 1.into());
            assert!(items[&Ident::from("b")] == 2.into());
            assert!(items[&Ident::from("c")] == 5.into());
        }

        #[tokio::test]
        async fn multiple_requests() {
            let con = TestClient::new().await;

            con.define(
                r#"
            sq x:int -> int = x^2
            sq x:real -> real = x^2.0
            sum x:int y:int -> int = x + y
        "#,
            )
            .await
            .unwrap();

            con.define(
                r#"
            a = 1
            b = 2
            c = sum (sq a) (sq b)
        "#,
            )
            .await
            .unwrap();

            let items = con.get_all_items().await.unwrap();

            assert!(items.len() == 3);
            assert!(items[&Ident::from("a")] == 1.into());
            assert!(items[&Ident::from("b")] == 2.into());
            assert!(items[&Ident::from("c")] == 5.into());
        }
    }

    mod eval {
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

    mod items {
        use crate::test_utils::TestClient;
        use types::core::Ident;

        #[tokio::test]
        async fn get_all() {
            let con = TestClient::new().await;

            con.define(
                r#"
            x = 1
            y = 2
            z = x + y
        "#,
            )
            .await
            .unwrap();

            let items = con.get_all_items().await.unwrap();

            assert!(items.len() == 3);
            assert!(items[&Ident::from("x")] == 1.into());
            assert!(items[&Ident::from("y")] == 2.into());
            assert!(items[&Ident::from("z")] == 3.into());
        }

        #[tokio::test]
        async fn get_item() {
            let con = TestClient::new().await;

            con.define(
                r#"
            x = 1
            y = 2
            z = x + y
        "#,
            )
            .await
            .unwrap();

            assert!(con.get_item("x").await.unwrap() == 1.into());
            assert!(con.get_item("y").await.unwrap() == 2.into());
            assert!(con.get_item("z").await.unwrap() == 3.into());
            assert!(con.get_item("t").await.is_err());
        }
    }

    mod rm {
        use crate::test_utils::TestClient;
        use types::core::Ident;

        #[tokio::test]
        async fn rm() {
            let client = TestClient::new().await;

            client
                .define(
                    r#"
                x = 1.0
                y = 2.0 * x
                z = 3.0
                p1 = pt x y
                p2 = pt 1.0 z
            "#,
                )
                .await
                .unwrap();

            client.rm("x").await.unwrap();

            let items = client.get_all_items().await.unwrap();

            assert!(items.len() == 2);
            assert!(items.contains_key(&Ident::from("z")));
            assert!(items.contains_key(&Ident::from("p2")));
        }
    }

    mod set {
        use crate::test_utils::TestClient;

        #[tokio::test]
        async fn set() {
            let client = TestClient::new().await;

            client
                .define(
                    r#"
                x = 2
                y = 3
                z = x^2 + y^2
            "#,
                )
                .await
                .unwrap();

            assert_eq!(client.get_item("z").await.unwrap(), 13.into());

            client.set("x", "5").await.unwrap();
            client.set("y", "7").await.unwrap();

            assert_eq!(client.get_item("z").await.unwrap(), 74.into());
        }
    }
}
