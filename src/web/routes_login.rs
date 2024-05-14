use crate::{web, Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<16} - api_login", "handler");

    // TODO: DB integration

    if payload.username != "admin" || payload.password != "Secret" {
        return Err(Error::LoginFailed);
    }

    // FIXME: Implement real auth-token
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
