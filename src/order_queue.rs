use std::sync::Arc;
use crossbeam::queue::SegQueue;
use crate::order_type::OrderType;

pub struct OrderQueue {

    orders: SegQueue<Arc<OrderType>>
}