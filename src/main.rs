mod aggregator;
mod config;
mod database;
mod http_client;
mod parser;
mod websocket_client;

use config::settings::Settings;
use database::DatabaseConnection;
use http_client::RestClient;
use websocket_client::WebSocketClient;

use database::establish_connection;

#[tokio::main]
async fn main() {
    // 
    let settings = Settings {
        api_key: "your_api_key".to_string(),
        api_secret: "your_api_secret".to_string(),
        rest_url: "https://poloniex.com/rest".to_string(),
        ws_url: "wss://poloniex.com/ws".to_string(),
        db_url: "postgres://user:password@localhost/database".to_string(),
    };

    // let db_connection = DatabaseConnection::new(&settings.db_url);

    // let http_client = RestClient::new(&settings.rest_url);

    // let ws_client = WebSocketClient::new(&settings.ws_url);

    println!("Starting application...");

    // Устанавливаем соединение с базой данных
    let db_pool = establish_connection("db.sqlite").await;

    // Здесь вы можете передать пул в другие части приложения
    println!("Подключение к SQLite установлено!");
}

