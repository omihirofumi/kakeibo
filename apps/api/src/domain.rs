pub mod events;
pub mod transaction;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

// 各EntityのIDの型定義
pub type EntityId = Ulid;
pub type UserId = EntityId;
pub type AccountId = EntityId;
pub type CategoryId = EntityId;
pub type TransactionId = EntityId;
pub type BudgetId = EntityId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Money {
    pub amount: i64,
    pub currecy: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Currency {
    JPY,
    USD,
}

pub trait DomainEvent {
    fn event_type(&self) -> &str;
    fn aggregate_id(&self) -> &EntityId;
    fn occurred_at(&self) -> DateTime<Utc>;
}

pub trait Aggregate {
    type Event: DomainEvent;

    fn id(&self) -> &EntityId;
    fn apply(&mut self, event: &Self::Event) -> Result<(), DomainError>;
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("検証エラー: {0}")]
    Validation(String),

    #[error("エンティティが見つかりません: {0}")]
    NotFound(String),

    #[error("アカウントの残高不足")]
    InsufficientFunds,

    #[error("予算の上限超過")]
    BudgetLimitExceeded,

    #[error("予期しなエラー: {0}")]
    Unexpected(String),
}
