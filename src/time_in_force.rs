use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {

    #[serde(rename(serialize = "DAY"))]
    #[serde(alias = "day", alias = "Day", alias = "DAY")]
    Day,
}