pub struct Kline {
    pub pair: String,       // Название пары
    pub time_frame: String, // Таймфрейм (1m, 15m и т.д.)
    pub o: f64,             // Цена открытия
    pub h: f64,             // Максимальная цена
    pub l: f64,             // Минимальная цена
    pub c: f64,             // Цена закрытия
    pub utc_begin: i64,     // Время начала формирования свечки
    pub volume_bs: VBS,
}

pub struct VBS {
    pub buy_base: f64,   // Объём покупок в базовой валюте
    pub sell_base: f64,  // Объём продаж в базовой валюте
    pub buy_quote: f64,  // Объём покупок в котируемой валюте
    pub sell_quote: f64, // Объём продаж в котируемой валюте
}
