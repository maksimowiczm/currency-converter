use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Currency {
    symbol: String,
    name: String,
    symbol_native: String,
    decimal_digits: i32,
    rounding: i32,
    code: String,
    name_plural: String,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeRate {
    base_symbol: String,
    currency_symbol: String,
    rate: f64,
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
