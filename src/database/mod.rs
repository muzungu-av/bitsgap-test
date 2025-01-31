pub mod connection;
mod db_init;
pub mod models;

use db_init::initialize_database;
use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{query, Pool, Sqlite};
use tracing::debug;

use crate::parser::kline::Kline;

/*
    Sets up a connection to a SQLite database
*/
pub async fn establish_connection(filename: &str) -> Pool<Sqlite> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true); // Creates a file if it does not exist

    let pool = SqlitePoolOptions::new()
        .max_connections(5) // Pool - maximum of 5 connections
        .connect_with(options) // Establishing a connection
        .await
        .expect("Database connection error");

    // Database initialization
    initialize_database(&pool).await;
    debug!("Database connection established!");
    pool
}

/// Creates a test database in memory
#[allow(dead_code)]
pub async fn get_test_database_sqlitePool() -> SqlitePool {
    SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Error connecting to the test database")
}

pub async fn save_klines(db_pool: &Pool<Sqlite>, klines: &[Kline]) -> Result<(), sqlx::Error> {
    // let mut tx = db_pool.begin().await?;
    for kline in klines {
        sqlx::query(
            r#"
            INSERT INTO klines (pair, time_frame, o, h, l, c, utc_begin)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&kline.pair)
        .bind(&kline.time_frame)
        .bind(kline.o)
        .bind(kline.h)
        .bind(kline.l)
        .bind(kline.c)
        .bind(kline.utc_begin)
        .execute(db_pool) // или `&mut *tx` для транзакции
        .await?;
    }
    // tx.commit().await?;
    Ok(())
}

/*
 *  Test module
 */
#[cfg(test)]
mod tests {
    use super::*; // import get_test_database_sqlitePool
    use sqlx::{query, Row};

    #[tokio::test] // Asynchronous test
    async fn test_database_with_recent_trades_and_klines() {
        // 1. Create a connection pool to the test base
        let pool = get_test_database_sqlitePool().await;

        // Database initialization
        initialize_database(&pool).await;

        // 3. Insert test data into recent_trades
        query(
            r#"
            INSERT INTO recent_trades (tid, pair, price, amount, side, timestamp)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind("1")
        .bind("BTC_USDT")
        .bind("30000.50")
        .bind("0.01")
        .bind("buy")
        .bind(1737709931)
        .execute(&pool)
        .await
        .expect("Failed to insert data into recent_trades");

        // 4. Check the inserted data in recent_trades
        let row = query(
            r#"
            SELECT tid, pair, price, amount, side, timestamp
            FROM recent_trades
            WHERE tid = ?
            "#,
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
        assert_eq!(timestamp, 1737709931);

        // 5. Inserting test data into klines
        query(
            r#"
            INSERT INTO klines (
                pair, time_frame, o, h, l, c, utc_begin, buy_base, sell_base, buy_quote, sell_quote
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind("BTC_USDT")
        .bind("1m")
        .bind(30000.0) // open
        .bind(30100.0) // high
        .bind(29900.0) // low
        .bind(30050.0) // close
        .bind(1737709931) // unix time open k
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
        assert_eq!(utc_begin, 1737709931);
        assert_eq!(buy_base, 0.5);
        assert_eq!(sell_base, 0.3);
        assert_eq!(buy_quote, 15000.0);
        assert_eq!(sell_quote, 9000.0);
    }
}
