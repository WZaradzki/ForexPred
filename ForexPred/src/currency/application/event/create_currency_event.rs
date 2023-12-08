use async_trait::async_trait;

use crate::{
    currency::{
        application::validator::create_currency_validator::CurrencyCreateValidator,
        domain::entity::currency::Currency,
        infrastructure::repository::currency_repository::CurrencyRepository,
    },
    shared::event::event_root::Event,
};

pub struct CreateCurrencyEvent {
    pub data: CurrencyCreateValidator,
    currency_repository: CurrencyRepository<'static>,
}
impl CreateCurrencyEvent {
    pub async fn new(data: CurrencyCreateValidator) -> Self {
        let currency_repository = CurrencyRepository::new().await;

        Self {
            data,
            currency_repository,
        }
    }
}

#[async_trait]
impl<'a> Event<'a> for CreateCurrencyEvent {
    async fn handle(&'a self) -> Result<(), String> {
        let currency = Currency {
            id: uuid::Uuid::new_v4().to_string(),
            name: self.data.name.clone(),
            iso: self.data.iso.clone(),
        };

        self.currency_repository
            .create_currency(currency)
            .await
            .expect("Error creating new currency");

        Ok(())
    }
}
