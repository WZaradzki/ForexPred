pub trait Event: Send + Sync {
    fn handle(&self) -> Result<(), String>;
}