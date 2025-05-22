use std::str::FromStr;
use crate::order_error::OrderError;
use crate::side::Side::Sell;

pub enum OrderStatus {

    New,
    Active,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired
}

impl OrderStatus {

    pub fn is_active(&self) -> bool {
        match self{
            Self::Active => true,
            Self::PartiallyFilled => true,
            _ => false
        }
    }

    pub fn is_terminated(&self) -> bool {
        matches!(self, Self::Canceled | Self::Filled | Self::Expired | Self::Rejected)
    }
}

impl FromStr for OrderStatus {

    type Err = OrderError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NEW" => Ok(OrderStatus::New),
            "ACTIVE" =>  Ok(OrderStatus::Active),
            "PARTIALLYFILLED" => Ok(OrderStatus::PartiallyFilled),
            "FILLED" => Ok(OrderStatus::Filled),
            "CANCELED" => Ok(OrderStatus::Canceled),
            "REJECTED" => Ok(OrderStatus::Rejected),
            "EXPIRED" => Ok(OrderStatus::Expired),
            _ => Err(OrderError::ParseError {message : format!("Invalid OrderStatus {}", s)}),
        }
    }
}