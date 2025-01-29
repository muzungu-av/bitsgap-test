use std::fmt;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Kline {
    pub pair: String,       // Название пары
    pub time_frame: String, // Таймфрейм (1m, 15m и т.д.)
    pub o: f64,             // Цена открытия
    pub h: f64,             // Максимальная цена
    pub l: f64,             // Минимальная цена
    pub c: f64,             // Цена закрытия
    pub utc_begin: i64,
    // pub(crate) volume_bs: VBS,
}

impl fmt::Display for Kline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Kline {{ pair: {}, time_frame: {}, open: {:.6}, high: {:.6}, low: {:.6}, close: {:.6}, utc_begin: {} }}",
            self.pair,
            self.time_frame,
            self.o,
            self.h,
            self.l,
            self.c,
            self.utc_begin,
        )
    }
}

#[derive(Debug, Clone)]
pub struct VBS {
    pub buy_base: f64,   // Объём покупок в базовой валюте - buyTakerQuantity
    pub sell_base: f64,  // Объём продаж в базовой валюте  - quantity
    pub buy_quote: f64,  // Объём покупок в котируемой валюте - buyTakerAmount
    pub sell_quote: f64, // Объём продаж в котируемой валюте  - amount
}
impl VBS {
    pub fn from_data(data: &[Value]) -> Option<Self> {
        if data.len() < 14 {
            return None; // Недостаточно данных
        }

        Some(VBS {
            buy_base: data[7].as_str()?.parse().ok()?, // buyTakerQuantity
            sell_base: data[5].as_str()?.parse().ok()?, // quantity
            buy_quote: data[6].as_str()?.parse().ok()?, // buyTakerAmount
            sell_quote: data[4].as_str()?.parse().ok()?, // amount
        })
    }
}
