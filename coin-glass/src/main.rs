use chrono::{prelude::*, Months};
use serde::{Deserialize, Serialize};
use setting::Settings;

mod setting;

static BASE_URL: &str = "https://open-api.coinglass.com";
static LONG_SHORT_ACCOUNT: &str = "/public/v2/indicator/top_long_short_account_ratio";

#[derive(Debug, Serialize, Deserialize)]
pub struct LongShortAccountResponse {
    longRatio: f32,
    shortRatio: f32,
    longShortRatio: f32,
    createTime: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LongShortAccountSuccessResponse {
    code: String,
    msg: String,
    data: Vec<LongShortAccountResponse>,
}

pub async fn get_long_short_account(
    ex: String,
    pair: String,
    interval: String,
) -> Result<Vec<LongShortAccountResponse>, reqwest::Error> {
    let api_key = Settings::default().coin_glass_keys.api_key;
    let dt = Local::now();
    let now_timestamp = dt.timestamp_millis();
    let latest_six_months_timestamp = dt
        .checked_sub_months(Months::new(6))
        .unwrap()
        .timestamp_millis();
    println!(
        "timestamp: {} {}",
        latest_six_months_timestamp, now_timestamp
    );

    let res = reqwest::Client::default()
        .get(format!(
            "{}{}?ex={}&pair={}&interval={}&limit=500&start_time={}&end_time={}",
            BASE_URL,
            LONG_SHORT_ACCOUNT,
            ex,
            pair,
            interval,
            latest_six_months_timestamp,
            now_timestamp
        ))
        .header("coinglassSecret", api_key)
        .send()
        .await?
        .json::<LongShortAccountSuccessResponse>()
        .await?;

    // println!("response: {:?}", res);
    Ok(res.data)
}

// pub fn get_volume() {}

fn main() {}

#[cfg(test)]
pub mod coin_glass_test {

    use super::*;

    #[tokio::test]
    async fn get_sol_long_short_account() {
        let ex = String::from("Okex");
        let pair = String::from("SOLUSDT");
        let interval = String::from("h1");

        match get_long_short_account(ex, pair, interval).await {
            Ok(result) => {
                for dataItem in result {
                    println!("longShortRatio: {}", dataItem.longShortRatio);
                }
            }
            Err(err) => {
                println!("err: {:}", err)
            }
        }

        // if let Ok(result) = get_long_short_account(ex, pair, interval).await {
        //     println!(
        //         "c: {}, t: {}, o: {}, l: {}, h: {}",
        //         result.c, result.t, result.o, result.l, result.h
        //     );
        //     Ok(())
        // } else {
        //     Err(String::from("Error"))
        // }
    }
}
