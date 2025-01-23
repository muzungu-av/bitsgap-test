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
