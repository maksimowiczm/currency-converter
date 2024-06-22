use derive_more::FromStr;
use serde::Deserialize;

#[derive(Debug, Deserialize, FromStr)]
#[cfg_attr(test, derive(PartialEq))]
pub struct CurrencyCode {
    code: String,
}

#[derive(Debug, Deserialize)]
pub struct Currency {
    symbol: String,
    name: String,
    symbol_native: String,
    decimal_digits: i32,
    rounding: i32,
    code: CurrencyCode,
    name_plural: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ExchangeRates {
    pub(crate) rates: Vec<(CurrencyCode, f64)>,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    quotas: Quotas,
}

#[derive(Debug, Deserialize)]
pub struct Month {
    total: i64,
    used: i64,
    remaining: i64,
}

#[derive(Debug, Deserialize)]
pub struct Quotas {
    month: Month,
}
