pub trait Event {
    fn handle(&self) -> Result<(), String>;
}