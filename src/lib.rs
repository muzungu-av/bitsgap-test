pub mod aggregator;
pub mod config;
pub mod database;
pub mod http_client;
pub mod parser;
pub mod websocket_client;

// export core modules for use as a library
pub use aggregator::CandleAggregator;
pub use config::settings::Settings;
pub use parser::{Kline, RecentTrade, VBS};
pub use websocket_client::WebSocketClient;
