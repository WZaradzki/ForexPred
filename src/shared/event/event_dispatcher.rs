use crate::shared::aggregate::aggregate_root::EventHandler;

use super::event_root::Event;

pub struct BaseEventDispatcher {
    pub listeners: Vec<Box<dyn EventHandler>>,
}

pub trait EventDispatcher {
    fn register_listener(&mut self, listener: Box<dyn EventHandler>);
    fn dispatch(&self, event: &dyn Event);
}

// impl EventDispatcher for BaseEventDispatcher {
//     fn register_listener(&mut self, listener: Box<dyn EventHandler>) {
//         self.listeners.push(listener);
//     }

//     fn dispatch(&self, event: &dyn Event) {
//         for listener in &self.listeners {
//             listener.handle(event);
//         }
//     }
// }
