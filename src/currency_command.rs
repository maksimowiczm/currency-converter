use currency::currency_service::CurrencyService;

pub enum CurrencyCommand {
    List {
        source_currency_code: String,
    },
    Get {
        source_currency_code: String,
        target_currency_code: String,
        amount: f64,
    },
}

impl CurrencyCommand {
    pub async fn execute(
        &self,
        service: Box<dyn CurrencyService>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match self {
            CurrencyCommand::List {
                source_currency_code,
            } => {
                let result = service
                    .get_exchange_rates(&source_currency_code.to_uppercase())
                    .await?;

                // Display currencies in alphabetical order
                let mut vec = result.iter().collect::<Vec<_>>();
                vec.sort_by(|a, b| a.0.cmp(b.0));

                let rows = vec
                    .iter()
                    .map(|(key, exchange_rate)| format!("{key} = {exchange_rate}\n"))
                    .collect::<String>();

                format!(
                    "Exchange rates for {}\n{}",
                    source_currency_code.to_uppercase(),
                    rows
                )
            }
            CurrencyCommand::Get {
                source_currency_code,
                target_currency_code,
                amount,
            } => {
                let exchange_rate = service
                    .get_currency_exchange_rate(
                        &source_currency_code.to_uppercase(),
                        &target_currency_code.to_uppercase(),
                    )
                    .await?;

                format!("{} {}", amount * exchange_rate, exchange_rate)
            }
        };

        Ok(result)
    }
}
