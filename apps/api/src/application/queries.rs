use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::{events::TransactionType, Money, TransactionId};

use super::ApplicationError;

#[derive(Debug, Serialize)]
pub struct TransactionDto {
    pub id: TransactionId,
    pub amount: Money,
    pub transaction_type: TransactionType,
    pub description: String,
    pub recorded_at: DateTime<Utc>,
}

pub struct GetTransactionQuery {
    pub id: TransactionId,
}

pub struct GetTransactionHandler {
    // TODO: 依存関係追加
}

impl GetTransactionHandler {
    pub async fn handle(
        &self,
        query: GetTransactionQuery,
    ) -> Result<Option<TransactionDto>, ApplicationError> {
        todo!("トランザクション取得実装")
    }
}
