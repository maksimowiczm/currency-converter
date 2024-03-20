mod currency_command;

use crate::currency_command::CurrencyCommand;
use clap::Parser;
use currency::api::api_currency_service::ApiCurrencyService;
use currency::api::cache::redis_cache::RedisCache;
use currency::api::cache_currency_service::CacheCurrencyService;
use currency::api::http_client::reqwest_client::ReqwestClient;
use currency::currency_service::CurrencyService;
use fred::prelude::{Builder, ClientLike, RedisConfig};
use log::info;
use std::env;

#[derive(Parser)]
struct Arguments {
    /// API key used for authentication
    #[arg(long)]
    api_key: Option<String>,

    /// Source currency code
    source_currency_code: Option<String>,

    /// Target currency code
    target_currency_code: Option<String>,

    /// Amount which will be converted
    amount: Option<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

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

    let api_key = args
        .api_key
        .or_else(|| env::var("CURRENCY_API_KEY").ok())
        .ok_or("You have to provide API key")?;

    let service = setup_service(api_key).await?;
    let result = command.execute(service).await?;

    println!("{result}");

    Ok(())
}

async fn setup_service(
    api_key: String,
) -> Result<Box<dyn CurrencyService>, Box<dyn std::error::Error>> {
    let api_url = env::var("API_URL").unwrap_or("https://api.freecurrencyapi.com/v1/latest".into());
    let http_client = Box::new(ReqwestClient::new());

    // create cacheless service
    let service = Box::new(ApiCurrencyService::new(
        api_url,
        api_key.to_string(),
        http_client,
    ));

    let client_option = env::var("REDIS_URL")
        .ok()
        .and_then(|url| RedisConfig::from_url(&url).ok())
        .and_then(|config| Builder::from_config(config).build().ok());

    // try creating service with cache
    let service: Box<dyn CurrencyService> = if let Some(client) = client_option {
        match client.init().await {
            Ok(_) => {
                info!("Using Redis as cache service");
                let redis_cache = RedisCache::new(client);
                Box::new(CacheCurrencyService::new(Box::new(redis_cache), service))
            }
            Err(_) => {
                info!("Using cacheless service");
                service
            }
        }
    } else {
        info!("Using cacheless service");
        service
    };

    Ok(service)
}
