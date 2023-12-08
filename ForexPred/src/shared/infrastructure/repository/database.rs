use sqlx::mysql::MySqlPool;
use dotenv::dotenv;
use std::env;
use tokio::sync::OnceCell;

pub static DB_POOL: OnceCell<Database> = OnceCell::const_new();

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = MySqlPool::connect(&database_url).await?;
        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}

pub async fn get_database() -> Result<&'static Database, sqlx::Error> {
    DB_POOL.get_or_try_init(Database::new).await
}