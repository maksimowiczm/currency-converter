use crate::api::cache::cache::Cache;
use crate::currency_service::{CurrencyService, CurrencyServiceError};
use async_trait::async_trait;
use std::collections::HashMap;

pub struct CacheCurrencyService {
    cache: Box<dyn Cache<String, String>>,
    wrapped: Box<dyn CurrencyService>,
}

impl CacheCurrencyService {
    pub fn new(cache: Box<dyn Cache<String, String>>, wrapped: Box<dyn CurrencyService>) -> Self {
        CacheCurrencyService { cache, wrapped }
    }

    fn create_cache_key(source_currency_code: &str, target_currency_code: &str) -> String {
        format!("{source_currency_code}-{target_currency_code}")
    }
}

#[async_trait]
impl CurrencyService for CacheCurrencyService {
    async fn get_currency_exchange_rate(
        &self,
        source_currency_code: &str,
        target_currency_code: &str,
    ) -> Result<f64, CurrencyServiceError> {
        let key = Self::create_cache_key(source_currency_code, target_currency_code);
        if let Ok(Some(cached)) = self.cache.get(key.clone()).await {
            if let Ok(exchange_rate) = serde_json::from_str::<f64>(&cached) {
                return Ok(exchange_rate);
            }
        }

        let exchange_rate = self
            .wrapped
            .get_currency_exchange_rate(source_currency_code, target_currency_code)
            .await?;

        if let Err(e) = self.cache.set(key, exchange_rate.to_string()).await {
            Err(CurrencyServiceError::Other(e.to_string()))
        } else {
            Ok(exchange_rate)
        }
    }

    async fn get_exchange_rates(
        &self,
        source_currency_code: &str,
    ) -> Result<HashMap<String, f64>, CurrencyServiceError> {
        let key = source_currency_code.to_string();
        if let Ok(Some(cached)) = self.cache.get(key.clone()).await {
            if let Ok(exchange_rate) = serde_json::from_str::<HashMap<String, f64>>(&cached) {
                return Ok(exchange_rate);
            }
        }

        let rates = self
            .wrapped
            .get_exchange_rates(source_currency_code)
            .await?;

        let json = serde_json::to_string(&rates)
            .or_else(|e| Err(CurrencyServiceError::Other(e.to_string())))?;

        if let Err(e) = self.cache.set(key, json).await {
            Err(CurrencyServiceError::Other(e.to_string()))
        } else {
            Ok(rates)
        }
    }
}
