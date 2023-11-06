use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::shared::infrastructure::db_schema::schema::currencies)]
pub struct Currency {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub iso: String,
}