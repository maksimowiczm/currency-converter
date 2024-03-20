use crate::api::cache::cache::Cache;
use crate::currency_service::{CurrencyService, CurrencyServiceError};
use async_trait::async_trait;
use log::info;
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
                info!("Cache hit with key = {key:?}");
                return Ok(exchange_rate);
            }
        }

        info!("Cache miss with key = {key:?}");
        let exchange_rate = self
            .wrapped
            .get_currency_exchange_rate(source_currency_code, target_currency_code)
            .await?;

        if let Err(e) = self.cache.set(key.clone(), exchange_rate.to_string()).await {
            Err(CurrencyServiceError::Other(e.to_string()))
        } else {
            info!("Stored key = {key:?}");
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
                info!("Cache hit with key = {key:?}");
                return Ok(exchange_rate);
            }
        }

        info!("Cache miss with key = {key:?}");
        let rates = self
            .wrapped
            .get_exchange_rates(source_currency_code)
            .await?;

        let json = serde_json::to_string(&rates)
            .or_else(|e| Err(CurrencyServiceError::Other(e.to_string())))?;

        if let Err(e) = self.cache.set(key.clone(), json).await {
            Err(CurrencyServiceError::Other(e.to_string()))
        } else {
            info!("Stored key = {key:?}");
            Ok(rates)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::cache::cache::MockCache;
    use crate::currency_service::MockCurrencyService;
    use maplit::hashmap;
    use mockall::predicate;

    const BASE_CURRENCY: &str = "USD";

    #[tokio::test]
    async fn get_exchange_rates_use_cached_values() {
        let mut cache_mock = MockCache::new();
        let cached = r#"{"PLN":4.1,"EUR":0.9}"#.to_string();
        cache_mock
            .expect_get()
            .with(predicate::eq(BASE_CURRENCY.to_string()))
            .times(1)
            .returning(move |_| Ok(Some(cached.clone())));
        let mut currency_service_mock = MockCurrencyService::new();
        currency_service_mock.expect_get_exchange_rates().never();
        let cache_service =
            CacheCurrencyService::new(Box::new(cache_mock), Box::new(currency_service_mock));

        let result = cache_service.get_exchange_rates(BASE_CURRENCY).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        let pln = result.get("PLN").unwrap();
        assert_eq!(*pln, 4.1f64);

        let eur = result.get("EUR").unwrap();
        assert_eq!(*eur, 0.9f64)
    }

    #[tokio::test]
    async fn get_exchange_rates_without_cached_values() {
        let mut cache_mock = MockCache::new();
        cache_mock
            .expect_get()
            .with(predicate::eq(BASE_CURRENCY.to_string()))
            .times(1)
            .returning(move |_| Ok(None));

        let mut currency_service_mock = MockCurrencyService::new();
        let map = hashmap! {
            "PLN".to_string() => 4.1,
            "EUR".to_string() => 0.9,
        };
        let json_map = serde_json::to_string(&map).unwrap();
        currency_service_mock
            .expect_get_exchange_rates()
            .with(predicate::eq(BASE_CURRENCY.to_string()))
            .times(1)
            .returning(move |_| Ok(map.clone()));

        cache_mock
            .expect_set()
            .with(
                predicate::eq(BASE_CURRENCY.to_string()),
                predicate::eq(json_map),
            )
            .times(1)
            .returning(move |_, _| Ok(()));

        let cache_service =
            CacheCurrencyService::new(Box::new(cache_mock), Box::new(currency_service_mock));

        let result = cache_service.get_exchange_rates(BASE_CURRENCY).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        let pln = result.get("PLN").unwrap();
        assert_eq!(*pln, 4.1f64);

        let eur = result.get("EUR").unwrap();
        assert_eq!(*eur, 0.9f64)
    }

    // more tests
}
