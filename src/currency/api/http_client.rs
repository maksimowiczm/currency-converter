use async_trait::async_trait;
use std::error::Error;
use std::fmt::{Display, Formatter};

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

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for HttpError {}
