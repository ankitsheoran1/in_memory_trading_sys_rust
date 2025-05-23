use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use crate::side::Side;
use dashmap::DashMap;
use crate::level::PriceLevel;
use crate::match_result::MatchResult;
use crate::order_error::OrderError;
use crate::order_type::{OrderId, OrderType};

pub struct OrderBook {
    side: Side,
    bids: DashMap<u64, PriceLevel>,
    asks: DashMap<u64, PriceLevel>,
    orders: DashMap<OrderId, (u64, Side)>,
    has_traded: AtomicBool,
    last_trade_price: AtomicU64,
    market_close_timestamp: AtomicU64,
}

impl OrderBook {
    pub fn new(side: Side) -> Self {
        OrderBook {
            side,
            bids: DashMap::new(),
            asks: DashMap::new(),
            orders: DashMap::new(),
            has_traded: AtomicBool::new(false),
            last_trade_price: AtomicU64::new(0),
            market_close_timestamp: AtomicU64::new(0),
        }
    }

    pub fn best_bid(&self) -> Option<u64> {
        let mut price = None;

        for item in self.bids.iter()  {
            let key = *item.key();
            if price.is_none() || price.unwrap() < key {
                price = Some(key)
            }
        }
        price
    }

    pub fn best_ask(&self) -> Option<u64> {
        let mut price = None;

        for item in self.asks.iter()  {
            let key = *item.key();
            if price.is_none() || price.unwrap() > key {
                price = Some(key)
            }
        }
        price
    }

    pub fn mid_price(&self) -> Option<f64> {
        match ((self.best_bid(), self.best_ask())) {
            Some((bid, ask)) => Some((bid as f64 + ask as f64) / 2.0),
            _=> None
        }
    }

    pub fn last_trade_price(&self) -> Option<u64> {
        if let price = self.last_trade_price.load(Ordering::SeqCst) {
            Some(price)
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
       let mut bids = Vec::new();
        for item in self.bids {
            bids.push(item.1.get_order());
        }

        let mut asks = Vec::new();
        for item in self.asks {
            asks.push(item.1.get_order());
        }
        bids.into_iter().chain(asks.into_iter()).flatten().collect()
    }

    pub fn get_order_by_id(&self, id: OrderId) -> Option<Arc<OrderType>> {
        match self.orders.get(&id)?.value() {
            Some((price, side)) => {
                match side {
                    Side::Sell => {
                        if let Some(level) = self.asks.get(&price) {
                            for order in level.get_order() {
                                if order.id() == id {
                                     return Some(order);
                                }
                            }
                        }
                    }
                    Side::Buy => {
                        if let Some(level) = self.bids.get(&price) {
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

    pub fn match_market_order(&self, order_id: OrderId, side: Side, quantity: u64) -> Result<MatchResult, OrderError>{
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
                if let Some(level) = match_side.get_mut(&price) {
                    let mut price_level = level.value();
                    let matched_res = price_level.match_order(order_id, remaining_quantity);
                    if !matched_res.transactions.is_empty() {
                        self.last_trade_price.store(price, Ordering::SeqCst);
                        self.has_traded.store(true, Ordering::SeqCst);
                    }
                    for transaction in matched_res.transactions {
                        match_result.add_transaction(transaction);
                    }
                    for filled_order_id in matched_res.filled_order_ids {
                        match_result.add_filled_order_id(*filled_order_id);
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
                } else {  break; /* ignore this, it should never possible */}
            } else {  break; /* ignore this, it should never possible */}

        }

        for order_id in filled_orders {
            self.orders.remove(&order_id);
        }

        match_result.remaining_quantity = remaining_quantity;
        match_result.is_complete = remaining_quantity == 0;

        if match_result.transactions.is_empty() {
            Err(OrderError::InsufficientLiquidity{
                side,
                requested: quantity,
                available: 0,
            })
        }
        Ok(match_result)
    }
}