use std::sync::Arc;
use crossbeam::queue::SegQueue;
use crate::order_type::{OrderId, OrderType};

pub struct OrderQueue {

    orders: SegQueue<Arc<OrderType>>
}

impl OrderQueue {

    pub fn new() -> Self {
        Self {
            orders: SegQueue::new(),
        }
    }

    pub fn push(&self, order: Arc<OrderType>) {
        self.orders.push(order)
    }

    pub fn pop(&self) -> Option<Arc<OrderType>> {
        self.orders.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn remove(&self, order_id: OrderId) -> Option<Arc<OrderType>> {
        let mut temp_storage = Vec::new();
        let mut removed_order = None;

        // Pop all items from the queue
        while let Some(order) = self.orders.pop() {
            if order.id() == order_id {
                removed_order = Some(order);
            } else {
                temp_storage.push(order);
            }
        }

        // Push back the orders we want to keep
        for order in temp_storage {
            self.orders.push(order);
        }

        removed_order
    }

}