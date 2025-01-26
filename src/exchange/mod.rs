mod error;
use sqlx::{Pool, Sqlite};
use tracing::info;

use crate::{
    config::settings::Settings,
    exchange::error::ExchangeFactoryError,
    http_client::http_client::{ReqwestClient, RestClient},
};

/// Universal structure for the exchange
pub struct Exchange {
    pub name: String,
    pub rest_url: String,
    pub rest_client: Box<dyn RestClient>,
}

impl Exchange {
    pub fn new(name: &str, rest_url: &str, rest_client: Box<dyn RestClient>) -> Self {
        Self {
            name: name.to_string(),
            rest_url: rest_url.to_string(),
            rest_client,
        }
    }

    pub async fn connect(&self) -> Result<(), String> {
        info!("Connecting to {} API at {}", self.name, self.rest_url);
        Ok(())
    }

    /// Получение данных из API
    pub async fn fetch_data(&self, endpoint: &str) -> Result<String, String> {
        let url = format!("{}/{}", self.rest_url, endpoint);
        println!("Fetching data from {}", url);
        match self.rest_client.get(&url).await {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("Failed to fetch data: {}", e)),
        }
    }
}

pub struct ExchangeFactory;

impl ExchangeFactory {
    /// Creates an exchange instance based on the EXCHANGE variable and the corresponding REST_URL
    pub fn create(settings: &Settings) -> Result<ExchangeBuilder, ExchangeFactoryError> {
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
        /*** Exchange Builder pattern ***/
        Ok(ExchangeBuilder::new()
            .set_name(exchange_name)
            .set_rest_url(rest_url)
            .set_rest_client(Box::new(ReqwestClient::new()))) // Return Builder with Exchange configured

        //todo добавить другие
    }
}

#[derive(Debug)]
pub enum ExchangeBuilderError {
    MissingName,
    MissingRestUrl,
    MissingRestClient,
}

pub struct ExchangeBuilder {
    name: Option<String>,
    rest_url: Option<String>,
    rest_client: Option<Box<dyn RestClient>>,
    db_pool: Option<Pool<Sqlite>>,
}

impl ExchangeBuilder {
    // Create a new empty Builder
    pub fn new() -> Self {
        Self {
            name: None,
            rest_url: None,
            rest_client: None,
            db_pool: None,
        }
    }

    // Set the name of the exchange
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Set rest URL
    pub fn set_rest_url(mut self, rest_url: &str) -> Self {
        self.rest_url = Some(rest_url.to_string());
        self
    }

    // Set rest client
    pub fn set_rest_client(mut self, client: Box<dyn RestClient>) -> Self {
        self.rest_client = Some(client);
        self
    }

    // Set DB pool
    pub fn set_target_db(&mut self, db_pool: Pool<Sqlite>) -> &Self {
        self.db_pool = Some(db_pool);
        self
    }

    // Build Exchange, verifying that all parameters are present
    pub fn build(self) -> Result<Exchange, ExchangeBuilderError> {
        let name = self.name.ok_or(ExchangeBuilderError::MissingName)?;
        let rest_url = self.rest_url.ok_or(ExchangeBuilderError::MissingRestUrl)?;
        let rest_client = self
            .rest_client
            .ok_or(ExchangeBuilderError::MissingRestClient)?;

        Ok(Exchange::new(&name, &rest_url, rest_client))
    }
}
