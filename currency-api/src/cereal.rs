use crate::models::{CurrencyCode, ExchangeRates};
use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for ExchangeRates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExchangeRatesVisitor;

        impl<'de> serde::de::Visitor<'de> for ExchangeRatesVisitor {
            type Value = ExchangeRates;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map of currency codes to exchange rates")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut rates: Vec<(CurrencyCode, f64)> =
                    Vec::with_capacity(map.size_hint().unwrap_or(5));
                while let Some((currency_symbol, rate)) = map.next_entry::<String, f64>()? {
                    let currency_symbol: CurrencyCode =
                        currency_symbol.parse().map_err(serde::de::Error::custom)?;
                    rates.push((currency_symbol, rate));
                }

                Ok(ExchangeRates { rates })
            }
        }

        deserializer.deserialize_map(ExchangeRatesVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_exchange_rates() {
        let json = json!({
            "AED": 3.67306,
            "AFN": 91.80254,
            "ALL": 108.22904,
            "AMD": 480.41659,
        });

        let expected = ExchangeRates {
            rates: vec![
                ("AED".parse().unwrap(), 3.67306),
                ("AFN".parse().unwrap(), 91.80254),
                ("ALL".parse().unwrap(), 108.22904),
                ("AMD".parse().unwrap(), 480.41659),
            ],
        };

        let actual: ExchangeRates = serde_json::from_value(json).unwrap();

        assert_eq!(expected, actual);
    }
}
