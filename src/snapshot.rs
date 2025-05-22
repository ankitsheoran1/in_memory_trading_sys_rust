use std::str::FromStr;
use std::sync::Arc;
use crate::order_error::OrderError;
use crate::order_type::OrderType;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

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

    fn reset(&mut self) {
        self.quantity = 0;
        self.order_count = 0;
        self.orders= Vec::new();
        self.force_push = false;
    }

    fn default() -> Self {
        Self {
            price: 0,
            quantity: 0,
            order_count: 0,
            orders: Vec::new(),
            force_push: false,
        }
    }

    pub fn add(&mut self, price: u64, order: Arc<OrderType>, last_order: bool) {
        self.orders.push(order);
        self.order_count += 1;
        self.force_push = false;
        self.price = price;

        // if force // last_order push to disk
        if self.force_push || last_order {
            self.persist();
            *self = Self::default();

        }
    }

    fn persist(&self) {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let filename = format!("snapshot_{}.json",
                               date);
        let mut file = OpenOptions::new().create(true).append(true).open(filename);
        if let Ok(mut file) = file {
            if let Ok(json) = self.to_string() {
                if let Err(e) = writeln!(file, "{}", json) {
                    eprintln!("Error writing to file: {}", e);
                }
            } else {
                eprintln!("Error serializing snapshot to JSON");
            }
        } else {
            eprintln!("Error opening file");
        }
    }

    pub fn to_string(&self) -> Result<String, OrderError> {
        Ok(self.to_json_string())
    }

    fn to_json_string(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");

        // Add basic fields
        json.push_str(&format!("  \"price\": {},\n", self.price));
        json.push_str(&format!("  \"quantity\": {},\n", self.quantity));
        json.push_str(&format!("  \"order_count\": {},\n", self.order_count));
        json.push_str(&format!("  \"force_push\": {},\n", self.force_push));

        // Add orders array
        json.push_str("  \"orders\": [\n");
        for (i, order) in self.orders.iter().enumerate() {
            json.push_str("    ");
            json.push_str(&self.order_to_json_string(order));

            // Add comma if not the last element
            if i < self.orders.len() - 1 {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ]\n");

        json.push('}');
        json
    }

    // pub fn to_compact_json(&self) -> String {
    //     let mut json = String::new();
    //     json.push('{');
    //
    //     json.push_str(&format!("\"price\":{},", self.price));
    //     json.push_str(&format!("\"quantity\":{},", self.quantity));
    //     json.push_str(&format!("\"order_count\":{},", self.order_count));
    //     json.push_str(&format!("\"force_push\":{},", self.force_push));
    //
    //     json.push_str("\"orders\":[");
    //     for (i, order) in self.orders.iter().enumerate() {
    //         json.push_str(&self.order_to_compact_json_string(order));
    //         if i < self.orders.len() - 1 {
    //             json.push(',');
    //         }
    //     }
    //     json.push_str("]}");
    //
    //     json
    // }

    fn order_to_json_string(&self, order: &Arc<OrderType>) -> String {
        // This is a placeholder implementation since OrderType structure isn't provided
        // You'll need to replace this with the actual OrderType fields
        match order.as_ref() {
            OrderType::MarketOrder {
                id,
                price,
                quantity,
                side,
                timestamp,
                time_in_force,
            } =>
                format!(
                    "{{\"type\":\"MarketOrder\",\"id\":\"{}\",\"price\":{},\"quantity\":{},\"side\":\"{:?}\",\"timestamp\":{},\"time_in_force\":\"{:?}\"}}",
                    id.0, price, quantity, side, timestamp, time_in_force
                ),
            OrderType::LimitOrder {
                id,
                price,
                quantity,
                side,
                timestamp,
                time_in_force,
            } =>
                format!(
                    "{{\"type\":\"MarketOrder\",\"id\":\"{}\",\"price\":{},\"quantity\":{},\"side\":\"{:?}\",\"timestamp\":{},\"time_in_force\":\"{:?}\"}}",
                    id.0, price, quantity, side, timestamp, time_in_force
                )
        }
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

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_snapshot_to_string() {
        let snapshot = Snapshot::new(100);
        let json_str = snapshot.to_string().unwrap();
        println!("Snapshot JSON:\n{}", json_str);
    }

    #[test]
    fn test_file_operations() {
        let snapshot = Snapshot::new(100);

        snapshot.persist();
        let snapshot = Snapshot::new(101);
        snapshot.persist();
    }



}



