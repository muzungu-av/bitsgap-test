use std::fmt;

#[derive(Debug)]
pub struct Kline {
    pub pair: String,       // Название пары
    pub time_frame: String, // Таймфрейм (1m, 15m и т.д.)
    pub o: f64,             // Цена открытия
    pub h: f64,             // Максимальная цена
    pub l: f64,             // Минимальная цена
    pub c: f64,             // Цена закрытия
    pub utc_begin: i64,     // Время начала формирования свечки
                            // pub volume_bs: VBS,
}

impl fmt::Display for Kline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Kline {{ pair: {}, time_frame: {}, open: {:.6}, high: {:.6}, low: {:.6}, close: {:.6}, utc_begin: {} }}",
            // "Kline {{ pair: {}, time_frame: {}, open: {:.6}, high: {:.6}, low: {:.6}, close: {:.6}, utc_begin: {}, volume: {{ amount: {:.6}, quantity: {:.6}, buy_taker_amount: {:.6}, buy_taker_quantity: {:.6} }} }}",
            self.pair,
            self.time_frame,
            self.o,
            self.h,
            self.l,
            self.c,
            self.utc_begin,
            // self.volume_bs.amount,
            // self.volume_bs.quantity,
            // self.volume_bs.buy_taker_amount,
            // self.volume_bs.buy_taker_quantity
        )
    }
}

// #[derive(Debug)]
// pub struct CandleData {
//     open: f64,
//     high: f64,
//     low: f64,
//     close: f64,
//     volume: f64,
//     quote_asset_volume: f64,
//     taker_buy_base_asset_volume: f64,
//     taker_buy_quote_asset_volume: f64,
//     number_of_trades: u64,
//     close_time: u64,
//     last_price: f64,
//     interval: String,
//     start_time: u64,
//     end_time: u64,
// }

pub struct VBS {
    pub buy_base: f64,   // Объём покупок в базовой валюте
    pub sell_base: f64,  // Объём продаж в базовой валюте
    pub buy_quote: f64,  // Объём покупок в котируемой валюте
    pub sell_quote: f64, // Объём продаж в котируемой валюте
}
