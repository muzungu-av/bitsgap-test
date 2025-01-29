use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::{debug, info, Level};

use crate::parser::kline::Kline;

pub struct CandleAggregator {
    chain: Mutex<FilterChain>,
}

/*
    An implementation of a slightly modified “Chain of Duty” template, in which chains are built dynamically
    and correspond to the combination of timeframes and characters in env. Each handler processes its own piece of data
    passing the unsuitable ones to the next handler, so that they work like a filter system (or sieve).
    It takes the data it needs and performs the necessary actions with it (stores it in the database).

    It was designed with the ability to handle mixed data from multiple url's (different pairs and timeframes),
    sorting each data series by a separate handler, However, this has not been tested....
*/
impl CandleAggregator {
    pub fn get_instance() -> &'static Mutex<Self> {
        static INSTANCE: Lazy<Mutex<CandleAggregator>> = Lazy::new(|| {
            Mutex::new(CandleAggregator {
                chain: Mutex::new(FilterChain::new()),
            })
        });
        &INSTANCE
    }

    // builds handlers by keys
    pub fn build_handlers(&self, keys: &[(String, String)]) {
        let mut chain = self.chain.lock().unwrap();

        for (index, key) in keys.iter().enumerate() {
            let key = key.clone();
            let handler_name = format!("Handler_{}", index); // Unique name of the handler

            let handler = Arc::new(move |data: &mut HashMap<(String, String), Vec<Kline>>| {
                if let Some(klines) = data.remove(&key) {
                    if tracing::level_enabled!(Level::DEBUG) {
                        debug!(
                            "Handler is started: {} for the key: ({}, {}), Data: {:?} ",
                            handler_name, key.0, key.1, klines
                        );
                    }
                    if tracing::level_enabled!(Level::INFO) {
                        info!(
                            "Handler is started: {} for the key: ({}, {}) and {} rows",
                            handler_name,
                            key.0,
                            key.1,
                            klines.len()
                        );
                    }

                    true // Data retrieved and processed
                } else {
                    false // No data
                }
            });

            chain.add_handler(handler);
        }
        println!("Цепочка из {} обработчиков", chain.handlers.len());
    }

    // starts a chain of handlers
    pub fn http_response_process(&self, mut grouped_kline: HashMap<(String, String), Vec<Kline>>) {
        let chain = self.chain.lock().unwrap(); // Accessing the chain via Mutex
        chain.execute(&mut grouped_kline);
    }
}

/*
    The structure stores handlers, which are functions that can be run with arguments
*/
pub struct FilterChain {
    handlers: Vec<Arc<dyn Fn(&mut HashMap<(String, String), Vec<Kline>>) -> bool + Send + Sync>>,
}

impl FilterChain {
    pub fn new() -> Self {
        FilterChain {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(
        &mut self,
        handler: Arc<dyn Fn(&mut HashMap<(String, String), Vec<Kline>>) -> bool + Send + Sync>,
    ) {
        self.handlers.push(handler);
    }

    pub fn execute(&self, grouped_kline: &mut HashMap<(String, String), Vec<Kline>>) {
        for handler in &self.handlers {
            if handler(grouped_kline) {}
        }
    }
}
