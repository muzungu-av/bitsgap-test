#!/bin/bash

ROOT_DIR="src"

mkdir -p "$ROOT_DIR"

declare -a dirs=(
    "config/mod.rs"
    "config/settings.rs"
    "http_client/mod.rs"
    "http_client/rest_client.rs"
    "http_client/error.rs"
    "websocket_client/mod.rs"
    "websocket_client/ws_client.rs"
    "websocket_client/message.rs"
    "parser/mod.rs"
    "parser/recent_trade.rs"
    "parser/kline.rs"
    "database/mod.rs"
    "database/connection.rs"
    "database/models.rs"
    "aggregator/mod.rs"
    "aggregator/candle_aggregator.rs"
    "main.rs"
    "lib.rs"
)

for file in "${dirs[@]}"; do
    dir_path=$(dirname "$ROOT_DIR/$file")
    mkdir -p "$dir_path"
    touch "$ROOT_DIR/$file"
done