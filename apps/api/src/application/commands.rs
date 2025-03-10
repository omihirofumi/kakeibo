use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::{events::TransactionType, Money, TransactionId};

use super::ApplicationError;

pub struct CreateTransactionCommand {
    pub amount: Money,
    pub transaction_type: TransactionType,
    pub description: String,
    pub recorded_at: Option<DateTime<Utc>>,
}

pub struct CreateTransactionHandler {
    // TODO: イベントストアの依存関係を追加
}

impl CreateTransactionHandler {
    pub async fn handle(
        &self,
        command: CreateTransactionCommand,
    ) -> Result<TransactionId, ApplicationError> {
        todo!("トランザクション作成の実装")
    }
}
