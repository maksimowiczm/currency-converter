use async_trait::async_trait;
use currency_api::client::CurrencyApiClient;
use currency_api::models::{CurrencyCode, ExchangeRates};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub(crate) fn from_api_key(
        api_key: String,
    ) -> Result<ApiClient, ApiClientError<reqwest::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "apikey",
            api_key.parse().map_err(|err| {
                ApiClientError::HeaderCreationError(format!(
                    "API key header creation error: {}",
                    err
                ))
            })?,
        );
        headers.insert(
            "Content-Type",
            "application/json".parse().map_err(|err| {
                ApiClientError::HeaderCreationError(format!(
                    "Content-Type header creation error: {}",
                    err
                ))
            })?,
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|err| ApiClientError::ClientBuildError(err))?;

        Ok(ApiClient { client })
    }
}

#[async_trait]
impl CurrencyApiClient<ApiClientError<reqwest::Error>> for ApiClient {
    async fn get_exchange_rates(
        &self,
        base: CurrencyCode,
        target: &[CurrencyCode],
    ) -> Result<ExchangeRates, ApiClientError<reqwest::Error>> {
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
            .map_err(|err| ApiClientError::RequestError(err))?
            .error_for_status()
            .map_err(|err| ApiClientError::RequestError(err))?
            .json::<ExchangeRatesWrapper>()
            .await
            .map_err(|err| ApiClientError::SerializationError(err))?;

        Ok(response.data)
    }
}

pub enum ApiClientError<TError> {
    HeaderCreationError(String),
    ClientBuildError(TError),
    RequestError(TError),
    SerializationError(TError),
}

impl Error for ApiClientError<reqwest::Error> {}

impl Display for ApiClientError<reqwest::Error> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ApiClientError::HeaderCreationError(err) => {
                write!(f, "Header creation error: {}", err)
            }
            ApiClientError::ClientBuildError(err) => {
                write!(f, "Client build error: {}", err)
            }
            ApiClientError::RequestError(err) => {
                write!(f, "Request error: {}", err)
            }
            ApiClientError::SerializationError(err) => {
                write!(f, "Serialization error: {}", err)
            }
        }
    }
}

impl Debug for ApiClientError<reqwest::Error> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
