use axum::{Json, Router, routing::post};
use serde::Serialize;

#[derive(Serialize)]
struct AuthRes {
    token: &'static str,
}

async fn auth_handler() -> Json<AuthRes> {
    Json(AuthRes { token: "dummy" })
}

async fn connect_handler() -> &'static str {
    "connect: accepted"
}

async fn disconnect_handler() -> &'static str {
    "disconnect: accepted"
}

async fn reboot_handler() -> &'static str {
    "reboot: accepted"
}

pub fn build_router() -> Router {
    Router::new()
        .route("/auth", post(auth_handler))
        .route("/connect", post(connect_handler))
        .route("/disconnect", post(disconnect_handler))
        .route("/reboot", post(reboot_handler))
}
