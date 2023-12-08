use chrono::Utc;
use uuid::Uuid;
use std::fmt::Error;
use sqlx::MySqlPool;

use crate::currency::domain::entity::currency::Currency;
use crate::shared::infrastructure::repository::database::{Database, DB_POOL};

pub struct CurrencyRepository<'a> {
    connection: &'a MySqlPool,
}

impl<'a> CurrencyRepository<'a> {
    pub async fn new() -> CurrencyRepository <'a> {

        let database = DB_POOL.get_or_try_init(Database::new).await.unwrap();

        CurrencyRepository { connection: database.get_pool() }
    }

    // pub fn get_users(&self) -> Vec<User> {
    //     users
    //         .load::<User>(&mut self.connection.get().unwrap())
    //         .expect("Error loading all users")
    // }

    pub async fn create_currency(&self, currency: Currency) -> Result<Currency, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO currencies (id, name, iso)
            VALUES (?, ?, ?)
            "#,
            currency.id, currency.name, currency.iso
        )
            .execute(self.connection)
            .await?;

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
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use diesel::r2d2::Pool;
//     use diesel::Connection;
//     use mockall::{automock, predicate::*};
//     use crate::currency::domain::entity::currency::Currency;
//
//     #[automock]
//     #[derive(Clone)]
//     pub struct MockDBPool {
//         pub pool: Pool<ConnectionManager<MysqlConnection>>,
//     }
//
//
//     #[test]
//     fn test_create_currency() {
//         let mock_pool = MockDBPool::new();
//
//         // Utwórz mocka dla metody get
//         mock_pool.expect_get()
//             .returning(|| Ok(MysqlConnection::establish("mysql://localhost/test").unwrap()));
//
//         let currency_repository = CurrencyRepository {
//             connection: mock_pool,
//         };
//
//         let currency = Currency {
//             id: "".to_string(),
//             name: "".to_string(),
//             iso: "".to_string(),
//         };
//
//         let result = currency_repository.create_currency(currency.clone());
//
//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), currency);
//     }
// }
