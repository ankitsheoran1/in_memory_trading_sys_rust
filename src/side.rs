
pub enum Side {

    #[serde(alias= "BUY", alias="buy", alias="Buy")]
    Buy,
    #[serde(alias= "SELL", alias="sell", alias="Sell")]
    Sell,
}

impl Side {

    pub fn opposite(&self) -> Self {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }

}