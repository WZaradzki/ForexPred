use crate::{
    currency::application::{
        event::create_currency_event::CreateCurrencyEvent,
        validator::create_currency_validator::CurrencyCreateValidator,
    },
    shared::event::event_dispatcher::EVENT_DISPATCHER,
};

pub struct CurrencyService {}

impl CurrencyService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(&self, currency: CurrencyCreateValidator) -> Result<(), String> {
        let event = CreateCurrencyEvent::new(currency);

        EVENT_DISPATCHER.dispatch_event(&event).await;

        Ok(())
    }
}
