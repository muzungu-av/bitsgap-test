use dotenvy::dotenv;
use std::env;

pub struct Settings {
    pub exchange: String,
    pub poloniex_rest_url: String,
    pub poloniex_ws_url: String,
    pub binance_rest_url: String,
    pub binance_ws_url: String,
    pub db_url: String,
}

impl Settings {
    pub fn from_env() -> Self {
        dotenv().ok(); // Loading variables from .env

        Settings {
            exchange: env::var("EXCHANGE").expect("EXCHANGE must be set"),
            poloniex_rest_url: env::var("POLONIEX_REST_URL")
                .expect("POLONIEX_REST_URL must be set"),
            poloniex_ws_url: env::var("POLONIEX_WS_URL").expect("POLONIEX_WS_URL must be set"),
            binance_rest_url: env::var("BINANCE_REST_URL").expect("BINANCE_REST_URL must be set"),
            binance_ws_url: env::var("BINANCE_WS_URL").expect("BINANCE_WS_URL must be set"),
            db_url: env::var("DB_URL").expect("DB_URL must be set"),
        }
    }
}
