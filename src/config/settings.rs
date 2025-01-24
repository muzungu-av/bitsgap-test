use dotenvy::dotenv;
use std::env;

pub struct Settings {
    pub api_key: String,
    pub api_secret: String,
    pub rest_url: String,
    pub ws_url: String,
    pub db_url: String,
}

impl Settings {
    pub fn from_env() -> Self {
        dotenv().ok(); // Loading variables from .env

        Settings {
            api_key: env::var("API_KEY").expect("API_KEY must be set"),
            api_secret: env::var("API_SECRET").expect("API_SECRET must be set"),
            rest_url: env::var("REST_URL").expect("REST_URL must be set"),
            ws_url: env::var("WS_URL").expect("WS_URL must be set"),
            db_url: env::var("DB_URL").expect("DB_URL must be set"),
        }
    }
}
