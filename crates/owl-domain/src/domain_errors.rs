use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("authentication failed for key `{0}`")]
    AuthFailed(String),

    #[error("connection error: {0}")]
    ConnectionError(String),

    #[error("permission denied for operation")]
    PermissionDenied,
}
