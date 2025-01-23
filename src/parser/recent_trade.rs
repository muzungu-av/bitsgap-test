pub struct RecentTrade {
    pub tid: String,    // ID транзакции
    pub pair: String,   // Название валютной пары
    pub price: String,  // Цена транзакции
    pub amount: String, // Объём в базовой валюте
    pub side: String,   // Покупка или продажа
    pub timestamp: i64, // Время UTC UnixNano
}
