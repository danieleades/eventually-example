use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub item_sku: String,
    pub quantity: u8,
    pub price: f32,
}
