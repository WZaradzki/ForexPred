use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyRate {
    #[serde(default)]
    pub id: String,
    pub from_currency_id: String,
    pub to_currency_id: String,
}
