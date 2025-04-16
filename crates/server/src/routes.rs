use std::collections::HashMap;

use axum::{http::StatusCode, routing::post, Router};
use executor::exec::Exec;
use types::api::Error;

use crate::{
    result::{api_err, api_err_no_result, api_ok, IntoError},
    App,
};

pub fn router() -> Router<App> {
    let mut router = Router::new().route("/ping", post(StatusCode::OK));

    route!(INTO router INSERT
        ROUTE (clear)() SCOPE scope {
            scope.clear();
            api_ok(R {})
        }

        ROUTE (eval)(exprs) SCOPE scope {
            let mut values = Vec::with_capacity(exprs.len());

            for expr in exprs {
                values.push(
                    scope
                        .eval_expr(expr, HashMap::new())
                        .map_err(IntoError::into_error),
                );
            }

            api_ok(R { values })
        }

        ROUTE (exec)(defs) SCOPE scope {
            defs.exec(&mut scope).map_err(api_err_no_result)?;
            api_ok(R {})
        }

        ROUTE (func::list)() SCOPE scope {
            let (builtins, user_defined) = scope.list_funcs();
            api_ok(R { builtins, user_defined, })
        }

        ROUTE (items::get_all)() SCOPE scope {
            let items = scope.get_all_items();
            api_ok(R { items })
        }

        ROUTE (items::get)(name) SCOPE scope {
            let item = scope.get_item(&name);
            match item {
                Some(value) => api_ok(R { value }),
                None => api_err(Error {
                    msg: format!("item {name} not found"),
                }),
            }
        }

        ROUTE (rm)(name) SCOPE scope {
            scope.rm(name).map_err(api_err_no_result)?;
            api_ok(R {})
        }

        ROUTE (set)(name, expr) SCOPE scope {
            let value = scope
                .eval_expr(expr, HashMap::new())
                .map_err(api_err_no_result)?;

            scope.set(&name, value).map_err(api_err_no_result)?;

            api_ok(R {})
        }
    );

    router
}

macro_rules! route {
    (INTO $router:ident INSERT) => {};

    (INTO $router:ident INSERT
        ROUTE
        // The route path, a submodule of types::api
        ($api_route:path)

        // Args, extracted from request
        ($($arg:ident),*)

        // Locked scope
        $(SCOPE $scope:ident)?

        // The api response type is provided to body as R
        $body:block

        // The following routes
        $($rest:tt)*
    ) => {
        paste::paste! {{
            #[axum::debug_handler(state = App)]
            async fn route(
                axum::extract::State(crate::App { scope, .. }): axum::extract::State<crate::App>,
                axum::Json(types::api::$api_route::Request { $($arg),* }): axum::Json<types::api::$api_route::Request>,
            ) -> crate::result::ApiResult<types::api::$api_route::Response> {
                $(
                    #[allow(unused_mut)]
                    let mut $scope = scope.lock().await;
                )?

                #[allow(unused_imports)]
                use types::api::$api_route::Response as R;
                $body
            }

            $router = $router.route(types::api::$api_route::ROUTE, axum::routing::post(route));
        }}

        route!(INTO $router INSERT $($rest)*);
    };
}
use route;
