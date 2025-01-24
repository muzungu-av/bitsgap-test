use sqlx::SqlitePool;

/*
    Database initialization (creating tables and indexes)
*/
pub async fn initialize_database(pool: &SqlitePool) {
    sqlx::query(
        r#"
        -- Создание таблицы recent_trades
        CREATE TABLE IF NOT EXISTS recent_trades (
            tid TEXT PRIMARY KEY,
            pair TEXT NOT NULL,
            price TEXT NOT NULL,
            amount TEXT NOT NULL,
            side TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        );

        -- Создание таблицы klines
        CREATE TABLE IF NOT EXISTS klines (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            pair TEXT NOT NULL,
            time_frame TEXT NOT NULL,
            o REAL NOT NULL,
            h REAL NOT NULL,
            l REAL NOT NULL,
            c REAL NOT NULL,
            utc_begin INTEGER NOT NULL,
            buy_base REAL NOT NULL,
            sell_base REAL NOT NULL,
            buy_quote REAL NOT NULL,
            sell_quote REAL NOT NULL
        );

        -- For quick retrieval of data in the right time order
        CREATE INDEX IF NOT EXISTS idx_recent_trades_timestamp ON recent_trades (timestamp);
        CREATE INDEX IF NOT EXISTS idx_klines_utc_begin ON klines (utc_begin);
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to initialize database");
}
