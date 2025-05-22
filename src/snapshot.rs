use std::str::FromStr;
use std::sync::Arc;
use crate::order_error::OrderError;
use crate::order_type::OrderType;

pub struct Snapshot {

    pub price: u64,
    pub quantity: u64,
    pub order_count: usize,
    pub orders: Vec<Arc<OrderType>>,
    pub force_push: bool,
}


impl Snapshot {

    pub fn new(price: u64) -> Self {
        Snapshot {
            price,
            quantity: 0,
            order_count: 0,
            orders: Vec::new(),
            force_push: false
        }
    }

    pub fn add(&mut self, price: u64, order: Arc<OrderType>, last_order: bool) {
        self.orders.push(order);
        self.order_count += 1;
        self.force_push = false;
        self.price = price;

        // if force // last_order push to disk
        if self.force_push || last_order {

        }
    }

    fn persist(&self) {

    }
}


impl FromStr for Snapshot {

    type Err = OrderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 || parts[0] != "OrderSnapShot" {
            return Err(OrderError::InvalidFormat);
        }
        let fields_str = parts[1];
        let mut fields = std::collections::HashMap::new();
        for field_pair in fields_str.split(';') {
            let kv: Vec<&str> = field_pair.split('=').collect();
            if kv.len() == 2 {
                fields.insert(kv[0], kv[1]);
            }
        }

        let get_field = |field: &str| -> Result<&str, OrderError> {
            match fields.get(field) {
                Some(res) => Ok(*res),
                None => Err(OrderError::MissingField(field.to_string())),
            }
        };

        let parse_u64 = |field: &str, value: &str| -> Result<u64, OrderError> {
            value.parse::<u64>()
                .map_err(|_| OrderError::InvalidFieldValue {
                    field: field.to_string(),
                    value: value.to_string(),
                })
        };

        let parse_usize = |field: &str, value: &str| -> Result<usize, OrderError> {
            value.parse::<usize>()
                .map_err(|_| OrderError::InvalidFieldValue {
                    field: field.to_string(),
                    value: value.to_string(),
                })
        };

        let price_str = get_field("price")?;
        let price = parse_u64("price", price_str)?;
        let quantity_str = get_field("quantity")?;
        let quantity = parse_u64("quantity", quantity_str)?;
        let order_count_str = get_field("order_count")?;
        let order_count = parse_usize("order_count", order_count_str)?;
        let force_push_str = get_field("force_push")?;
        let force_push = force_push_str.parse::<bool>().map_err(|_| OrderError::InvalidFieldValue {
            field: "force_push".to_string(),
            value: force_push_str.to_string(),
        })?;

        Ok(Snapshot {
            price,
            quantity,
            order_count,
            orders: Vec::new(),
            force_push,
        })
    }
}



