pub mod connection;
pub mod models;

pub use connection::DatabaseConnection;
pub use models::{KlineModel, RecentTradeModel};

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::Pool;
use sqlx::sqlite::SqlitePool;

/// Sets up a connection to a SQLite database
pub async fn establish_connection(filename: &str) -> Pool<sqlx::Sqlite> {
    let options = SqliteConnectOptions::new()
        .filename(filename) // Database path
        .create_if_missing(true); // Creates a file if it does not exist

    SqlitePoolOptions::new()
        .max_connections(5) // Maximum of 5 connections in a pool
        .connect_with(options) // Establishing a connection
        .await
        .expect("Database connection error")
}


/// Creates a test database in memory
pub async fn get_test_database_pool() -> SqlitePool {
    SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Error connecting to the test database")
}


/*
 *  Test module
 */
#[cfg(test)] 
mod tests {
    use super::*; // import get_test_database_pool
    use sqlx::{query, Row};

    #[tokio::test] // Asynchronous test
    async fn test_database_with_recent_trades_and_klines() {
        // 1. Create a connection pool to the test base
        let pool = get_test_database_pool().await;

        // 2. Create tables recent_trades and klines
        query(
            r#"
            CREATE TABLE recent_trades (
                tid TEXT PRIMARY KEY,
                pair TEXT NOT NULL,
                price TEXT NOT NULL,
                amount TEXT NOT NULL,
                side TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            );
            CREATE TABLE klines (
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
            "#
        )
        .execute(&pool)
        .await
        .expect("Не удалось создать таблицы");

        // 3. Insert test data into recent_trades
        query(
            r#"
            INSERT INTO recent_trades (tid, pair, price, amount, side, timestamp)
            VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind("1")
        .bind("BTC_USDT")
        .bind("30000.50")
        .bind("0.01")
        .bind("buy")
        .bind(1737656100)
        .execute(&pool)
        .await
        .expect("Failed to insert data into recent_trades");

        // 4. Check the inserted data in recent_trades
        let row = query(
            r#"
            SELECT tid, pair, price, amount, side, timestamp
            FROM recent_trades
            WHERE tid = ?
            "#
        )
        .bind("1")
        .fetch_one(&pool)
        .await
        .expect("Failed to select data from recent_trades");

        let tid: String = row.get("tid");
        let pair: String = row.get("pair");
        let price: String = row.get("price");
        let amount: String = row.get("amount");
        let side: String = row.get("side");
        let timestamp: i64 = row.get("timestamp");

        assert_eq!(tid, "1");
        assert_eq!(pair, "BTC_USDT");
        assert_eq!(price, "30000.50");
        assert_eq!(amount, "0.01");
        assert_eq!(side, "buy");
        assert_eq!(timestamp, 1737656100);

        // 5. Inserting test data into klines
        query(
            r#"
            INSERT INTO klines (
                pair, time_frame, o, h, l, c, utc_begin, buy_base, sell_base, buy_quote, sell_quote
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind("BTC_USDT")
        .bind("1m")
        .bind(30000.0) // open
        .bind(30100.0) // high
        .bind(29900.0) // low
        .bind(30050.0) // close
        .bind(1690233000) // unix time open k
        .bind(0.5) // buy_base
        .bind(0.3) // sell_base
        .bind(15000.0) // buy_quote
        .bind(9000.0) // sell_quote
        .execute(&pool)
        .await
        .expect("Failed to insert data into klines");

        // 6.Checking the inserted data in klines
        let row = query(
            r#"
            SELECT pair, time_frame, o, h, l, c, utc_begin, buy_base, sell_base, buy_quote, sell_quote
            FROM klines
            WHERE pair = ? AND time_frame = ?
            "#
        )
        .bind("BTC_USDT")
        .bind("1m")
        .fetch_one(&pool)
        .await
        .expect("Failed to select data from klines");

        let pair: String = row.get("pair");
        let time_frame: String = row.get("time_frame");
        let o: f64 = row.get("o");
        let h: f64 = row.get("h");
        let l: f64 = row.get("l");
        let c: f64 = row.get("c");
        let utc_begin: i64 = row.get("utc_begin");
        let buy_base: f64 = row.get("buy_base");
        let sell_base: f64 = row.get("sell_base");
        let buy_quote: f64 = row.get("buy_quote");
        let sell_quote: f64 = row.get("sell_quote");

        assert_eq!(pair, "BTC_USDT");
        assert_eq!(time_frame, "1m");
        assert_eq!(o, 30000.0);
        assert_eq!(h, 30100.0);
        assert_eq!(l, 29900.0);
        assert_eq!(c, 30050.0);
        assert_eq!(utc_begin, 1690233000);
        assert_eq!(buy_base, 0.5);
        assert_eq!(sell_base, 0.3);
        assert_eq!(buy_quote, 15000.0);
        assert_eq!(sell_quote, 9000.0);
    }
}