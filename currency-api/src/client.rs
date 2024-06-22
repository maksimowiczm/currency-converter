use crate::models::{CurrencyCode, ExchangeRates};
use async_trait::async_trait;

#[async_trait]
pub trait CurrencyApiClient<TError> {
    async fn get_exchange_rates(
        &self,
        base: CurrencyCode,
        target: &[CurrencyCode],
    ) -> Result<ExchangeRates, TError>;
}
