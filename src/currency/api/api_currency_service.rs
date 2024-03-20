use crate::api::http_client::{HttpClient, HttpError};
use crate::currency_service::{CurrencyService, CurrencyServiceError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::info;

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
        info!("Fetched {source_currency_code}-{target_currency_code}");

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
        info!("Fetched {source_currency_code}");

        Ok(response.data)
    }
}

/// Represents freecurrencyapi validation error
/// https://freecurrencyapi.com/docs/status-codes#validation-errors
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiValidationError {
    message: String,
    errors: HashMap<String, Vec<String>>,
    info: String,
}

impl ApiCurrencyService {
    async fn process_request(&self, request: &str) -> Result<ApiResponse, CurrencyServiceError> {
        let response = self.client.get(&request).await;

        if let Err(http_error) = response {
            return match http_error {
                HttpError::ValidationError(http_error_str) => {
                    match serde_json::from_str::<ApiValidationError>(&http_error_str) {
                        Ok(e) => {
                            if e.errors.contains_key("base_currency") {
                                Err(CurrencyServiceError::SourceCurrencyError)
                            } else if e.errors.contains_key("currencies") {
                                Err(CurrencyServiceError::TargetCurrencyError)
                            } else {
                                Err(CurrencyServiceError::Other(http_error_str))
                            }
                        }
                        Err(e) => Err(CurrencyServiceError::Other(e.to_string())),
                    }
                }
                HttpError::UnexpectedError(_)
                | HttpError::NetworkError(_)
                | HttpError::AuthorizationError
                | HttpError::RateLimitError => {
                    Err(CurrencyServiceError::Other(http_error.to_string()))
                }
            };
        }

        let json = response.unwrap();

        match serde_json::from_str::<ApiResponse>(&json) {
            Ok(deserialized) => Ok(deserialized),
            Err(e) => Err(CurrencyServiceError::Other(e.to_string())), // !?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::http_client::MockHttpClient;
    use mockall::predicate;

    const API_URL: &str = "url";
    const API_KEY: &str = "key";
    const BASE_CURRENCY: &str = "USD";

    const SOURCE_CURRENCY_VALIDATION_ERROR_JSON: &str = r#"{"message":"Validation error","errors":{"base_currency":["The selected base currency is invalid."]},"info":""}"#;
    const TARGET_CURRENCY_VALIDATION_ERROR_JSON: &str = r#"{"message":"Validation error","errors":{"currencies":["The selected currencies is invalid."]},"info":""}"#;

    #[tokio::test]
    async fn get_exchange_rates_with_valid_currency_code() {
        let mut http_mock = MockHttpClient::new();
        let expected_request = format!(
            "{}?apikey={}&base_currency={}",
            API_URL, API_KEY, BASE_CURRENCY
        );
        let response = "{\"data\": {\"PLN\": 4.001}}".to_string();
        http_mock
            .expect_get()
            .with(predicate::eq(expected_request))
            .times(1)
            .returning(move |_| Ok(response.to_string()));

        let service = ApiCurrencyService::new(API_URL.into(), API_KEY.into(), Box::new(http_mock));

        let result = service.get_exchange_rates(BASE_CURRENCY).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.contains_key("PLN"));
        assert_eq!(*result.get("PLN").unwrap(), 4.001f64);
    }

    #[tokio::test]
    async fn get_exchange_rates_with_invalid_currency_code() {
        let mut http_mock = MockHttpClient::new();
        let expected_request = format!(
            "{}?apikey={}&base_currency={}",
            API_URL, API_KEY, BASE_CURRENCY
        );
        http_mock
            .expect_get()
            .with(predicate::eq(expected_request))
            .times(1)
            .returning(move |_| {
                Err(HttpError::ValidationError(
                    SOURCE_CURRENCY_VALIDATION_ERROR_JSON.to_string(),
                ))
            });

        let service = ApiCurrencyService::new(API_URL.into(), API_KEY.into(), Box::new(http_mock));

        let result = service.get_exchange_rates(BASE_CURRENCY).await;

        assert!(result.is_err());
        assert_eq!(result, Err(CurrencyServiceError::SourceCurrencyError));
    }

    #[tokio::test]
    async fn get_exchange_rate_for_given_currencies_with_invalid_target_currency_code() {
        let mut http_mock = MockHttpClient::new();
        let expected_request = format!(
            "{}?apikey={}&base_currency={}",
            API_URL, API_KEY, BASE_CURRENCY
        );
        http_mock
            .expect_get()
            .with(predicate::eq(expected_request))
            .times(1)
            .returning(move |_| {
                Err(HttpError::ValidationError(
                    TARGET_CURRENCY_VALIDATION_ERROR_JSON.to_string(),
                ))
            });

        let service = ApiCurrencyService::new(API_URL.into(), API_KEY.into(), Box::new(http_mock));

        let result = service.get_exchange_rates(BASE_CURRENCY).await;

        assert!(result.is_err());
        assert_eq!(result, Err(CurrencyServiceError::TargetCurrencyError));
    }

    // more tests
}
