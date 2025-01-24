mod error;
use crate::{config::settings::Settings, exchange::error::ExchangeFactoryError};

/// Universal structure for the exchange
pub struct Exchange {
    pub name: String,
    pub rest_url: String,
}

impl Exchange {
    pub fn new(name: &str, rest_url: &str) -> Self {
        Self {
            name: name.to_string(),
            rest_url: rest_url.to_string(),
        }
    }
}

pub struct ExchangeFactory;

impl ExchangeFactory {
    /// Creates an exchange instance based on the EXCHANGE variable and the corresponding REST_URL
    pub fn create(settings: &Settings) -> Result<Exchange, ExchangeFactoryError> {
        // Getting exchange type from EXCHANGE
        let exchange_name = &settings.exchange;
        if exchange_name.is_empty() {
            return Err(ExchangeFactoryError::MissingExchangeEnv);
        }

        let rest_url = match exchange_name.to_lowercase().as_str() {
            "poloniex" => &settings.poloniex_rest_url,
            "binance" => &settings.binance_rest_url,
            _ => return Err(ExchangeFactoryError::UnknownExchange()),
        };

        if rest_url.is_empty() {
            return Err(ExchangeFactoryError::MissingRestUrl(
                exchange_name.to_uppercase(),
            ));
        }
        Ok(Exchange::new(exchange_name, rest_url))
    }
}
