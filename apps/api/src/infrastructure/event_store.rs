use std::future::Future;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::{DomainEvent, EntityId};

#[derive(Debug, Error)]
pub enum EventStoreError {
    #[error("シリアライゼーションエラー: {0}")]
    Serialization(String),
    #[error("デシリアライゼーションエラー: {0}")]
    Deserialization(String),
    #[error("イベントが見つかりません: {0}")]
    NotFound(String),
    #[error("予期せぬエラー: {0}")]
    Unexpected(String),
}

pub trait EventStore: Send + Sync {
    async fn save_event<E>(&self, event: E) -> Result<(), EventStoreError>
    where
        E: DomainEvent + Serialize + Send + 'static;

    async fn get_events_for_aggregate<E>(
        &self,
        aggregate_id: EntityId,
    ) -> Result<Vec<E>, EventStoreError>
    where
        E: DomainEvent + for<'de> Deserialize<'de> + Send + 'static;
}

// pub mod memory {
//     use super::*;
//     use std::collections::HashMap;
//     use std::sync::{Arc, Mutex};
//
//     #[derive(Default, Clone)]
//     pub struct InMemoryEventStore {
//         events: Arc<Mutex<HashMap<EntityId, Vec<String>>>>,
//     }
//
//     impl InMemoryEventStore {
//         pub fn new() -> Self {
//             Self {
//                 events: Arc::new(Mutex::new(HashMap::new())),
//             }
//         }
//     }
//
//     impl EventStore for InMemoryEventStore {
//         fn save_event<E>(
//             &self,
//             event: E,
//         ) -> impl Future<Output = Result<(), EventStoreError>> + Send
//         where
//             E: DomainEvent + Serialize + Send + 'static,
//         {
//             let events = self.events.clone();
//
//             async move {
//                 let aggreate_id = *event.aggregate_id();
//             }
//         }
//     }
// }
