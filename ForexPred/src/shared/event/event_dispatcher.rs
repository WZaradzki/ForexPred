use once_cell::sync::Lazy;
use std::sync::Arc;

use super::event_root::Event;

pub static EVENT_DISPATCHER: Lazy<EventDispatcher> = Lazy::new(|| EventDispatcher::new());

trait MyEventListener: Send + Sync {
    fn on_event(&self, event: &dyn Event);
}

pub struct MyEventListenerImpl;

impl MyEventListener for MyEventListenerImpl {
    fn on_event(&self, event: &dyn Event<'_>) {
        let _ = event.handle();
    }
}

pub struct EventDispatcher {
    listeners: Vec<Arc<dyn MyEventListener>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        let listeners = vec![Arc::new(MyEventListenerImpl {}) as Arc<dyn MyEventListener>];

        EventDispatcher { listeners }
    }

    // pub fn add_listener(&mut self, listener: Arc<dyn MyEventListener>) {
    //     self.listeners.push(listener);
    // }

    pub async fn dispatch_event(&self, event: &dyn Event<'static>) {
        for listener in &self.listeners {
            listener.on_event(event);
        }
    }
}
