use crate::domains::data::{
    configs::currencies::Currencies, http::exchange_rate_api::requests::ExchangeRateApi,
    subdomains::responses::ExchangeRatePairResponse,
};

pub struct ExchangeRateApiService {
    exchange_rate_api: ExchangeRateApi,
}

impl ExchangeRateApiService {
    pub fn new() -> Self {
        Self {
            exchange_rate_api: ExchangeRateApi::new(),
        }
    }

    pub async fn get_rate_by_currency_pair(
        &self,
        from_currency: Currencies,
        to_currency: Currencies,
    ) -> Result<ExchangeRatePairResponse, reqwest::Error> {
        let response = self
            .exchange_rate_api
            .get_rate_by_currency_pair_request(from_currency, to_currency)
            .await?;

        let response = response.as_object().unwrap();

        return Ok(ExchangeRatePairResponse {
            from_currency,
            to_currency,
            rate: response.get("conversion_rate").unwrap().as_f64().unwrap(),
            date: response
                .get("time_last_update_utc")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        });
    }
}
