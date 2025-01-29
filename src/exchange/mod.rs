mod error;
use std::sync::Arc;
use tokio::sync::Mutex;

use sqlx::{Pool, Sqlite};
use tracing::error;
use tracing::info;

use crate::{
    aggregator::CandleAggregator,
    config::settings::Settings,
    exchange::error::ExchangeFactoryError,
    http_client::http_client::{ReqwestClient, RestClient},
    parser::KlineParser,
};

/// Universal structure for the exchange
pub struct Exchange {
    pub name: String,
    pub rest_url: String,
    pub rest_client: Box<dyn RestClient>,
    pub parser: KlineParser,
    pub aggregator: Option<Arc<CandleAggregator>>,
    pub db_pool: Option<Arc<Pool<Sqlite>>>,
}

impl Exchange {
    pub fn new(
        name: &str,
        rest_url: &str,
        rest_client: Box<dyn RestClient>,
        parser: KlineParser,
        aggregator: Option<Arc<CandleAggregator>>,
        db_pool: Option<Arc<Pool<Sqlite>>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            rest_url: rest_url.to_string(),
            rest_client,
            parser,
            aggregator,
            db_pool,
        }
    }

    //todo убрать
    pub async fn connect(&self) -> Result<(), String> {
        info!("Connecting to {} API at {}", self.name, self.rest_url);
        Ok(())
    }

    /// Getting data from API
    pub async fn run(&self, urls: &[(String, String, String)]) -> Result<(), String> {
        // 1. Collect (symbol, timeframe) before the loop
        let keys: Vec<(String, String)> = urls
            .iter()
            .map(|(key1, key2, _)| (key1.clone(), key2.clone()))
            .collect();

        // 2. Call build_handlers() once before the loop to build a chain of handlers for filtering
        if let Some(aggregator) = self.aggregator.as_ref() {
            aggregator.build_handlers(&keys, &self.db_pool).await;
        } else {
            error!("CandleAggregator is not set in ExchangeBuilder");
        }

        // 3. In the loop we only receive and process data
        for (key1, key2, url) in urls {
            match self.rest_client.get(url).await {
                Ok(data) => {
                    // Parsing the data
                    match self.parser.parse(&data, key1) {
                        Ok(parsed_data) => {
                            if let Some(aggregator) = self.aggregator.as_ref() {
                                // aggregator.build_handlers(&keys, &self.db_pool);
                                /* Here you can theoretically send the result of several requests from different Url */
                                aggregator.http_response_process(parsed_data).await;
                            } else {
                                error!("CandleAggregator is not set in ExchangeBuilder");
                            }
                        }
                        Err(parse_error) => {
                            eprintln!("Failed to parse data from {}: {}", url, parse_error);
                        }
                    }
                }
                Err(fetch_error) => {
                    eprintln!("Failed to fetch data from {}: {}", url, fetch_error);
                }
            }
        }

        Ok(())
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
            "poloniex" => &settings.poloniex_rest_url_base,
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
    MissingParser,
    MissingCandleAggregator,
    MissingDBPool,
}

pub struct ExchangeBuilder {
    name: Option<String>,
    rest_url: Option<String>,
    rest_client: Option<Box<dyn RestClient>>,
    db_pool: Option<Arc<Pool<Sqlite>>>,
    parser: Option<KlineParser>,
    aggregator: Option<Arc<CandleAggregator>>,
}

impl ExchangeBuilder {
    // Create a new empty Builder
    pub fn new() -> Self {
        let aggregator = CandleAggregator::get_instance();
        Self {
            name: None,
            rest_url: None,
            rest_client: None,
            db_pool: None,
            parser: None,
            aggregator: None,
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
    pub fn set_target_db(mut self, db_pool: Pool<Sqlite>) -> Self {
        self.db_pool = Some(Arc::new(db_pool));
        self
    }

    pub fn set_parser(mut self, parser: KlineParser) -> Self {
        self.parser = Some(parser);
        self
    }

    pub fn set_aggregator(mut self, aggregator: Arc<CandleAggregator>) -> Self {
        self.aggregator = Some(aggregator);
        self
    }

    // Build Exchange, verifying that all parameters are present
    pub fn build(self) -> Result<Exchange, ExchangeBuilderError> {
        let name = self.name.ok_or(ExchangeBuilderError::MissingName)?;
        let rest_url = self.rest_url.ok_or(ExchangeBuilderError::MissingRestUrl)?;
        let rest_client = self
            .rest_client
            .ok_or(ExchangeBuilderError::MissingRestClient)?;
        let parser = self.parser.ok_or(ExchangeBuilderError::MissingParser)?;
        let aggregator = self.aggregator.clone();
        let pool = self.db_pool.ok_or(ExchangeBuilderError::MissingDBPool)?;
        Ok(Exchange::new(
            &name,
            &rest_url,
            rest_client,
            parser,
            aggregator,
            Some(pool),
        ))
    }
}
