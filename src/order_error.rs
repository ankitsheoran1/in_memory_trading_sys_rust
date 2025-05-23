use crate::side::Side;

#[derive(Debug)]
pub enum OrderError {

    ParseError {
        message: String
    },
    InvalidFormat,
    UnknownOrderType(String),
    InvalidFieldValue {
        field: String,
        value: String,
    },
    MissingField(String),
    InvalidOperation(String),
    InsufficientLiquidity {
        /// The side of the market order
        side: Side,
        /// Quantity requested
        requested: u64,
        /// Quantity available
        available: u64,
    },
}