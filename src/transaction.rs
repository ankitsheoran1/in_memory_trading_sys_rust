use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::order_type::OrderId;
use crate::side::Side;

pub struct Transaction {

    pub transaction_id: Uuid,

    pub taker_order_id: OrderId,

    pub maker_order_id: OrderId,

    pub price: u64,

    pub quantity: u64,

    pub taker_side: Side,

    pub timestamp: u64,

}

impl Transaction {
    /// Create a new transaction
    pub fn new(
        transaction_id: Uuid,
        taker_order_id: OrderId,
        maker_order_id: OrderId,
        price: u64,
        quantity: u64,
        taker_side: Side,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        Self {
            transaction_id,
            taker_order_id,
            maker_order_id,
            price,
            quantity,
            taker_side,
            timestamp,
        }
    }

    pub fn maker_side(&self) -> Side {
        match self.taker_side {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }

    pub fn total_value(&self) -> u64 {
        self.price * self.quantity
    }
}


impl fmt::Display for Transaction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction ID: {}, Taker Order ID: {}, Maker Order ID: {}, Price: {}, Quantity: {}, Taker Side: {:?}, Timestamp: {}",
               self.transaction_id, self.taker_order_id.0, self.maker_order_id.0, self.price, self.quantity, self.taker_side, self.timestamp)
    }
}