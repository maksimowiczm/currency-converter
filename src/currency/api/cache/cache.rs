use async_trait::async_trait;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[async_trait]
pub trait Cache<TKey, TValue>: Sync + Send {
    async fn set(&self, key: TKey, value: TValue) -> Result<(), CacheError>;
    async fn get(&self, key: TKey) -> Result<Option<TValue>, CacheError>;
}

#[derive(Debug)]
pub enum CacheError {}

impl Display for CacheError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for CacheError {}
