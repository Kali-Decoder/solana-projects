use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::inputs::{CreateOrderInput, DeleteOrder, Side};



pub struct OpenOrder {
    pub price: f64,
    pub quantity: f64,
    pub user_id: String,
    pub side: Side,
    pub order_id: String,
    pub filled_quantity: f64,
}

pub struct Orderbook {
    pub bids: HashMap<String, Vec<OpenOrder>>,
    pub asks: HashMap<String, Vec<OpenOrder>>,
    pub order_id_index: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Depth {
    pub price: f64,
    pub quantity: f64,
}

pub struct DepthResponse {
    pub bids: Vec<Depth>,
    pub asks: Vec<Depth>,
}

impl Default for Orderbook {
    fn default() -> Self {
        Self {
            bids: HashMap::new(),
            asks: HashMap::new(),
            order_id_index: 0,
        }
    }
}

impl Orderbook {
    pub fn create_order(&mut self, order: CreateOrderInput) {
        let order_id = self.order_id_index.to_string();
        self.order_id_index += 1;
        match order.side {
            Side::Buy => {
                let open_order = OpenOrder {
                    price: order.price,
                    quantity: order.quantity,
                    side: order.side,
                    user_id: order.user_id,
                    order_id: order_id,
                    filled_quantity: 0.0,
                };
                self.bids
                    .entry(order.price.to_string())
                    .or_insert(Vec::new())
                    .push(open_order);
            }

            Side::Sell => {
                let open_order = OpenOrder {
                    price: order.price,
                    quantity: order.quantity,
                    side: order.side,
                    user_id: order.user_id,
                    order_id: order_id,
                    filled_quantity: 0.0,
                };
                self.asks
                    .entry(order.price.to_string())
                    .or_insert(Vec::new())
                    .push(open_order);
            }
        }
    }
    pub fn delete_order(&mut self, order: DeleteOrder) {
        // Find and remove from bids
        if let Some(price) = self.bids.iter().find_map(|(price, orders)| {
            if orders.iter().any(|o| o.order_id == order.order_id) {
                Some(price.clone())
            } else {
                None
            }
        }) {
            if let Some(orders) = self.bids.get_mut(&price) {
                orders.retain(|o| o.order_id != order.order_id);
            }
        }

        // Find and remove from asks
        if let Some(price) = self.asks.iter().find_map(|(price, orders)| {
            if orders.iter().any(|o| o.order_id == order.order_id) {
                Some(price.clone())
            } else {
                None
            }
        }) {
            if let Some(orders) = self.asks.get_mut(&price) {
                orders.retain(|o| o.order_id != order.order_id);
            }
        }
    }
    pub fn get_depth(&self) -> DepthResponse {
        let mut bids = Vec::new();
        let mut asks = Vec::new();
        for (price, orders) in self.bids.iter() {
            bids.push(Depth { price: price.parse().unwrap(), quantity: orders.iter().map(|o| o.quantity).sum() });
        }
        for (price, orders) in self.asks.iter() {
            asks.push(Depth { price: price.parse().unwrap(), quantity: orders.iter().map(|o| o.quantity).sum() });  
        }
        DepthResponse { bids, asks }
    }
}
