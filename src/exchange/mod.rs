mod error;
use std::sync::Mutex;

use sqlx::{Pool, Sqlite};
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
    pub aggregator: &'static Mutex<CandleAggregator>,
}

impl Exchange {
    pub fn new(
        name: &str,
        rest_url: &str,
        rest_client: Box<dyn RestClient>,
        parser: KlineParser,
        aggregator: &'static Mutex<CandleAggregator>,
    ) -> Self {
        Self {
            name: name.to_string(),
            rest_url: rest_url.to_string(),
            rest_client,
            parser,
            aggregator,
        }
    }

    pub async fn connect(&self) -> Result<(), String> {
        info!("Connecting to {} API at {}", self.name, self.rest_url);
        Ok(())
    }

    /// Получение данных из API
    pub async fn run(&self, urls: &Vec<(String, String, String)>) -> Result<(), String> {
        //-> Result<(String, String, String), String>
        //todo берет пока первый URLS

        for (key1, key2, url) in urls {
            match self.rest_client.get(url).await {
                Ok(data) => {
                    // Парсим данные
                    match self.parser.parse(&data, &key1) {
                        Ok(parsed_data) => {
                            // Передаем данные в Агрегатор
                            let mut aggregator = self.aggregator.lock().unwrap();

                            aggregator.process(parsed_data);
                        }
                        Err(parse_error) => {
                            eprintln!("Failed to parse data from {}: {}", url, parse_error);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch data from {}: {}", url, e);
                }
            }
        }
        Ok(())
        // match self.rest_client.get(&urls[0].2).await {
        //     Ok(data) => {
        //         let r = (urls[0].0.clone(), urls[0].1.clone(), data);
        //         println!("Response: {}", r.0);
        //         println!("Response: {}", r.1);
        //         println!("Response: {}", r.2);
        //         Ok(r)
        //     }
        //     Err(e) => Err(format!("Failed to fetch data: {}", e)),
        // }
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
}

pub struct ExchangeBuilder {
    name: Option<String>,
    rest_url: Option<String>,
    rest_client: Option<Box<dyn RestClient>>,
    db_pool: Option<Pool<Sqlite>>,
    parser: Option<KlineParser>,
    aggregator: Option<&'static Mutex<CandleAggregator>>, // Ссылка на синглтон
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
    pub fn set_target_db(&mut self, db_pool: Pool<Sqlite>) -> &Self {
        self.db_pool = Some(db_pool);
        self
    }

    pub fn set_parser(&mut self, parser: KlineParser) -> &Self {
        self.parser = Some(parser);
        self
    }

    pub fn set_aggregator(&mut self, aggregator: &'static Mutex<CandleAggregator>) -> &Self {
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
        let aggregator = self
            .aggregator
            .ok_or(ExchangeBuilderError::MissingCandleAggregator)?;
        Ok(Exchange::new(
            &name,
            &rest_url,
            rest_client,
            parser,
            aggregator,
        ))
    }
}
