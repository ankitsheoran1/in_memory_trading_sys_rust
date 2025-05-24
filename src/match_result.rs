use serde::{Deserialize, Serialize};
use crate::order_type::OrderId;
use crate::transaction::Transaction;

pub struct MatchResult {
    /// The ID of the incoming order that initiated the match
    pub order_id: OrderId,

    pub remaining_quantity: u64,

    pub transactions: Vec<Transaction>,

    pub is_complete: bool,

    pub filled_order_ids: Vec<OrderId>,
}


impl MatchResult {
    pub fn new(order_id: OrderId, remaining_quantity: u64) -> Self {
        MatchResult {
            order_id,
            remaining_quantity,
            transactions: Vec::new(),
            is_complete: false,
            filled_order_ids: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.remaining_quantity = self.remaining_quantity.saturating_sub(transaction.quantity);
        self.is_complete = self.remaining_quantity == 0;
        self.transactions.push(transaction);
    }

    pub fn add_filled_order_id(&mut self, order_id: OrderId) {
        self.filled_order_ids.push(order_id);
    }

    pub fn total_quantity_executed(&self) -> u64 {
        self.transactions.iter().map(|t| t.quantity).sum()
    }

    pub fn executed_value(&self) -> u64 {
        self.transactions
            .iter()
            .map(|t| t.price * t.quantity)
            .sum()
    }

    pub fn executed_quantity(&self) -> u64 {
        self.transactions.iter().map(|t| t.quantity).sum()
    }

    pub fn average_price(&self) -> Option<f64> {
        let executed_qty = self.executed_quantity();
        if executed_qty == 0 {
            None
        } else {
            Some(self.executed_value() as f64 / executed_qty as f64)
        }
    }
}


