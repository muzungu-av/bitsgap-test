mod aggregator;
mod config;
mod database;
mod exchange;
mod http_client;
mod parser;
mod websocket_client;
use config::settings::Settings;
use tracing::{error, info, warn};
use tracing_subscriber::FmtSubscriber;

use database::establish_connection;
use exchange::ExchangeFactory;

#[tokio::main]
async fn main() {
    // Configuring logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO) // Logging level
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    //todo
    info!("Info logging level is enabled!");
    warn!("Warn logging level is enabled!");
    error!("Error logging level is enabled!");

    let settings = Settings::from_env();

    info!("Database URL: {}", settings.db_url);
    info!("EXCHANGE: {}", settings.exchange);

    info!("Starting application...");

    // Create exchange according to the settings
    match ExchangeFactory::create(&settings) {
        Ok(exchange) => {
            info!(
                "Exchange created: name = {}, rest_url = {}",
                exchange.name, exchange.rest_url
            );
        }
        Err(err) => {
            error!("Failed to create exchange: {:?}", err);
            panic!("Stop")
        }
    }

    // Establish database connection
    let db_pool = establish_connection(&settings.db_url).await;

    // You can pass the db_pool to other parts of the application
    info!("Database connection established!");

    // Establishing a connection to the database
    let db_pool = establish_connection(&settings.db_url).await;

    // pass the pool to other parts of the application
    info!("Подключение к SQLite установлено!");
}
