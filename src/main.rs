mod currency_command;

use crate::currency_command::CurrencyCommand;
use clap::Parser;
use currency::api::api_currency_service::ApiCurrencyService;
use currency::api::http_client::reqwest_client::ReqwestClient;
use std::env;

#[derive(Parser)]
struct Arguments {
    /// API key used for authentication
    #[arg(long)]
    api_key: String,

    /// Source currency code
    source_currency_code: Option<String>,

    /// Target currency code
    target_currency_code: Option<String>,

    /// Amount which will be converted
    amount: Option<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    let source_currency_code = args
        .source_currency_code
        .ok_or("You have to provide source currency code!")?;

    let command = if let (Some(target_currency_code), Some(amount)) =
        (args.target_currency_code, args.amount)
    {
        CurrencyCommand::Get {
            source_currency_code,
            target_currency_code,
            amount,
        }
    } else {
        CurrencyCommand::List {
            source_currency_code,
        }
    };

    let api_url = env::var("API_URL").unwrap_or("https://api.freecurrencyapi.com/v1/latest".into());
    let http_client = Box::new(ReqwestClient::new());
    let service = ApiCurrencyService::new(api_url, args.api_key, http_client);

    let result = command.execute(Box::new(service)).await?;

    println!("{result}");

    Ok(())
}
