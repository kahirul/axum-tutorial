#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    client.do_get("/hello?name=Alice").await?.print().await?;

    client.do_get("/src/main.rs").await?.print().await?;

    let login_request = client.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "Secret"
        }),
    );
    login_request.await?.print().await?;

    client.do_get("/halo/Budi").await?.print().await?;

    let create_ticket_request = client.do_post(
        "/api/tickets",
        json!({
            "title": "Learning Axum REST"
        }),
    );
    create_ticket_request.await?.print().await?;

    client.do_get("/api/tickets").await?.print().await?;

    client.do_delete("/api/tickets/0").await?.print().await?;
    client.do_delete("/api/tickets/0").await?.print().await?;

    client.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
