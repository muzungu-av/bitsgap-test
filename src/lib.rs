pub mod aggregator;
pub mod config;
pub mod database;
pub mod http_client;
pub mod parser;
pub mod websocket_client;

// экспортировать основные модули для использования как библиотека
pub use aggregator::CandleAggregator;
pub use config::settings::Settings;
pub use database::DatabaseConnection;
pub use http_client::RestClient;
pub use parser::{Kline, RecentTrade, VBS};
pub use websocket_client::WebSocketClient;
