pub mod reqwest_client;

use async_trait::async_trait;
use mockall::automock;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[automock]
#[async_trait]
pub trait HttpClient<T>: Send + Sync
where
    T: Send + Sync,
{
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
