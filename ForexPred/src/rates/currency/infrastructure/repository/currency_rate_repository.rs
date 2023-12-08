use chrono::Utc;
use uuid::Uuid;
use std::fmt::Error;
use sqlx::MySqlPool;

use crate::shared::infrastructure::repository::database::DBPool;


pub struct CurrencyRateRepository {
    connection: MySqlPool,
}

impl CurrencyRateRepository {
    pub fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        CurrencyRateRepository { connection: pool }
        
    }

    // pub fn get_users(&self) -> Vec<User> {
    //     users
    //         .load::<User>(&mut self.connection.get().unwrap())
    //         .expect("Error loading all users")
    // }

    // pub fn create_user(&self, user: User) -> Result<User, Error> {
    //     let user = User {
    //         id: Uuid::new_v4().to_string(),
    //         created_at: Some(Utc::now().naive_utc()),
    //         updated_at: Some(Utc::now().naive_utc()),
    //         ..user
    //     };
    //     diesel::insert_into(users)
    //         .values(&user)
    //         .execute(&mut self.connection.get().unwrap())
    //         .expect("Error creating new user");
    //     Ok(user)
    // }

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
