use crate::currency_client::CurrencyClient;
use crate::currency_service::CurrencyService;
use crate::models::CurrencyCode;
use clap::Parser;

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    let api_key = args.api_key.or_else(|| std::env::var("CURRENCY_API_KEY").ok())
        .unwrap_or_else(|| {
            eprintln!("API key is missing. Please provide it via --api-key or CURRENCY_API_KEY environment variable.");
            std::process::exit(1);
        });

    let client = CurrencyClient::from_api_key(api_key)?;

    let base: CurrencyCode = args.source_currency_code.parse()?;
    let target: CurrencyCode = args.target_currency_code.parse()?;
    let exchange_rates = {
        let target = target.clone();
        client.get_exchange_rates(base, &[target]).await?
    };

    let rate = exchange_rates.get_rate(&target).unwrap_or_else(|| {
        eprintln!("Exchange rate for {target} not found.");
        std::process::exit(2);
    });
    let converted_amount = args.amount * rate;

    println!("{converted_amount} {rate}");

    Ok(())
}
