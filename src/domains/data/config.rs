pub enum Currencies {
    USD,
}

impl Currencies {
    pub fn get_iso_4217(&self) -> &'static str {
        match *self {
            Currencies::USD => "USD",
        }
    }
}
