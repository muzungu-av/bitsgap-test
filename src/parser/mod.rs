pub mod kline;
pub mod recent_trade;
use std::collections::HashMap;

use kline::{Kline, VBS};
use serde_json::Value;

pub struct KlineParser;

impl KlineParser {
    pub fn new() -> Self {
        KlineParser
    }

    // fn group_klines_by_pair_and_time_frame(
    //     klines: Vec<Kline>,
    // ) -> HashMap<(String, String), Vec<Kline>> {
    //     let mut grouped_klines: HashMap<(String, String), Vec<Kline>> = HashMap::new();

    //     for kline in klines {
    //         let key = (kline.pair.clone(), kline.time_frame.clone());
    //         grouped_klines
    //             .entry(key)
    //             .or_insert_with(Vec::new)
    //             .push(kline);
    //     }

    //     grouped_klines
    // }

    pub fn parse(
        &self,
        response: &str,
        pair: &str,
    ) -> Result<HashMap<(String, String), Vec<Kline>>, String> {
        match serde_json::from_str::<Vec<Vec<Value>>>(response) {
            Ok(parsed) => {
                let grouped_klines = parsed
                    .into_iter()
                    .flat_map(|item| {
                        if item.len() != 14 {
                            return None; // Пропускаем невалидные элементы
                        }

                        // Создаем VBS из данных
                        let vbs = VBS::from_data(&item)?;

                        Some(Kline {
                            pair: pair.to_string(),
                            time_frame: item[11]
                                .as_str()
                                .map(|s| s.to_string())
                                .unwrap_or_default(),
                            o: item[2]
                                .as_str()
                                .and_then(|s| s.parse::<f64>().ok())
                                .unwrap_or_default(),
                            h: item[1]
                                .as_str()
                                .and_then(|s| s.parse::<f64>().ok())
                                .unwrap_or_default(),
                            l: item[0]
                                .as_str()
                                .and_then(|s| s.parse::<f64>().ok())
                                .unwrap_or_default(),
                            c: item[3]
                                .as_str()
                                .and_then(|s| s.parse::<f64>().ok())
                                .unwrap_or_default(),
                            utc_begin: item[12].as_i64().unwrap_or(0),
                            volume_bs: vbs,
                        })
                    })
                    .fold(
                        HashMap::new(),
                        |mut acc: HashMap<(String, String), Vec<Kline>>, kline| {
                            let key = (kline.pair.clone(), kline.time_frame.clone());
                            acc.entry(key).or_insert_with(Vec::new).push(kline);
                            acc
                        },
                    );

                Ok(grouped_klines)
            }
            Err(e) => Err(format!("Failed to parse response: {}", e)),
        }
    }
}

//todo на последней свече все меняется.
