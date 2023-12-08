use crate::shared::event::event_root::Event;

pub struct BaseListener {}

pub trait EventHandler {
    fn handle(&self, event: &dyn Event);
}


