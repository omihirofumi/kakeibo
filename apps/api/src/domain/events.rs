use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use super::{DomainEvent, Money, TransactionId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub event_id: Ulid,
    pub occurred_at: DateTime<Utc>,
}

impl EventMetadata {
    pub fn new(event_id: Option<Ulid>, occurred_at: Option<DateTime<Utc>>) -> Self {
        Self {
            event_id: event_id.unwrap_or_default(),
            occurred_at: occurred_at.unwrap_or_default(),
        }
    }

    pub fn event_id(&self) -> &Ulid {
        &self.event_id
    }

    pub fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

// トランザクションのタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Expense, // 支出
    Income,  // 収入
}

pub struct TransactionRecorded {
    pub transaction_id: TransactionId,
    pub amount: Money,
    pub transaction_type: TransactionType,
    pub description: String,
    pub recorded_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl DomainEvent for TransactionRecorded {
    fn event_type(&self) -> &str {
        "transaction.recorded"
    }
    fn aggregate_id(&self) -> &super::EntityId {
        &self.transaction_id
    }
    fn occurred_at(&self) -> DateTime<Utc> {
        self.metadata.occurred_at
    }
}
