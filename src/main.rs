mod aggregator;
mod config;
mod database;
mod http_client;
mod parser;
mod websocket_client;

use config::settings::Settings;
use tracing::{error, info, warn};
use tracing_subscriber::FmtSubscriber;

use database::establish_connection;

#[tokio::main]
async fn main() {
    // Настройка логирования
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO) // Уровень логирования
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // Пример логов
    info!("Info logging level is enabled!");
    warn!("Warn logging level is enabled!");
    error!("Error logging level is enabled!");

    let settings = Settings::from_env();

    info!("API Key: {}", settings.api_key);
    info!("REST URL: {}", settings.rest_url);
    info!("Database URL: {}", settings.db_url);

    // let db_connection = DatabaseConnection::new(&settings.db_url);

    // let http_client = RestClient::new(&settings.rest_url);

    // let ws_client = WebSocketClient::new(&settings.ws_url);

    info!("Starting application...");

    // Устанавливаем соединение с базой данных
    let db_pool = establish_connection(&settings.db_url).await;

    // Здесь вы можете передать пул в другие части приложения
    info!("Подключение к SQLite установлено!");
}
