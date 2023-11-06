use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn get_database_url() -> String {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        return database_url;
    }
}
