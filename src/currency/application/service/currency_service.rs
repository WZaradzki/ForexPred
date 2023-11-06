use crate::{
    currency::{
        application::validator::create_currency_validator::CurrencyCreateValidator,
        domain::entity::currency::Currency,
        infrastructure::repository::currency_repository::CurrencyRepository,
    },
    shared::infrastructure::repository::base::Repositories,
};

pub struct CurrencyService {
    currency_repository: CurrencyRepository,
}

impl CurrencyService {
    pub fn new() -> Self {
        let currency_repository = Repositories::new().currency_repository;

        Self {
            currency_repository,
        }
    }

    pub fn create(&self, currency: CurrencyCreateValidator) -> Result<Currency, String> {
        let currency = Currency {
            id: uuid::Uuid::new_v4().to_string(),
            name: currency.name,
            iso: currency.iso,
        };

        return Ok(self
            .currency_repository
            .create_currency(currency)
            .expect("Error creating new currency"));
    }
}
