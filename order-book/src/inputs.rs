use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct CreateOrderInput {
    pub price: f64,
    pub quantity: f64,
    pub user_id: String,
    pub side: Side
}

#[derive(Deserialize, Debug)]
pub enum Side {
    Buy,
    Sell
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrder {
    pub order_id: String,
    pub user_id: String,
}