use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("認証エラー: 無効なトークン")]
    Unauthorized,
    #[error("リクエストが不正です")]
    BadRequest,
    #[error("内部サーバーエラー")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}
