use crate::domains::data::configs::currencies::Currencies;


#[derive(Debug)]
pub struct ExchangeRatePairResponse {
    pub from_currency: Currencies,
    pub to_currency: Currencies,
    pub rate: f64,
    pub date: String,
}
