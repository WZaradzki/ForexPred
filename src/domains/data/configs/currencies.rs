#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currencies {
    USD,
    PLN
}

impl Currencies {
    pub fn get_iso_4217(&self) -> &'static str {
        match *self {
            Currencies::USD => "USD",
            Currencies::PLN => "PLN",
        }
    }
}
