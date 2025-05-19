mod api_errors;

use crate::api_errors::ApiError;
use axum::{Json, Router, routing::post};
use serde::Serialize;

#[derive(Serialize)]
struct AuthRes {
    token: &'static str,
}

async fn auth_handler() -> Result<Json<AuthRes>, ApiError> {
    // 本来は認証処理を行い、失敗時はErr(ApiError::Unauthorized)などを返す
    Ok(Json(AuthRes { token: "dummy" }))
}

async fn connect_handler() -> Result<&'static str, ApiError> {
    Ok("connect: accepted")
}

async fn disconnect_handler() -> Result<&'static str, ApiError> {
    Ok("disconnect: accepted")
}

async fn reboot_handler() -> Result<&'static str, ApiError> {
    Ok("reboot: accepted")
}

pub fn build_router() -> Result<Router, ApiError> {
    // ここで初期化等に失敗した場合はErr(ApiError::Internal)などを返す
    Ok(Router::new()
        .route("/auth", post(auth_handler))
        .route("/connect", post(connect_handler))
        .route("/disconnect", post(disconnect_handler))
        .route("/reboot", post(reboot_handler)))
}
