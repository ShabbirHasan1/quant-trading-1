use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Method, Request, RequestBuilder, Body,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize}
use sha2::Sha256;

use crate::setting::Settings;

type HmacSha256 = Hmac<Sha256>;

lazy_static! {
    static ref REQ_CLIENT: Client = Client::new();
}

pub struct TradeHttp {
    base_url: String,
}

impl TradeHttp {
    pub fn auth_headers(method: Method, request_path: String, body: String) -> HeaderMap {
        let mut headerMap = HeaderMap::new();

        let api_key = Settings::default().trading_keys.api_key;
        let secret_key = Settings::default().trading_keys.secret_key;
        let passphrase = Settings::default().trading_keys.passphrase;

        let timestamp = Utc::now()
            .to_rfc3339_opts(SecondsFormat::Millis, true)
            .as_str();

        let raw_sign = timestamp.clone().to_owned() + &method.to_string() + &request_path;
        tracing::debug!("message to sign: {}", raw_sign);
        let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes()).expect("hmac_sha256 error");
        mac.update(raw_sign.as_bytes());

        let api_key_value = HeaderValue::from_str(api_key.as_str()).expect("Invalid API key");
        let passphrase_value =
            HeaderValue::from_str(passphrase.as_str()).expect("Invalid passphrase");
        let timestamp_value = HeaderValue::from_str(timestamp).expect("Invalid timestamp");

        let sign = HeaderValue::from_str(
            general_purpose::STANDARD
                .encode(mac.finalize().into_bytes())
                .as_str(),
        )
        .expect("signature format error");
        headerMap.append("OK-ACCESS-KEY", api_key_value);
        headerMap.append("OK-ACCESS-PASSPHRASE", passphrase_value);
        headerMap.append("OK-ACCESS-TIMESTAMP", timestamp_value);
        headerMap.append("OK-ACCESS-SIGN", sign);
        headerMap
    }

    pub fn new(base_url: String) -> TradeHttp {
        TradeHttp { base_url }
    }
    pub async fn get<T>(self, url: &str, headers: Option<HeaderMap>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let cus_headers = match headers {
            Some(headers) => headers,
            None => HeaderMap::new()
        };
        let auth_headers = TradeHttp::auth_headers(Method::GET, url.to_string(), String::new());
        Ok(REQ_CLIENT
            .get(url)
            .headers(auth_headers)
            .headers(cus_headers)
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    pub async fn post<S, T>(self, url: &str, headers: Option<HeaderMap>, body: S) -> Result<T>
    where
        S: Serialize + Into<Body>,
        T: DeserializeOwned,
    {
        let cus_headers = match headers {
            Some(headers) => headers,
            None => HeaderMap::new()
        };
        let body_str = serde_json::to_string(&body).unwrap();
        let auth_headers = TradeHttp::auth_headers(Method::GET, url.to_string(), body_str);
        Ok(REQ_CLIENT.post(url).headers(auth_headers).headers(cus_headers).body(body).send().await?.json::<T>().await?)
    }
}
