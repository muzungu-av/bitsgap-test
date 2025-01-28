use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

use crate::parser::kline::Kline;

pub struct CandleAggregator {
    // Пока без состояния, добавим позже
}

impl CandleAggregator {
    pub fn get_instance() -> &'static Mutex<Self> {
        static INSTANCE: Lazy<Mutex<CandleAggregator>> =
            Lazy::new(|| Mutex::new(CandleAggregator {}));
        &INSTANCE
    }

    pub fn process(&self, grouped_kline: HashMap<(String, String), Vec<Kline>>) {
        for (key, klines) in grouped_kline {
            println!("Key: {:?}, Count: {}", key, klines.len());
        }
    }
}
