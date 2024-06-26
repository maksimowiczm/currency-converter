use crate::currency_service::CurrencyService;
use crate::models::{CurrencyCode, ExchangeRates};
use async_trait::async_trait;
use derive_more::Display;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CurrencyClient {
    client: Client,
}

impl CurrencyClient {
    pub fn from_api_key(api_key: String) -> Result<CurrencyClient, CurrencyClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "apikey",
            api_key
                .parse()
                .map_err(|_| CurrencyClientError::InvalidApiKeyFormat)?,
        );
        headers.insert(
            "Content-Type",
            // unwrap is safe here because the string is hardcoded
            "application/json".parse().unwrap(),
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(CurrencyClientError::ClientBuildError)?;

        Ok(CurrencyClient { client })
    }
}

#[async_trait]
impl CurrencyService<CurrencyClientError> for CurrencyClient {
    async fn get_exchange_rates(
        &self,
        base: CurrencyCode,
        target: &[CurrencyCode],
    ) -> Result<ExchangeRates, CurrencyClientError> {
        #[derive(Deserialize)]
        struct ExchangeRatesWrapper {
            data: ExchangeRates,
        }

        let target_str = target
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url = format!(
            "https://api.freecurrencyapi.com/v1/latest?base_currency={}&currencies={}",
            base, target_str
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(CurrencyClientError::RequestError)?;

        let response = match response.status().as_u16() {
            200 => deserialize_response::<ExchangeRatesWrapper>(response).await,
            422 => deserialize_response::<ErrorResponse>(response)
                .await
                .and_then(|body| Err(CurrencyClientError::ValidationError(body))),
            401 => Err(CurrencyClientError::ApiKeyInvalid),
            _ => {
                // no idea how to make it pretty code without unreachable :/
                response
                    .error_for_status()
                    .map_err(CurrencyClientError::ResponseError)?;
                unreachable!()
            }
        }?;

        Ok(response.data)
    }
}

async fn deserialize_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, CurrencyClientError> {
    response
        .json::<T>()
        .await
        .map_err(|_| CurrencyClientError::ResponseSerializationError)
}

#[derive(Deserialize, Display, Debug)]
#[display(fmt = "{:?}, {}", errors, info)]
pub struct ErrorResponse {
    message: String,
    pub errors: Errors,
    info: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Errors {
    #[serde(default)]
    currencies: Vec<String>,
    #[serde(default)]
    base_currency: Vec<String>,
}

impl Errors {
    pub fn base_currency_error(&self) -> bool {
        !self.base_currency.is_empty()
    }

    pub fn currencies_error(&self) -> bool {
        !self.currencies.is_empty()
    }
}

#[derive(Debug, Display)]
pub enum CurrencyClientError {
    #[display(fmt = "Invalid API key format")]
    InvalidApiKeyFormat,
    #[display(fmt = "Client build error: {}", _0)]
    ClientBuildError(reqwest::Error),
    #[display(fmt = "API key is invalid")]
    ApiKeyInvalid,
    #[display(fmt = "Validation error: {}", _0)]
    ValidationError(ErrorResponse),
    #[display(fmt = "Response serialization error")]
    ResponseSerializationError,
    #[display(fmt = "Request error: {}", _0)]
    RequestError(reqwest::Error),
    #[display(fmt = "Response error: {}", _0)]
    ResponseError(reqwest::Error),
}

impl Error for CurrencyClientError {}
