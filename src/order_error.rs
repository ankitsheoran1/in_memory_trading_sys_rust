
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
    InvalidOperation(String)
}