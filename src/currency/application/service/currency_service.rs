use crate::{
    currency::{
        application::{
            event::create_currency_event::CreateCurrencyEvent,
            validator::create_currency_validator::CurrencyCreateValidator,
        },
        domain::entity::currency::Currency,
        infrastructure::repository::currency_repository::CurrencyRepository,
    },
    shared::{
        aggregate::aggregate_root::{BaseListener, EventHandler},
        event::{
            event_dispatcher::{BaseEventDispatcher, EventDispatcher},
            event_root::Event,
        },
        infrastructure::repository::base::Repositories,
    },
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
        let mut dispatcher = BaseEventDispatcher { listeners: vec![] };
        let listener = Box::new(BaseListener {});
        impl EventHandler for BaseListener {
            fn handle(&self, event: &dyn Event) {
                let _ = event.handle();
            }
        }
        impl EventDispatcher for BaseEventDispatcher {
            fn register_listener(&mut self, listener: Box<dyn EventHandler>) {
                self.listeners.push(listener);
            }

            fn dispatch(&self, event: &dyn Event) {
                for listener in &self.listeners {
                    listener.handle(event);
                }
            }
        }
        dispatcher.register_listener(listener);

        let event = CreateCurrencyEvent {
            data: "example data".to_string(),
        };
        impl Event for CreateCurrencyEvent {
            fn handle(&self) -> Result<(), String> {
                println!("Handling event with data: {}", self.data);
                Ok(())
            }
        }
        dispatcher.dispatch(&event);

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
