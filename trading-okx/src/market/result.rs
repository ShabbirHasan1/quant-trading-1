use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CANDLES_RESULT {
    ts: String,
    o: String,
    h: String,
    l: String,
    c: String,
    vol: String,
    volCcy: String,
    volCcyQuote: String,
    confirm: String,
}
