use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::currencies)]
pub struct Currency {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub iso: String,
}

pub struct CurrencyCreateValidator {
    pub name: String,
    pub iso: String,
}