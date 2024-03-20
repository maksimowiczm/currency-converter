use crate::api::cache::cache::{Cache, CacheError};
use async_trait::async_trait;
use fred::clients::RedisClient;
use fred::prelude::KeysInterface;

pub struct RedisCache {
    client: RedisClient,
}

impl RedisCache {
    pub fn new(client: RedisClient) -> Self {
        RedisCache { client }
    }
}

#[async_trait]
impl Cache<String, String> for RedisCache {
    async fn set(&self, key: String, value: String) -> Result<(), CacheError> {
        self.client
            .set(key, value, None, None, false)
            .await
            .map_err(|e| CacheError::UnknownError(Box::new(e)))
    }

    async fn get(&self, key: String) -> Result<Option<String>, CacheError> {
        self.client
            .get(key)
            .await
            .map_err(|e| CacheError::UnknownError(Box::new(e)))
    }
}
