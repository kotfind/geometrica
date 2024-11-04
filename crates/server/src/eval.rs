use std::{collections::HashMap, sync::Arc};

use axum::{debug_handler, extract::State, routing::post, Json, Router};
use executor::exec::ExecScope;
use tokio::sync::Mutex;
use types::{api, core::Ident, core::Value};

use crate::App;

pub fn router() -> Router<App> {
    Router::new().route("/", post(eval))
}

#[debug_handler(state = App)]
async fn eval(
    State(App { scope, .. }): State<App>,
    Json(api::eval::Request { exprs }): Json<api::eval::Request>,
) -> Json<api::eval::Response> {
    async fn process_expr(
        expr: String,
        vars: HashMap<Ident, Value>,
        scope: Arc<Mutex<ExecScope>>,
    ) -> Result<Value, api::Error> {
        let expr = parser::expr(&expr)?;
        let value = scope.lock().await.eval_expr(expr, vars)?;
        Ok(value)
    }

    let mut values = Vec::with_capacity(exprs.len());

    for api::eval::RequestExpr { expr, vars } in exprs {
        values.push(process_expr(expr, vars, scope.clone()).await);
    }

    Json(api::eval::Response { values })
}
