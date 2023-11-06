use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, sql_types::Decimal, Expression};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = crate::shared::infrastructure::db_schema::schema::currency_rates)]
pub struct CurrencyRate {
    #[serde(default)]
    pub id: String,
    pub from_currency_id: String,
    pub to_currency_id: String,
    // pub rate: Numeri,
    // pub date: NaiveDateTime
}
