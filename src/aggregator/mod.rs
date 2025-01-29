use crate::{database::save_klines, parser::kline::Kline};
use once_cell::sync::Lazy;
use sqlx::{Pool, Sqlite};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::{debug, error, info, Level};

pub struct CandleAggregator {
    chain: Mutex<FilterChain>, // асинхронный Mutex
    db_pool: Arc<Option<Pool<Sqlite>>>,
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
    pub fn get_instance() -> &'static Arc<Self> {
        static INSTANCE: Lazy<Arc<CandleAggregator>> = Lazy::new(|| {
            Arc::new(CandleAggregator {
                chain: Mutex::new(FilterChain::new()),
                db_pool: Arc::new(None),
            })
        });
        &INSTANCE
    }

    pub async fn build_handlers(
        &self,
        keys: &[(String, String)],
        db_pool: &Option<Arc<Pool<Sqlite>>>,
    ) {
        // Используем асинхронную блокировку для цепочки
        let mut chain = self.chain.lock().await; // Получаем асинхронный доступ
        let db_pool = db_pool;

        for (index, key) in keys.iter().enumerate() {
            let key = key.clone();
            let handler_name = format!("Handler_{}", index);

            if let Some(db_pool) = db_pool {
                let db_pool = Arc::clone(&db_pool);

                // начало кода цепочки
                let handler = Arc::new(move |data: &mut HashMap<(String, String), Vec<Kline>>| {
                    let handler_name = handler_name.clone();
                    let key = key.clone(); // по ключу будем фильтровать данные
                    let db_pool = db_pool.clone();
                    let mut data_copy = data.clone();
                    tokio::spawn(async move {
                        if let Some(klines) = data_copy.remove(&key) {
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

                            if let Err(e) = save_klines(&db_pool, &klines).await {
                                error!("Failed to save klines: {}", e);
                            }
                        }
                    });
                    true
                });
                //конец кода цепочки
                //добавляем обработчик в цепочку
                chain.add_handler(handler);
            }
        }
        println!("Chain of {} handlers", chain.handlers.len());
    }

    pub async fn http_response_process(
        &self,
        mut grouped_kline: HashMap<(String, String), Vec<Kline>>,
    ) {
        // Используем block_in_place для выполнения синхронной блокировки в асинхронном контексте
        tokio::task::block_in_place(|| {
            let chain = self.chain.blocking_lock(); // Синхронный доступ
            chain.execute(&mut grouped_kline);
        });
    }
}

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
