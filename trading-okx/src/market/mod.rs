pub mod result;
use chrono::prelude::*;

use crate::network::http::TradeHttp;

static HISTORY_CANDLES: &str = "/api/v5/market/history-candles"; // 获取最近几年的历史k线数据(1s k线支持查询最近3个月的数据)
static CANDLES: &str = "/api/v5/market/candles"; //获取K线数据。K线数据按请求的粒度分组返回，K线数据每个粒度最多可获取最近1,440条。
static BASE_URL: &str = "https://www.okx.com";

pub mod Market {
    use reqwest::header::HeaderMap;

    use super::{result::CANDLES_RESULT, *};

    pub fn init() -> TradeHttp {
        TradeHttp::new(BASE_URL.to_owned())
    }

    pub fn request_headers() -> HeaderMap {
        let mut headerMap = HeaderMap::new();

        headerMap
    }

    pub fn get_history_candles() {}

    pub fn get_candles() {
        Market::init().get::<CANDLES_RESULT>(CANDLES, None);
    }
}
