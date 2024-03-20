use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait HttpClient<T>: Send + Sync {
    async fn get(&self, url: &str) -> Result<T, HttpError>;
}

#[derive(Debug)]
pub enum HttpError {
    NetworkError,
    AuthorizationError,
    RateLimitError,
    ValidationError(String),
    UnexpectedError(Box<dyn Error>),
}
