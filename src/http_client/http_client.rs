use super::HttpClientError;
use reqwest::{self, Client};
use std::{error::Error, future::Future, pin::Pin};

pub trait RestClient: Send + Sync {
    fn get<'a>(
        &'a self,
        url: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error>>> + Send + 'a>>;
}

pub struct ReqwestClient {
    client: Client, // Используем неблокирующий клиент
}

impl ReqwestClient {
    pub fn new() -> Self {
        ReqwestClient {
            client: reqwest::Client::new(),
        }
    }
}

impl RestClient for ReqwestClient {
    fn get<'a>(
        &'a self,
        url: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            let response = self.client.get(url).send().await.map_err(|err| {
                Box::new(HttpClientError::new(&format!(
                    "Failed to send request: {}",
                    err
                ))) as Box<dyn Error>
            })?;
            let text = response.text().await.map_err(|err| {
                Box::new(HttpClientError::new(&format!(
                    "Failed to read response text: {}",
                    err
                ))) as Box<dyn Error>
            })?;
            Ok(text)
        })
    }
}
