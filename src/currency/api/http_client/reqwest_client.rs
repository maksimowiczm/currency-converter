use crate::api::http_client::{HttpClient, HttpError};
use async_trait::async_trait;
use reqwest::StatusCode;
use std::marker::PhantomData;

pub struct ReqwestClient<T>
where
    T: Sync + Send,
{
    p: PhantomData<T>,
}

impl<T> ReqwestClient<T>
where
    T: Sync + Send,
{
    pub fn new() -> ReqwestClient<T> {
        ReqwestClient {
            p: Default::default(),
        }
    }
}

#[async_trait]
impl HttpClient<String> for ReqwestClient<String> {
    async fn get(&self, url: &str) -> Result<String, HttpError> {
        let result = match reqwest::get(url).await {
            // match HTTP status codes
            Ok(response) => match response.status() {
                StatusCode::OK => Ok(response),
                StatusCode::UNAUTHORIZED => Err(HttpError::AuthorizationError),
                StatusCode::TOO_MANY_REQUESTS => Err(HttpError::RateLimitError),
                StatusCode::UNPROCESSABLE_ENTITY => {
                    Err(HttpError::ValidationError(response.text().await.unwrap()))
                    // !?
                }
                code => Err(HttpError::UnexpectedError(
                    format!("Unexpected response HTTP status code {code}").into(),
                )),
            },
            Err(e) => Err(HttpError::NetworkError(Box::new(e))),
        }?;

        // parse response to string
        match result.text().await {
            Ok(response) => Ok(response),
            Err(e) => Err(HttpError::UnexpectedError(Box::new(e))),
        }
    }
}
