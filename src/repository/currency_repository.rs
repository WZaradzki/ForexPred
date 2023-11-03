use chrono::Utc;
use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection, RunQueryDsl, QueryDsl,
};
use uuid::Uuid;
use std::fmt::Error;

use crate::domains::currency_rate::models::currency::{Currency, CurrencyCreateValidator};

use super::{database::DBPool, schema::currencies};

pub struct CurrencyRepository {
    connection: DBPool,
}

impl CurrencyRepository {
    pub fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        CurrencyRepository { connection: pool }
        
    }

    // pub fn get_users(&self) -> Vec<User> {
    //     users
    //         .load::<User>(&mut self.connection.get().unwrap())
    //         .expect("Error loading all users")
    // }

    pub fn create_currency(&self, currency: CurrencyCreateValidator) -> Result<Currency, Error> {
        let currency: Currency = Currency {
            id: Uuid::new_v4().to_string(),
            name: currency.name,
            iso: currency.iso,
        };
        diesel::insert_into(currencies::table)
            .values(&currency)
            .execute(&mut self.connection.get().unwrap())
            .expect("Error creating new user");

        Ok(currency)
    }

    // pub fn get_user_by_id(&self, user_id: &str) -> Option<User> {
    //     let user = users
    //         .find(user_id)
    //         .first::<User>(&mut self.connection.get().unwrap())
    //         .expect("Error loading user by id");
    //     Some(user)
    // }

    // pub fn delete_user_by_id(&self, user_id: &str) -> Option<usize> {
    //     let count = diesel::delete(users.find(user_id))
    //         .execute(&mut self.connection.get().unwrap())
    //         .expect("Error deleting user by id");
    //     Some(count)
    // }

    // pub fn update_user_by_id(&self, user_id: &str, mut user: User) -> Option<User> {
    //     user.updated_at = Some(Utc::now().naive_utc());

    //     let update = diesel::update(users.find(user_id))
    //         .set(&user)
    //         .execute(&mut self.connection.get().unwrap())
    //         .expect("Error updating user by id");

    //     if update == 0 {
    //         return None;
    //     }

    //     Some(user)
    // }
}
