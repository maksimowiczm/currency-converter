use crate::http::ApiClient;
use clap::Parser;
use currency_api::client::CurrencyApiClient;
use currency_api::models::CurrencyCode;

mod http;

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
    api_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    let client = ApiClient::from_api_key(args.api_key)?;

    let base: CurrencyCode = args.source_currency_code.parse()?;
    let target: CurrencyCode = args.target_currency_code.parse()?;
    let exchange_rates = {
        let target = target.clone();
        client.get_exchange_rates(base, &[target]).await?
    };

    let rate = exchange_rates.get_rate(&target).expect("lol");
    let converted_amount = args.amount * rate;

    println!("{converted_amount} {rate}");

    Ok(())
}
