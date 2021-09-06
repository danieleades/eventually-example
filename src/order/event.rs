use super::Item;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Created { id: String, at: DateTime<Utc> },
    ItemAdded { item: Item, at: DateTime<Utc> },
    Completed { at: DateTime<Utc> },
    Cancelled { at: DateTime<Utc> },
}
