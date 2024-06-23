use crate::currency_client::{CurrencyClient, CurrencyClientError};
use crate::currency_service::CurrencyService;
use crate::models::CurrencyCode;
use clap::Parser;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

mod currency_client;
mod currency_service;
mod models;

#[derive(Parser, Debug)]
struct Arguments {
    /// Source currency code
    source_currency_code: String,

    /// Target currency code
    target_currency_code: String,

    /// Amount which will be converted
    amount: f64,

    /// API key used for authentication
    #[arg(long)]
    api_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let args = Arguments::parse();

    let api_key = args.api_key.or_else(|| std::env::var("CURRENCY_API_KEY").ok())
        .unwrap_or_else(|| {
            eprintln!("API key is missing. Please provide it via --api-key or CURRENCY_API_KEY environment variable.");
            std::process::exit(1);
        });

    let client = CurrencyClient::from_api_key(api_key).map_err(|err| CliError(err))?;

    // parse will never fail because it wraps a string
    let base: CurrencyCode = args
        .source_currency_code
        .parse()
        .expect("Invalid base currency code");
    let target: CurrencyCode = args
        .target_currency_code
        .parse()
        .expect("Invalid target currency code");

    let exchange_rates = client
        .get_exchange_rates(base, &[target.clone()])
        .await
        .map_err(|err| CliError(err))?;

    // calculate the converted amount
    let rate = exchange_rates.get_rate(&target).unwrap_or_else(|| {
        eprintln!("Exchange rate for {target} not found.");
        std::process::exit(2);
    });
    let converted_amount = args.amount * rate;

    println!("{converted_amount} {rate}");

    Ok(())
}


struct CliError(CurrencyClientError<reqwest::Error>);

impl Debug for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            CurrencyClientError::ValidationError(err) => {
                let mut parts = vec![];

                if !err.errors.currencies_error() {
                    parts.push("Invalid target currency code".to_string());
                }

                if !err.errors.base_currency_error() {
                    parts.push("Invalid base currency code".to_string());
                }

                write!(f, "{}", parts.join(", "))
            }
            CurrencyClientError::ApiKeyInvalid => write!(f, "Invalid authentication credentials"),
            _ => write!(f, "{}", self.0),
        }
    }
}
