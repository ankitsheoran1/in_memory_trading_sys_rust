use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::side::Side;
use uuid::Uuid;
use crate::order_error::OrderError;

pub enum OrderType {

    MarketOrder {
        id: OrderId,
        price: u64,
        quantity: u64,
        side: Side,
        timestamp: u64,
        time_in_force: TimeInForce,
    },

    LimitOrder {
        id: OrderId,
        price: u64,
        quantity: u64,
        side: Side,
        timestamp: u64,
        time_in_force: TimeInForce,
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderId(pub Uuid);

impl FromStr for OrderId {
    type Err = OrderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Uuid::from_str(s) {
            Ok(id) => Ok(OrderId(id)),
            Err(e) => Err(OrderError::ParseError {
                message: format!("Failed to parse OrderId: {}", e),
            }),
        }
    }
}
