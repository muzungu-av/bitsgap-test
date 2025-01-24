use std::fmt;

#[derive(Debug)]
pub enum ExchangeFactoryError {
    MissingExchangeEnv,
    MissingRestUrl(String),
    UnknownExchange(),
}

impl fmt::Display for ExchangeFactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExchangeFactoryError::MissingExchangeEnv => {
                write!(f, "Missing exchange environment variable")
            }
            ExchangeFactoryError::MissingRestUrl(exchange) => {
                write!(f, "Missing REST URL for: {}", exchange)
            }
            ExchangeFactoryError::UnknownExchange() => write!(f, "Missing EXCHANGE"),
        }
    }
}
