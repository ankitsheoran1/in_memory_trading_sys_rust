use serde::{Deserialize, Serialize};
use crate::order_type::OrderId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderUpdate {
    /// Cancel an order
    Cancel {
        /// ID of the order to cancel
        order_id: OrderId,
    },
}