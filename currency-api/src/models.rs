use derive_more::FromStr;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, FromStr, Clone)]
pub struct CurrencyCode {
    code: String,
}

impl Display for CurrencyCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code.to_uppercase())
    }
}

impl PartialEq for CurrencyCode {
    fn eq(&self, other: &Self) -> bool {
        self.code.eq_ignore_ascii_case(&other.code)
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ExchangeRates {
    pub(crate) rates: Vec<(CurrencyCode, f64)>,
}

impl ExchangeRates {
    pub fn get_rate(&self, target: &CurrencyCode) -> Option<f64> {
        self.rates
            .iter()
            .find(|(currency, _)| currency == target)
            .map(|(_, rate)| *rate)
    }
}
