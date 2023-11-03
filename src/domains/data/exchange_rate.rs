use dotenv::dotenv;
use serde_json::Value;

#[derive(Debug)]
pub struct ExchangeRateApi {
    pub base_url: String,
}

impl ExchangeRateApi {
    pub fn new() -> Self {
        dotenv().ok();
        let exchange_rates_api_url =
            std::env::var("EXCHANGE_RATE_API_URL").expect("EXCHANGE_RATE_API_URL must be set");
        let exchange_rates_access_key: String =
            std::env::var("EXCHANGE_RATE_API_KEY").expect("EXCHANGE_RATE_API_KEY must be set");

        let url_with_api_key =
            exchange_rates_api_url.replace(":apiKey", &exchange_rates_access_key);

        Self {
            base_url: url_with_api_key,
        }
    }

    pub async fn get_latest_by_currency(&self, currency: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/latest/{}", self.base_url, currency);
        let response = reqwest::get(&url)
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response)
    }
}
