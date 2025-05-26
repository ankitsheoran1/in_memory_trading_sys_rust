use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use cargo::core::resolver::HasDevUnits::No;
use crate::side::Side;
use dashmap::DashMap;
use crate::level::PriceLevel;
use crate::match_result::MatchResult;
use crate::order_error::OrderError;
use crate::order_type::{OrderId, OrderType};
use crate::order_update::OrderUpdate;

pub struct OrderBook {
    symbol: String,
    bids: DashMap<u64, PriceLevel>,
    asks: DashMap<u64, PriceLevel>,
    orders: DashMap<OrderId, (u64, Side)>,
    has_traded: AtomicBool,
    last_trade_price: AtomicU64,
    market_close_timestamp: AtomicU64,
}

impl OrderBook {
    pub fn new(symbol: &str) -> Self {
        OrderBook {
            symbol: symbol.to_string(),
            bids: DashMap::new(),
            asks: DashMap::new(),
            orders: DashMap::new(),
            has_traded: AtomicBool::new(false),
            last_trade_price: AtomicU64::new(0),
            market_close_timestamp: AtomicU64::new(0),
        }
    }


    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn best_bid(&self) -> Option<u64> {
        let mut price = None;

        for item in self.bids.iter() {
            let key = *item.key();
            if price.is_none() || price.unwrap() < key {
                price = Some(key)
            }
        }
        price
    }

    pub fn best_ask(&self) -> Option<u64> {
        let mut price = None;

        for item in self.asks.iter() {
            let key = *item.key();
            if price.is_none() || price.unwrap() > key {
                price = Some(key)
            }
        }
        price
    }

    pub fn mid_price(&self) -> Option<f64> {
        match ((self.best_bid(), self.best_ask())) {
            (Some(bid), Some(ask)) => Some((bid as f64 + ask as f64) / 2.0),
            _ => None
        }
    }

    pub fn last_trade_price(&self) -> Option<u64> {
        if self.last_trade_price.load(Ordering::SeqCst) > 0 {
            Some(self.last_trade_price.load(Ordering::Relaxed))
        } else {
            None
        }
    }

    pub fn spread(&self) -> Option<u64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask.saturating_sub(bid)),
            _ => None,
        }
    }

    pub fn add_order(&self, order: OrderType) -> Arc<OrderType> {
        // Calculate quantities

        // Update atomic counters

        // Add to order queue
        let order_arc = Arc::new(order);
        self.orders.insert(order_arc.id(), (order_arc.price(), order_arc.side()));
        let price_level = match order_arc.side() {
            Side::Buy => self.bids.entry(order_arc.price()).or_insert_with(|| PriceLevel::new(order_arc.price())),
            Side::Sell => self.asks.entry(order_arc.price()).or_insert_with(|| PriceLevel::new(order_arc.price())),
        };
        price_level.add_order(order_arc.clone());

        order_arc
    }

    pub fn get_orders_at_price(&self, price: u64, side: Side) -> Vec<Arc<OrderType>> {
        let price_levels = match side {
            Side::Buy => &self.bids,
            Side::Sell => &self.asks,
        };

        let mut data = Vec::new();

        if let Some(price_level) = price_levels.get(&price) {
            for order in price_level.get_order() {
                data.push(order);
            }
        }

        data
    }

    pub fn get_all_orders(&self) -> Vec<Arc<OrderType>> {
        let mut result = Vec::new();

        // Get all bid orders
        for item in self.bids.iter() {
            let price_level = item.value();

            result.extend(price_level.get_order());
        }

        // Get all ask orders
        for item in self.asks.iter() {
            let price_level = item.value();
            result.extend(price_level.get_order());
        }

        result
    }

    pub fn get_order_by_id(&self, id: OrderId) -> Option<Arc<OrderType>> {
        match self.orders.get(&id)?.value() {
            (price, side) => {
                match side {
                    Side::Sell => {
                        if let Some(level) = self.asks.get(price) {
                            for order in level.get_order() {
                                if order.id() == id {
                                    return Some(order);
                                }
                            }
                        }
                    }
                    Side::Buy => {
                        if let Some(level) = self.bids.get(price) {
                            for order in level.get_order() {
                                if order.id() == id {
                                    return Some(order);
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn match_market_order(&self, order_id: OrderId, side: Side, quantity: u64) -> Result<MatchResult, OrderError> {
        // get order details
        let opposite_side = side.opposite();
        let match_side = match opposite_side {
            Side::Buy => &self.bids,
            Side::Sell => &self.asks,
        };

        let mut remaining_quantity = quantity;
        let mut match_result = MatchResult::new(order_id, quantity);
        let mut filled_orders = Vec::new();

        while remaining_quantity > 0 {
            let best_price = match opposite_side {
                Side::Buy => self.best_bid(),
                Side::Sell => self.best_ask(),
            };

            if let Some(price) = best_price {
                if let Some(mut level) = match_side.get_mut(&price) {
                    let mut price_level = level.value_mut();
                    let matched_res = price_level.match_order(order_id, remaining_quantity);
                    if !matched_res.transactions.is_empty() {
                        self.last_trade_price.store(price, Ordering::SeqCst);
                        self.has_traded.store(true, Ordering::SeqCst);
                    }
                    for transaction in matched_res.transactions {
                        match_result.add_transaction(transaction);
                    }
                    for filled_order_id in matched_res.filled_order_ids {
                        match_result.add_filled_order_id(filled_order_id);
                        filled_orders.push(filled_order_id);
                    }
                    remaining_quantity = matched_res.remaining_quantity;

                    if price_level.order_count() == 0 {
                        // We must drop the mutable reference before removing
                        drop(level);
                        match_side.remove(&price);
                    }

                    if remaining_quantity == 0 {
                        break; // Order fully matched
                    }
                } else { break; /* ignore this, it should never possible */ }
            } else { break; /* ignore this, it should never possible */ }
        }

        for order_id in filled_orders {
            self.orders.remove(&order_id);
        }

        match_result.remaining_quantity = remaining_quantity;
        match_result.is_complete = remaining_quantity == 0;

        if match_result.transactions.is_empty() {
            return Err(OrderError::InsufficientLiquidity {
                side,
                requested: quantity,
                available: 0,
            })
        }
        Ok(match_result)
    }

    pub fn cancel_order(&self, order_id: OrderId)
                        -> Result<Option<Arc<OrderType>>, OrderError> {
        let entry = self.orders.get(&order_id).map(|val| *val);
        if let Some((price, side)) = entry {
            let Some(level) = match side {
                Side::Buy => self.bids.get_mut(&price),
                Side::Sell => self.asks.get_mut(&price),
            };
            let update = OrderUpdate::Cancel { order_id };
            let mut result = None;
            let mut empty_level = false;
            if let Ok(cancelled) = level.update_order(update) {
                result = cancelled;
                empty_level = level.order_count() == 0;
            }

            if result.is_some() {
                self.orders.remove(&order_id);
                if empty_level {
                    match side {
                        Side::Buy => self.bids.remove(&price),
                        Side::Sell => self.asks.remove(&price),
                    };
                }
            }

            Ok(result)
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::Uuid;
    use crate::time_in_force::TimeInForce;
    use super::*;

    fn create_order_id() -> OrderId {
        OrderId(Uuid::new_v4())
    }

    fn create_standard_order(price: u64, quantity: u64, side: Side) -> OrderType {
        OrderType::MarketOrder {
            id: create_order_id(),
            price,
            quantity,
            side,
            timestamp:  SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64,
            time_in_force: TimeInForce::Day,
        }
    }

    #[test]
    fn test_new_order_book() {
        let symbol = "BTCUSD";
        let book = OrderBook::new(symbol);

        assert_eq!(book.symbol(), symbol);
        assert_eq!(book.best_bid(), None);
        assert_eq!(book.best_ask(), None);
        assert_eq!(book.mid_price(), None);
        assert_eq!(book.spread(), None);
        assert_eq!(book.last_trade_price(), None);
    }

    #[test]
    fn test_add_standard_order() {
        let book = OrderBook::new("BTCUSD");
        let order = create_standard_order(1000, 10, Side::Buy);
        let order_id = order.id();

        // Add the order
        let result = book.add_order(order);
        assert_eq!(result.price(), 1000);

        // Verify order was added correctly
        assert_eq!(book.best_bid(), Some(1000));

        // Get the order by ID
        let fetched_order = book.get_order_by_id(order_id);
        assert!(fetched_order.is_some());
        assert_eq!(fetched_order.unwrap().id(), order_id);
    }

    #[test]
    fn test_add_multiple_bids() {
        let book = OrderBook::new("BTCUSD");

        // Add three buy orders at different prices
        let _ = book.add_order(create_standard_order(1000, 10, Side::Buy));
        let _ = book.add_order(create_standard_order(1010, 5, Side::Buy));
        let _ = book.add_order(create_standard_order(990, 15, Side::Buy));

        // Best bid should be the highest price
        assert_eq!(book.best_bid(), Some(1010));

        // Total orders at a specific price
        let orders_at_1000 = book.get_orders_at_price(1000, Side::Buy);
        assert_eq!(orders_at_1000.len(), 1);

        // All orders in the book
        let all_orders = book.get_all_orders();
        assert_eq!(all_orders.len(), 3);
    }

    #[test]
    fn test_add_multiple_asks() {
        let book = OrderBook::new("BTCUSD");

        // Add three sell orders at different prices
        let _ = book.add_order(create_standard_order(1050, 10, Side::Sell));
        let _ = book.add_order(create_standard_order(1040, 5, Side::Sell));
        let _ = book.add_order(create_standard_order(1060, 15, Side::Sell));

        // Best ask should be the lowest price
        assert_eq!(book.best_ask(), Some(1040));
    }

    #[test]
    fn test_cancel_order() {
        let book = OrderBook::new("BTCUSD");

        // Add an order
        let order = create_standard_order(1000, 10, Side::Buy);
        let order_id = order.id();
        let _ = book.add_order(order);

        // Check the order exists
        assert_eq!(book.best_bid(), Some(1000));
        assert!(book.get_order_by_id(order_id).is_some());

        // Cancel the order
        let result = book.cancel_order(order_id);
        assert!(result.is_ok());

        if let Ok(cancelled_order) = result {
            if cancelled_order.is_some() {
                // Verify order is no longer in the book
                assert_eq!(book.best_bid(), None);
                assert!(book.get_order_by_id(order_id).is_none());
            } else {
                panic!("Failed to cancel the order");
            }
        } else {
            panic!("Cancel operation failed");
        }
    }
}