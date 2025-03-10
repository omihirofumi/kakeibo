pub mod commands;
pub mod queries;
use thiserror::Error;

use crate::domain::DomainError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("ドメインエラー: {0}")]
    Domain(#[from] DomainError),
    #[error("データベースエラー: {0}")]
    Database(String),
    #[error("予期せぬエラー: {0}")]
    Unexpected(String),
}
