use std::{collections::HashMap, sync::Arc};

use axum::{debug_handler, extract::State, routing::post, Json, Router};
use executor::exec::ExecScope;
use tokio::sync::Mutex;
use types::{
    api::{ApiError, EvalRequest, EvalRequestExpr, EvalResponse},
    core::Value,
    lang::Ident,
};

use crate::App;

pub fn router() -> Router<App> {
    Router::new().route("/", post(eval))
}

#[debug_handler(state = App)]
async fn eval(
    State(App { scope, .. }): State<App>,
    Json(EvalRequest { exprs }): Json<EvalRequest>,
) -> Json<EvalResponse> {
    async fn process_expr(
        expr: String,
        vars: HashMap<Ident, Value>,
        scope: Arc<Mutex<ExecScope>>,
    ) -> Result<Value, ApiError> {
        let expr = parser::expr(&expr)?;
        let value = scope.lock().await.eval_expr(expr, vars)?;
        Ok(value)
    }

    let mut values = Vec::with_capacity(exprs.len());

    for EvalRequestExpr { expr, vars } in exprs {
        values.push(process_expr(expr, vars, scope.clone()).await);
    }

    Json(EvalResponse { values })
}
