use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[async_trait]
pub trait CurrencyService: Send + Sync {
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

#[derive(Debug, PartialEq)]
pub enum CurrencyServiceError {
    SourceCurrencyError,
    TargetCurrencyError,
    Other(String),
}

impl Display for CurrencyServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for CurrencyServiceError {}
