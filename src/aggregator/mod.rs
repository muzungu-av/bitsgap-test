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
        self: Arc<Self>, // Pass self as Arc<Self>
        keys: &[(String, String)],
        db_pool: Arc<Pool<Sqlite>>,
    ) {
        let db_pool = db_pool.clone(); // Cloning so we don't have to keep the link
        let self_clone = Arc::clone(&self); // Clone self before we pass it to the asynchronous task
        let handler = Arc::new(move |data: &mut HashMap<(String, String), Vec<Kline>>| {
            let mut keys_to_remove = Vec::new();
            for (key, klines) in data.iter() {
                let key = key.clone();
                let klines = klines.clone();
                keys_to_remove.push(key.clone());

                let db_pool = db_pool.clone();
                let self_clone = Arc::clone(&self_clone); // Clone self for each task

                //Using tokio::task::spawn_blocking to prevent async runtime blocking when saving klines
                tokio::task::spawn_blocking(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap(); // Create a temporary runtime
                    rt.block_on(async {
                        let chain = self_clone.chain.lock().await;
                        if let Some(last_kline) = klines.iter().max_by_key(|k| k.utc_begin) {
                            chain
                                .update_last_kline(key.clone(), last_kline.clone())
                                .await;
                        }
                        match save_klines(&db_pool, &klines).await {
                            Ok(_) => debug!("Save klines completed"),
                            Err(e) => error!("Failed to save klines: {}", e),
                        }
                    });
                });
            }

            for key in keys_to_remove {
                data.remove(&key);
            }

            true
        });

        // Clone self when calling add_handler
        self.chain.lock().await.add_handler(handler);
    }

    pub async fn http_response_process(
        &self,
        mut grouped_kline: HashMap<(String, String), Vec<Kline>>,
    ) {
        // We use block_in_place to perform synchronous blocking in an asynchronous context
        tokio::task::block_in_place(|| {
            let chain = self.chain.blocking_lock(); // Synchronous access
            chain.execute(&mut grouped_kline);
        });
    }
}

pub struct FilterChain {
    handlers: Vec<Arc<dyn Fn(&mut HashMap<(String, String), Vec<Kline>>) -> bool + Send + Sync>>,
    last_klines: Mutex<HashMap<(String, String), Kline>>, // this is where we keep all the latest Kline
}

impl FilterChain {
    pub fn new() -> Self {
        FilterChain {
            handlers: Vec::new(),
            last_klines: Mutex::new(HashMap::new()),
        }
    }

    pub async fn update_last_kline(&self, key: (String, String), kline: Kline) {
        let mut last_klines = self.last_klines.lock().await;
        last_klines.insert(key, kline);
    }

    pub async fn get_last_kline(&self, key: &(String, String)) -> Option<Kline> {
        let last_klines = self.last_klines.lock().await;
        last_klines.get(key).cloned()
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
