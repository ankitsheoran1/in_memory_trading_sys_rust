// use std::{collections::HashMap, sync::Arc};
//
//
// type UID = u64;
// type Quantity = u32;
// type Price = u64;
// type Count = u32;
// type Volume = u64;
//
// #[derive(Clone, Copy, PartialEq)]
// enum Side {
//    Sell,
//    Buy,
// }
//
// impl Side {
//     fn opposite(self) -> Side {
//         match self {
//             Side::Sell => Side::Buy,
//             Side::Buy => Side::Sell,
//         }
//     }
// }
//
// #[derive(Clone)]
// pub struct Order {
//     uid: UID,
//     side: Side,
//     quantity: Quantity,
//     price: Price,
//     limit: Option<*mut Limit>,
//
// }
//
// pub struct Limit {
//     price: Price,
//     count: Count,
//     volume: Volume,
//     head: Option<Box<Order>>,
//     tail: Option<Box<Order>>,
//     left: Option<*mut Limit>,
//     right: Option<*mut Limit>,
//     parent: Option<*mut Limit>,
// }
//
// impl Limit {
//     fn new(order: &mut Order) -> Self {
//         Limit {
//             price: order.price,
//             count: 1,
//             volume: order.quantity as Volume,
//             head: Some(Box::new(order.clone())),
//             tail: Some(Box::new(order.clone())),
//             left: None,
//             right: None,
//             parent: None,
//
//         }
//     }
// }
//
// pub struct LimitTree {
//     root: Option<*mut Limit>,
//     limits: HashMap<Price, Box<Limit>>,
//     best: Option<*mut Limit>,
//     last_best_price: Price,
//     count: Count,
//     volume: Volume,
// }
//
// impl LimitTree {
//     fn new() -> Self {
//         LimitTree {
//             root: None,
//             limits: HashMap::new(),
//             best: None,
//             last_best_price: 0,
//             count: 0,
//             volume: 0
//         }
//     }
//
//     fn clear(&mut self) {
//         self.limits.clear();
//         self.root = None;
//         self.best = None;
//         self.count = 0;
//         self.volume = 0;
//
//     }
//
//     fn insert(&mut self, order: &mut Order) {
//         // let limit = self.limits.entry(order.price).or_insert_with(|| {
//         //     let mut new_limit = Box::new(Limit::new(order));
//         //     self.insert_bst(&mut new_limit);
//         //     new_limit
//         // });
//
//         // order.limit = Some(&mut **limit);
//         // limit.count += 1;
//         // limit.volume == order.quantity as Volume;
//         // self.count += 1;
//         // self.volume += order.quantity as Volume;
//
//         // let best_value = self.best;
//         // let is_better = match best_value {
//         //     Some(best) => self.is_better(order.price, unsafe {(*best).price}),
//         //     None => true,
//         // };
//
//         // if is_better {
//         //     self.best = Some(&mut **limit)
//         // }
//         let price = order.price;
//     let quantity = order.quantity as Volume;
//     let mut new_best = None;
//
//     {
//         let limit = self.limits.entry(price).or_insert_with(|| {
//             let mut new_limit = Box::new(Limit::new(order));
//             self.insert_bst(&mut new_limit);
//             new_limit
//         });
//
//         order.limit = Some(&mut **limit);
//         limit.count += 1;
//         limit.volume == quantity;
//         self.count += 1;
//         self.volume += quantity;
//
//         new_best = Some(limit);
//     }
//
//
//
//     if let Some(best) = new_best {
//         let is_better = match self.best {
//             Some(best) => self.is_better(price, unsafe {(*best).price}),
//             None => true,
//         };
//
//         if is_better {
//             self.best = Some(best.as_mut() as *mut Limit)
//         }
//     }
//     }
//     fn insert_bst(&mut self, new_limit: &mut Box<Limit>) {
//         let mut current = self.root;
//         let mut parent = None;
//
//         while let Some(node) = current {
//             parent = Some(node);
//             current = if new_limit.price < unsafe { (*node).price } {
//                 unsafe { (*node).left }
//             } else {
//                 unsafe { (*node).right }
//             };
//         }
//
//         new_limit.parent = parent;
//         let new_limit_ptr = &mut **new_limit as *mut Limit;
//
//         if let Some(parent_node) = parent {
//             if new_limit.price < unsafe { (*parent_node).price } {
//                 unsafe { (*parent_node).left = Some(new_limit_ptr) };
//             } else {
//                 unsafe { (*parent_node).right = Some(new_limit_ptr) };
//             }
//         } else {
//             self.root = Some(new_limit_ptr);
//         }
//     }
//
//     fn is_better(&self, price1: Price, price2: Price) -> bool {
//         match self.best {
//             Some(_) => price1 < price2,
//             None => true,
//         }
//     }
//
//
//     unsafe fn cancel(&mut self, order: &Order) {
//         if let Some(limit) = self.limits.get_mut(&order.price) {
//             limit.count -= 1;
//             limit.volume -= order.quantity as Volume;
//             // Remove order from the limit's order queue
//             if limit.count == 0 {
//                 self.remove_bst(limit);
//                 self.limits.remove(&order.price);
//             }
//         }
//         self.count -= 1;
//         self.volume -= order.quantity as Volume;
//         if let Some(best) = self.best {
//             self.last_best_price = unsafe { (*best).price };
//         }
//     }
//
//     fn remove_bst(&mut self, limit: &mut Box<Limit>) {
//         let mut node = self.root;
//         let mut parent = None;
//         let mut is_left_child = false;
//
//         // Find the node to remove
//         while let Some(current_node) = node {
//             unsafe {
//                 if (*current_node).price == limit.price {
//                     break;
//                 } else if limit.price < (*current_node).price {
//                     parent = node;
//                     node = (*current_node).left;
//                     is_left_child = true;
//                 } else {
//                     parent = node;
//                     node = (*current_node).right;
//                     is_left_child = false;
//                 }
//             }
//         }
//
//         // Node not found
//         if node.is_none() {
//             return;
//         }
//
//         let node = node.unwrap();
//
//         unsafe {
//             // Case 1: Node to be removed has no children
//             if (*node).left.is_none() && (*node).right.is_none() {
//                 if let Some(root) = self.root {
//                 if node == root {
//                     self.root = None;
//                 } else if is_left_child {
//                     (*parent.unwrap()).left = None;
//                 } else {
//                     (*parent.unwrap()).right = None;
//                 }
//             }
//
//             }
//             // Case 2: Node to be removed has only one child
//             else if (*node).right.is_none() {
//                 if let Some(root) = self.root {
//                 if node == root {
//                     self.root = (*node).left;
//                 } else if is_left_child {
//                     (*parent.unwrap()).left = (*node).left;
//                 } else {
//                     (*parent.unwrap()).right = (*node).left;
//                 }
//             }
//             } else if (*node).left.is_none() {
//                 if let Some(root) = self.root {
//                 if node == root {
//                     self.root = (*node).right;
//                 } else if is_left_child {
//                     (*parent.unwrap()).left = (*node).right;
//                 } else {
//                     (*parent.unwrap()).right = (*node).right;
//                 }
//             }
//             }
//             // Case 3: Node to be removed has two children
//             else {
//                 let mut replacement = (*node).right;
//                 let mut replacement_parent = node;
//                 while let Some(replacement_left) = (*replacement.unwrap()).left.as_ref()  {
//                     replacement_parent = replacement.expect("Failed Deref");
//                     replacement = Some(replacement_left.clone());
//                 }
//
//
//                 // let mut replacement = (*node).right;
//                 // let mut replacement_parent = node;
//
//                 // Find the in-order successor (the smallest in the right subtree)
//                 // while let Some(replacement_left) = unsafe { (*replacement).left.as_ref() }  {
//                 //     replacement_parent = replacement;
//                 //     replacement = replacement_left;
//                 // }
//
//                 // Replace node with in-order successor
//                 if node == self.root.expect("root Deref failed") {
//                     self.root = replacement;
//                 } else if is_left_child {
//                     (*parent.unwrap()).left = replacement;
//                 } else {
//                     (*parent.unwrap()).right = replacement;
//                 }
//
//                 // Correct the parent link of the in-order successor
//                 if replacement != (*node).right {
//                     (*replacement_parent).left = (*replacement.expect("error")).right;
//                     (*replacement.expect("error")).right = (*node).right;
//                 }
//                 (*replacement.expect("error")).left = (*node).left;
//             }
//         }
//     }
//
//     fn market<F>(&mut self, order: &mut Order, mut did_fill: F)
//     where
//         F: FnMut(UID),
//     {
//         while let Some(best) = self.best {
//             let best_limit = unsafe { &mut *best };
//             if self.can_match(best_limit.price, order.price) {
//                 let match_order = best_limit.head.as_mut().unwrap();
//                 if match_order.quantity >= order.quantity {
//                     if match_order.quantity == order.quantity {
//                         unsafe { self.cancel(match_order) };
//                         did_fill(match_order.uid);
//                     } else {
//                         match_order.quantity -= order.quantity;
//                         best_limit.volume -= order.quantity as Volume;
//                         self.volume -= order.quantity as Volume;
//                     }
//                     order.quantity = 0;
//                     return;
//                 } else {
//                     order.quantity -= match_order.quantity;
//                     unsafe { self.cancel(match_order) };
//                     did_fill(match_order.uid);
//                 }
//             } else {
//                 break;
//             }
//         }
//     }
//
//     fn can_match(&self, limit_price: Price, market_price: Price) -> bool {
//         market_price == 0 || market_price <= limit_price
//     }
//
// }
//
// impl Order {
//     fn new(uid: UID, side: Side, quantity: Quantity, price: Price) -> Self {
//         Order {
//             uid,
//             side,
//             quantity,
//             price,
//             limit: None,
//         }
//     }
//
//
// }
//
// struct LimitOrderBook {
//     sells: LimitTree,
//     buys: LimitTree,
//     orders: HashMap<UID, Order>,
// }
//
// impl LimitOrderBook {
//     fn new() -> Self {
//         LimitOrderBook {
//             sells: LimitTree::new(),
//             buys: LimitTree::new(),
//             orders: HashMap::new(),
//         }
//     }
//
//     fn clear(&mut self) {
//         self.sells.clear();
//         self.buys.clear();
//         self.orders.clear();
//     }
//
//     fn limit_sell(&mut self, order_id: UID, quantity: Quantity, price: Price) {
//         let mut order = Order::new(order_id, Side::Sell, quantity, price);
//         self.orders.insert(order_id, order.clone());
//         if let Some(best) = self.buys.best {
//             if price <= unsafe { (*best).price } {
//                 self.buys.market(&mut order, |uid| {
//                     self.orders.remove(&uid);
//                 });
//                 if order.quantity == 0 {
//                     self.orders.remove(&order_id);
//                     return;
//                 }
//             }
//         }
//         self.sells.insert(&mut order);
//     }
//
//     fn limit_buy(&mut self, order_id: UID, quantity: Quantity, price: Price) {
//         let mut order = Order::new(order_id, Side::Buy, quantity, price);
//         self.orders.insert(order_id, order.clone());
//         if let Some(best) = self.sells.best {
//             if price >= unsafe { (*best).price } {
//                 self.sells.market(&mut order, |uid| {
//                     self.orders.remove(&uid);
//                 });
//                 if order.quantity == 0 {
//                     self.orders.remove(&order_id);
//                     return;
//                 }
//             }
//         }
//         self.buys.insert(&mut order);
//     }
//
//     fn limit(&mut self, side: Side, order_id: UID, quantity: Quantity, price: Price) {
//         match side {
//             Side::Sell => self.limit_sell(order_id, quantity, price),
//             Side::Buy => self.limit_buy(order_id, quantity, price),
//         }
//     }
//
//     fn has(&self, order_id: UID) -> bool {
//         self.orders.contains_key(&order_id)
//     }
//
//     fn get(&self, order_id: UID) -> Option<&Order> {
//         self.orders.get(&order_id)
//     }
//
//     fn cancel(&mut self, order_id: UID) {
//         if let Some(order) = self.orders.get(&order_id) {
//             match order.side {
//                 Side::Sell => unsafe { self.sells.cancel(order) },
//                 Side::Buy => unsafe { self.buys.cancel(order) },
//             }
//             self.orders.remove(&order_id);
//         }
//     }
//
//     fn reduce(&mut self, order_id: UID, quantity: Quantity) {
//         if let Some(order) = self.orders.get_mut(&order_id) {
//             if quantity > order.quantity {
//                 panic!(
//                     "trying to remove {} from order with {} available!",
//                     quantity, order.quantity
//                 );
//             }
//             order.quantity -= quantity;
//             if let Some(limit) = order.limit {
//                 unsafe {
//                     (*limit).volume -= quantity as Volume;
//                 }
//             }
//             match order.side {
//                 Side::Sell => self.sells.volume -= quantity as Volume,
//                 Side::Buy => self.buys.volume -= quantity as Volume,
//             }
//             if order.quantity == 0 {
//                 self.cancel(order_id);
//             }
//         }
//     }
//
//     fn market_sell(&mut self, order_id: UID, quantity: Quantity) {
//         let mut order = Order::new(order_id, Side::Sell, quantity, 0);
//         self.buys.market(&mut order, |uid| {
//             self.orders.remove(&uid);
//         });
//     }
//
//     fn market_buy(&mut self, order_id: UID, quantity: Quantity) {
//         let mut order = Order::new(order_id, Side::Buy, quantity, 0);
//         self.sells.market(&mut order, |uid| {
//             self.orders.remove(&uid);
//         });
//     }
//
//     fn market(&mut self, side: Side, order_id: UID, quantity: Quantity) {
//         match side {
//             Side::Sell => self.market_sell(order_id, quantity),
//             Side::Buy => self.market_buy(order_id, quantity),
//         }
//     }
//
//     fn best_sell(&self) -> Price {
//         self.sells.best.map_or(0, |best| unsafe { (*best).price })
//     }
//
//     fn best_buy(&self) -> Price {
//         self.buys.best.map_or(0, |best| unsafe { (*best).price })
//     }
//
//     fn best(&self, side: Side) -> Price {
//         match side {
//             Side::Sell => self.best_sell(),
//             Side::Buy => self.best_buy(),
//         }
//     }
//
//     fn price(&self) -> Price {
//         match (self.sells.best, self.buys.best) {
//             (None, None) => 0,
//             (None, Some(best)) => unsafe { (*best).price },
//             (Some(best), None) => unsafe { (*best).price },
//             (Some(sell_best), Some(buy_best)) => {
//                 (unsafe { (*sell_best).price } + unsafe { (*buy_best).price }) / 2
//             }
//         }
//     }
//
//     fn last_best_sell(&self) -> Price {
//         self.sells.last_best_price
//     }
//
//     fn last_best_buy(&self) -> Price {
//         self.buys.last_best_price
//     }
//
//     fn last_best(&self, side: Side) -> Price {
//         match side {
//             Side::Sell => self.last_best_sell(),
//             Side::Buy => self.last_best_buy(),
//         }
//     }
//
//     fn last_price(&self) -> Price {
//         (self.sells.last_best_price + self.buys.last_best_price) / 2
//     }
//
//     fn volume_sell(&self, price: Price) -> Volume {
//         self.sells.limits.get(&price).map_or(0, |limit| limit.volume)
//     }
//
//     fn volume_sell_total(&self) -> Volume {
//         self.sells.volume
//     }
//
//     fn volume_sell_best(&self) -> Volume {
//         self.sells.best.map_or(0, |best| unsafe { (*best).volume })
//     }
//
//     fn volume_buy(&self, price: Price) -> Volume {
//         self.buys.limits.get(&price).map_or(0, |limit| limit.volume)
//     }
//
//     fn volume_buy_total(&self) -> Volume {
//         self.buys.volume
//     }
//
//     fn volume_buy_best(&self) -> Volume {
//         self.buys.best.map_or(0, |best| unsafe { (*best).volume })
//     }
//
//     fn volume(&self, price: Price) -> Volume {
//         self.volume_buy(price) + self.volume_sell(price)
//     }
//
//     fn total_volume(&self) -> Volume {
//         self.sells.volume + self.buys.volume
//     }
//
//     fn count_at(&self, price: Price) -> Count {
//         self.buys.limits.get(&price).map_or(0, |limit| limit.count)
//             + self.sells.limits.get(&price).map_or(0, |limit| limit.count)
//     }
//
//     fn count_sell(&self) -> Count {
//         self.sells.count
//     }
//
//     fn count_buy(&self) -> Count {
//         self.buys.count
//     }
//
//     fn total_count(&self) -> Count {
//         self.sells.count + self.buys.count
//     }
//
//
// }
//
//
// /*
// We are going to Design a optimized and scalable trading system,
// which would be lock free to get rid off of lot of problem it brings like deadlock, slow down process etc
//
// Main Feature -
//   It should Handle new Order
//   User Should able to support both update and cancel order
//   We should have a snapshot job running frequently to handle cases to store the state of system frequently
//   We should store statisitics  of system , may be day wise but for now just thinking to store aggregated
//   Order Execution should be parllel
//   Transaction path operation like order matching should be O(1) time compleixity only
//
//
// Proposed Design
//
//   PriceLevel - > {price, #order at this price, [orders], stats}
//   Order -> {type , id, quantity, side, timestamp, time_to_stop/ force}
//   OrderBook - {symbol, bids -{price_level, PriceLevel}, asks {price_level, PriceLevel}}
//   TradeSystem - {symbol , Orderbook}
//
//
//  */
//
//
fn main() {
    println!("Hello, world!");
}
