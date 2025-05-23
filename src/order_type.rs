use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::side::Side;
use uuid::Uuid;
use crate::order_error::OrderError;
use crate::time_in_force::TimeInForce;

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

impl OrderType {
    pub fn id(&self) -> OrderId {
        match self {
            Self::LimitOrder { id, .. } => *id,
            Self::MarketOrder { id, .. } => *id,
        }
    }

    pub fn price(&self) -> u64 {
        match self {
            Self::MarketOrder { price, ..} => *price,
            Self::LimitOrder { price,..} => *price

        }
    }

    pub fn quantity(&self) -> u64 {
        match self {
            Self::LimitOrder { quantity,..} => *quantity,
            Self::MarketOrder { quantity, ..} => *quantity,
        }
    }

    pub fn side(&self) -> Side {
        match self {
            Self::LimitOrder { side, ..} => *side,
            Self::MarketOrder { side, ..} => *side
        }
    }

    pub fn time_in_force(&self) -> TimeInForce {
        match self {
            Self::LimitOrder { time_in_force, ..} => *time_in_force,
            Self::MarketOrder { time_in_force, ..} => *time_in_force,
        }
    }

    /*
      match incoming order with current order
      // return quantity consumed of incoming
      // order (current if updated)
      // remaining quantity of current
     */

    pub fn  match_order(&self, incoming_qty: u64) -> ((u64, Option<Self>, u64)) {

        match self {
            OrderType::MarketOrder {
                quantity,
                id,
                price,
                side,
                timestamp,
                time_in_force
            } => {
                if incoming_qty < *quantity {

                        (
                        incoming_qty,
                        Some(Self::MarketOrder {
                                id: *id,
                                price: *price,
                                side: *side,
                                timestamp: *timestamp,
                                time_in_force: *time_in_force,
                                quantity: *quantity - incoming_qty
                            }),
                        0,
                        )
                } else {
                    (incoming_qty - *quantity, None, 0)
                }
            }
            OrderType::LimitOrder {
                quantity,
                id,
                price,
                side,
                timestamp,
                time_in_force
            } => {

                let order = Self::MarketOrder {
                    id: *id,
                    price: *price,
                    side: *side,
                    timestamp: *timestamp,
                    time_in_force: *time_in_force,
                    quantity: *quantity - incoming_qty
                };
                if incoming_qty < *quantity {

                        (
                            incoming_qty,
                            Some(order),
                            0,
                        )
                } else {
                    (incoming_qty - *quantity, None, 0)
                }
            }

        }
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
