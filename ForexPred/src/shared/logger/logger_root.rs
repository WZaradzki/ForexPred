use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Logger {
    data: Vec<u8>,
}
pub static LOGGER_INSTANCE: OnceCell<Logger> = OnceCell::new();

impl Logger {
    pub fn global() -> &'static Logger {
        LOGGER_INSTANCE.get().expect("logger is not initialized")
    }

    pub fn print(print_text: &str) {
        println!("Logger: {:?}", print_text);
    }

    pub fn init() -> Result<Logger, std::io::Error> {
        Ok(Logger {
            data: [0; 2].to_vec(),
        })
    }
}
