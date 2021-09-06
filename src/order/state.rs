use super::Item;
use chrono::{DateTime, Utc};
use derive_more::From;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order<S> {
    #[serde(skip_serializing)]
    id: String,
    created_at: DateTime<Utc>,
    items: Vec<Item>,
    state: S,
}

impl<S> Order<S> {
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
}

impl Order<Editable> {
    pub fn create(id: String, at: DateTime<Utc>) -> Self {
        let items = Vec::default();
        let state = Editable { updated_at: at };

        Self {
            id,
            created_at: at,
            items,
            state,
        }
    }

    pub fn add_item(&mut self, item: Item, at: DateTime<Utc>) {
        self.items.push(item);
        self.state.updated_at = at;
    }

    pub fn complete(self, at: DateTime<Utc>) -> Order<Complete> {
        let state = Complete { at };
        Order {
            id: self.id,
            created_at: self.created_at,
            items: self.items,
            state,
        }
    }

    pub fn cancel(self, at: DateTime<Utc>) -> Order<Cancelled> {
        let state = Cancelled { at };
        Order {
            id: self.id,
            created_at: self.created_at,
            items: self.items,
            state,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub enum State {
    NotStarted,
    Editable(Order<Editable>),
    Complete(Order<Complete>),
    Cancelled(Order<Cancelled>),
}

impl Default for State {
    fn default() -> Self {
        Self::NotStarted
    }
}

impl State {
    pub fn create(id: String, at: DateTime<Utc>) -> Self {
        Self::Editable(Order::create(id, at))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Editable {
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Complete {
    at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cancelled {
    at: DateTime<Utc>,
}
