use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    events::{EventMetadata, TransactionRecorded, TransactionType},
    Aggregate, DomainError, Money, TransactionId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    id: TransactionId,
    amount: Money,
    transaction_type: TransactionType,
    description: String,
    recorded_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        transaction_id: Option<TransactionId>,
        amount: Money,
        transaction_type: TransactionType,
        description: String,
        recorded_at: Option<DateTime<Utc>>,
    ) -> Result<(Self, TransactionRecorded), DomainError> {
        if description.trim().is_empty() {
            return Err(DomainError::Validation("説明は空にできません".to_string()));
        }

        if amount.amount <= 0 {
            return Err(DomainError::Validation(
                "金額は正の数でなければいけません".to_string(),
            ));
        }

        let transaction_id = transaction_id.unwrap_or_default();
        let now = Utc::now();
        let recorded_at = recorded_at.unwrap_or(now);

        let event = TransactionRecorded {
            transaction_id,
            amount: amount.clone(),
            transaction_type: transaction_type.clone(),
            description: description.clone(),
            recorded_at,
            metadata: EventMetadata::new(None, None),
        };

        let transaction = Self {
            id: transaction_id,
            amount,
            transaction_type,
            recorded_at,
            description,
            created_at: now,
        };

        Ok((transaction, event))
    }

    pub fn id(&self) -> &TransactionId {
        &self.id
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn transaction_type(&self) -> &TransactionType {
        &self.transaction_type
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn recorded_at(&self) -> DateTime<Utc> {
        self.recorded_at
    }
}

impl Aggregate for Transaction {
    type Event = TransactionRecorded;
    fn id(&self) -> &super::EntityId {
        &self.id
    }
    fn apply(&mut self, event: &Self::Event) -> Result<(), DomainError> {
        self.id = event.transaction_id;
        self.amount = event.amount.clone();
        self.transaction_type = event.transaction_type.clone();
        self.description = event.description.clone();
        self.recorded_at = event.recorded_at;

        Ok(())
    }
}
