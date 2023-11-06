use crate::{currency::infrastructure::repository::currency_repository::CurrencyRepository, rates::currency::infrastructure::repository::currency_rate_repository::CurrencyRateRepository};

use super::database::Database;


pub struct Repositories {
    pub currency_repository: CurrencyRepository,
    pub currency_rate_repository: CurrencyRateRepository,
    // // Add other repositories here...
}

impl Repositories {
    pub fn new() -> Self {
        let database_url = Database::get_database_url();
        let currency_repository = CurrencyRepository::new(database_url.clone());
        let currency_rate_repository = CurrencyRateRepository::new(database_url.clone());
        // Initialize other repositories...

        Repositories {
            currency_repository,
            currency_rate_repository,
            // Add other repositories here...
        }
    }
}