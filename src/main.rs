mod aggregator;
mod config;
mod database;
mod exchange;
mod http_client;
mod parser;
mod websocket_client;
use aggregator::CandleAggregator;
use config::settings::Settings;
use parser::KlineParser;
use tracing::{debug, error, info, warn};
use tracing_subscriber::FmtSubscriber;

use database::establish_connection;
use exchange::{Exchange, ExchangeBuilderError, ExchangeFactory};

#[tokio::main]
async fn main() {
    // Configuring logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO) // Logging level INFO or DEBUG
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    debug!("Debug logging level is enabled!");
    info!("Info logging level is enabled!");
    warn!("Warn logging level is enabled!");
    error!("Error logging level is enabled!");

    let settings = Settings::from_env();

    info!("Database URL: {}", settings.db_url);
    info!("EXCHANGE: {}", settings.exchange);

    info!("Starting application...");

    // Create and customize the exchange
    let exchange = match setup_exchange(&settings).await {
        Ok(exchange) => {
            //todo не нужно
            if let Err(err) = exchange.connect().await {
                error!("Failed to connect to exchange: {}", err);
                return;
            }
            Some(exchange)
        }
        Err(err) => {
            error!("Failed to setup exchange: {}", err);
            None
        }
    };

    if let Some(exchange) = exchange {
        info!("The Exchange process is running");

        let urls = generate_urls(
            &settings.poloniex_rest_url_base,
            &settings.poloniex_rest_url_endpoint,
            &settings.symbols,
            &settings.timeframes,
        );

        {
            for url in &urls {
                debug!("{}", url.2);
            }
        }

        if let Err(err) = exchange.run(&urls).await {
            error!("Failed to run exchange: {}", err);
        }
    } else {
        // Handling a case where an exchange has not been created
        error!("Exchange not available");
    }
    // You can pass the db_pool to other parts of the application
    info!("Finish");
}

fn generate_urls(
    base_url: &str,
    endpoint_url: &str,
    symbols: &[String],
    timeframes: &[String],
) -> Vec<(String, String, String)> {
    let mut urls = Vec::new();

    for symbol in symbols {
        for timeframe in timeframes {
            let url = endpoint_url
                .replace("{base_url}", base_url)
                .replace("{symbol}", symbol)
                .replace("{timeframe}", timeframe);
            urls.push((symbol.to_string(), timeframe.to_string(), url));
        }
    }
    urls
}

/// Creates and configures an Exchange instance
async fn setup_exchange(settings: &Settings) -> Result<Exchange, String> {
    /*** Factory returns Builder ***/
    let mut builder =
        ExchangeFactory::create(settings).map_err(|err| format!("Factory error: {}", err))?;
    debug!("The ExchangeFactory is complete ");

    // Establishing a connection to the database
    let db_pool = establish_connection(&settings.db_url).await;
    builder = builder.set_target_db(db_pool);
    debug!("Builder setting db pool is complete");

    // Create a parser
    let parser = KlineParser::new();
    builder = builder.set_parser(parser);

    // Create an aggregator
    let aggregator = CandleAggregator::get_instance().clone();
    builder = builder.set_aggregator(aggregator);

    // Assembling the Exchange object
    match builder.build() {
        Ok(exchange) => {
            debug!("The Exchange is complete ");
            Ok(exchange)
        }
        Err(ExchangeBuilderError::MissingName) => Err("Name is missing".to_string()),
        Err(ExchangeBuilderError::MissingRestUrl) => Err("REST URL is missing".to_string()),
        Err(ExchangeBuilderError::MissingRestClient) => Err("RestClient is missing".to_string()),
        Err(ExchangeBuilderError::MissingParser) => Err("Parser is missing".to_string()),
        Err(ExchangeBuilderError::MissingCandleAggregator) => {
            Err("CandleAggregator is missing".to_string())
        }
        Err(ExchangeBuilderError::MissingDBPool) => Err("DBPool is missing".to_string()),
    }
}
