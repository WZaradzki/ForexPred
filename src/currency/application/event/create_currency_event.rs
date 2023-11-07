use crate::{
    currency::{
        application::validator::create_currency_validator::CurrencyCreateValidator,
        domain::entity::currency::Currency,
        infrastructure::repository::currency_repository::CurrencyRepository,
    },
    shared::{event::event_root::Event, infrastructure::repository::base::Repositories},
};

pub struct CreateCurrencyEvent {
    pub data: CurrencyCreateValidator,
    currency_repository: CurrencyRepository,
}
impl CreateCurrencyEvent {
    pub fn new(data: CurrencyCreateValidator) -> Self {
        let currency_repository = Repositories::new().currency_repository;

        Self {
            data,
            currency_repository,
        }
    }
}
impl Event for CreateCurrencyEvent {
    fn handle(&self) -> Result<(), String> {
        let currency = Currency {
            id: uuid::Uuid::new_v4().to_string(),
            name: self.data.name.clone(),
            iso: self.data.iso.clone(),
        };

        self.currency_repository
            .create_currency(currency)
            .expect("Error creating new currency");

        Ok(())
    }
}
