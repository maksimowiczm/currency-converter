use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait CurrencyService {
    async fn get_currency_exchange_rate(
        &self,
        source_currency_code: &str,
        target_currency_code: &str,
    ) -> Result<f64, CurrencyServiceError>;

    async fn get_exchange_rates(
        &self,
        source_currency_code: &str,
    ) -> Result<HashMap<String, f64>, CurrencyServiceError>;
}

#[derive(Debug)]
pub enum CurrencyServiceError {
    SourceCurrencyError,
    TargetCurrencyError,
    Other(String),
}
