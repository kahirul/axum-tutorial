#![allow(unused)]

pub use self::error::{Error, Result};

use crate::ctx::Ctx;
use crate::log::log_request;
use crate::model::ModelController;
use axum::extract::{MatchedPath, Path, Query, RawPathParams};
use axum::http::{Method, Uri};
use axum::middleware::from_fn;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service, Route};
use axum::{middleware, Json, Router, ServiceExt};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let model_controller = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(model_controller.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            model_controller.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("->> Listening on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    request_method: Method,
    response: Response,
) -> Response {
    println!("->> {:<16} - main_response_mapper", "response_mapper");

    let uuid = Uuid::new_v4();

    let service_error = response.extensions().get::<Error>();

    let client_status_error = service_error.map(|e| e.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "requestId": uuid.to_string(),
                }
            });

            println!("    ->> client_error_body: {client_error_body}");
            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    log_request(uuid, request_method, uri, ctx, service_error, client_error).await;

    println!();

    error_response.unwrap_or(response)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/halo/:name", get(handler_halo))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<16} - handler_hello {params:?}", "handler");
    let name = params.name.as_deref().unwrap_or("World");
    {
        Html(format!("Hello <strong>{name}!</strong>"))
    }
}

async fn handler_halo(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<16} - handler_halo name: {name:?}", "handler");
    {
        Html(format!("Halo <strong>{name}!</strong>"))
    }
}
