use std::error::Error;
use std::fmt;

// Структура для обработки ошибок HTTP
#[derive(Debug)]
pub struct HttpClientError {
    details: String,
}

impl HttpClientError {
    pub fn new(msg: &str) -> Self {
        HttpClientError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for HttpClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpClientError: {}", self.details)
    }
}

impl Error for HttpClientError {}
