use crate::currency_service::CurrencyService;
use crate::models::{CurrencyCode, ExchangeRates};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct CurrencyClient {
    client: Client,
}

impl CurrencyClient {
    pub(crate) fn from_api_key(
        api_key: String,
    ) -> Result<CurrencyClient, CurrencyClientError<reqwest::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "apikey",
            api_key.parse().map_err(|err| {
                CurrencyClientError::HeaderCreationError(format!(
                    "API key header creation error: {}",
                    err
                ))
            })?,
        );
        headers.insert(
            "Content-Type",
            "application/json".parse().map_err(|err| {
                CurrencyClientError::HeaderCreationError(format!(
                    "Content-Type header creation error: {}",
                    err
                ))
            })?,
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|err| CurrencyClientError::ClientBuildError(err))?;

        Ok(CurrencyClient { client })
    }
}

#[async_trait]
impl CurrencyService<CurrencyClientError<reqwest::Error>> for CurrencyClient {
    async fn get_exchange_rates(
        &self,
        base: CurrencyCode,
        target: &[CurrencyCode],
    ) -> Result<ExchangeRates, CurrencyClientError<reqwest::Error>> {
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
            .map_err(|err| CurrencyClientError::RequestError(err))?
            .error_for_status()
            .map_err(|err| CurrencyClientError::RequestError(err))?
            .json::<ExchangeRatesWrapper>()
            .await
            .map_err(|err| CurrencyClientError::SerializationError(err))?;

        Ok(response.data)
    }
}

pub enum CurrencyClientError<TError> {
    HeaderCreationError(String),
    ClientBuildError(TError),
    RequestError(TError),
    SerializationError(TError),
}

impl Error for CurrencyClientError<reqwest::Error> {}

impl Display for CurrencyClientError<reqwest::Error> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CurrencyClientError::HeaderCreationError(err) => {
                write!(f, "Header creation error: {}", err)
            }
            CurrencyClientError::ClientBuildError(err) => {
                write!(f, "Client build error: {}", err)
            }
            CurrencyClientError::RequestError(err) => {
                write!(f, "Request error: {}", err)
            }
            CurrencyClientError::SerializationError(err) => {
                write!(f, "Serialization error: {}", err)
            }
        }
    }
}

impl Debug for CurrencyClientError<reqwest::Error> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
