use derive_more::{Display, FromStr};
use serde::Deserialize;

mod cereal;

#[derive(Debug, Deserialize, FromStr, Clone, Display)]
#[display(fmt = "{}", "code.to_uppercase()")]
pub struct CurrencyCode {
    code: String,
}

impl PartialEq for CurrencyCode {
    fn eq(&self, other: &Self) -> bool {
        self.code.eq_ignore_ascii_case(&other.code)
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ExchangeRates {
    rates: Vec<(CurrencyCode, f64)>,
}

impl ExchangeRates {
    pub fn new(rates: Vec<(CurrencyCode, f64)>) -> Self {
        ExchangeRates { rates }
    }

    pub fn get_rate(&self, target: &CurrencyCode) -> Option<f64> {
        self.rates
            .iter()
            .find(|(currency, _)| currency == target)
            .map(|(_, rate)| *rate)
    }
}
