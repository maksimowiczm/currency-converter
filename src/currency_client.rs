use crate::currency_service::CurrencyService;
use crate::models::{CurrencyCode, ExchangeRates};
use async_trait::async_trait;
use derive_more::Display;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
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
            // unwrap is safe here because the string is hardcoded
            "application/json"
                .parse()
                .expect("Content-Type header creation error"),
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
            .map_err(|err| CurrencyClientError::HttpRequestError(err))?;

        let response = match response.status().as_u16() {
            200 => deserialize_response::<ExchangeRatesWrapper>(response).await,
            422 => deserialize_response::<ErrorResponse>(response)
                .await
                .and_then(|body| Err(CurrencyClientError::ValidationError(body))),
            _ => {
                // no idea how to make it pretty code without unreachable :/
                response
                    .error_for_status()
                    .map_err(|err| CurrencyClientError::HttpRequestError(err))?;
                unreachable!()
            }
        }?;

        Ok(response.data)
    }
}

async fn deserialize_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, CurrencyClientError<reqwest::Error>> {
    response
        .json::<T>()
        .await
        .map_err(CurrencyClientError::SerializationError)
}

#[derive(Deserialize, Display)]
#[display(
    fmt = "{:?}, {}",
    errors,
    info
)]
pub struct ErrorResponse {
    message: String,
    errors: Errors,
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

pub enum CurrencyClientError<TError> {
    HeaderCreationError(String),
    ClientBuildError(TError),
    HttpRequestError(TError),
    ValidationError(ErrorResponse),
    SerializationError(TError),
}

impl Error for CurrencyClientError<reqwest::Error> {}

impl Display for CurrencyClientError<reqwest::Error> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Currency client error, ")?;
        match self {
            CurrencyClientError::HeaderCreationError(err) => {
                write!(f, "Header creation error: {}", err)
            }
            CurrencyClientError::ClientBuildError(err) => {
                write!(f, "Client build error: {}", err)
            }
            CurrencyClientError::HttpRequestError(err) => {
                write!(f, "Request error: {}", err)
            }
            CurrencyClientError::SerializationError(err) => {
                write!(f, "Serialization error: {}", err)
            }
            CurrencyClientError::ValidationError(err) => {
                write!(f, "Validation error: {}", err)
            }
        }
    }
}

impl Debug for CurrencyClientError<reqwest::Error> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
