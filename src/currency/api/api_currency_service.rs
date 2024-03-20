use crate::api::http_client::{HttpClient, HttpError};
use crate::currency_service::{CurrencyService, CurrencyServiceError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ApiCurrencyService {
    api_url: String,
    api_key: String,
    client: Box<dyn HttpClient<String>>,
}

impl ApiCurrencyService {
    pub fn new(api_url: String, api_key: String, client: Box<dyn HttpClient<String>>) -> Self {
        ApiCurrencyService {
            api_url,
            api_key,
            client,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    data: HashMap<String, f64>,
}

#[async_trait]
impl CurrencyService for ApiCurrencyService {
    async fn get_currency_exchange_rate(
        &self,
        source_currency_code: &str,
        target_currency_code: &str,
    ) -> Result<f64, CurrencyServiceError> {
        let request = format!(
            "{}?apikey={}&base_currency={}&currencies={}",
            self.api_url, self.api_key, source_currency_code, target_currency_code,
        );

        let response = self.process_request(&request).await?;

        match response.data.get(target_currency_code) {
            Some(value) => Ok(*value),
            // if it was not found that is really odd
            None => Err(CurrencyServiceError::TargetCurrencyError),
        }
    }

    async fn get_exchange_rates(
        &self,
        source_currency_code: &str,
    ) -> Result<HashMap<String, f64>, CurrencyServiceError> {
        let request = format!(
            "{}?apikey={}&base_currency={}",
            self.api_url, self.api_key, source_currency_code
        );

        let response = self.process_request(&request).await?;

        Ok(response.data)
    }
}

impl ApiCurrencyService {
    async fn process_request(&self, request: &str) -> Result<ApiResponse, CurrencyServiceError> {
        let response = self.client.get(&request).await;

        if let Err(e) = response {
            return match e {
                HttpError::ValidationError(e) => {
                    todo!();
                    Err(CurrencyServiceError::Other("todo".to_string()))
                }
                HttpError::UnexpectedError(_)
                | HttpError::NetworkError
                | HttpError::AuthorizationError
                | HttpError::RateLimitError => Err(CurrencyServiceError::Other(e.to_string())),
            };
        }

        let json = response.unwrap();

        match serde_json::from_str::<ApiResponse>(&json) {
            Ok(deserialized) => Ok(deserialized),
            Err(e) => Err(CurrencyServiceError::Other(e.to_string())), // !?
        }
    }
}
