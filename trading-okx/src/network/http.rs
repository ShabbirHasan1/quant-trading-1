use anyhow::Result;
use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Client, Request, RequestBuilder};
use serde::de::DeserializeOwned;

lazy_static! {
    static ref REQ_CLIENT: Client = Client::new();
}

pub struct TradeHttp {
    base_url: String,
}

impl TradeHttp {
    pub fn new(base_url: String) -> TradeHttp {
        TradeHttp { base_url }
    }
    pub async fn get<T>(self, url: &str, headers: HeaderMap) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(REQ_CLIENT
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    pub async fn post<T>(args: Request) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(RequestBuilder::from_parts(REQ_CLIENT.to_owned(), args)
            .send()
            .await?
            .json::<T>()
            .await?)
        // let url = args.url().clone();
        // let headers = args.headers().clone();
        // let body = args.body().expect("Invalid body");
        // Ok(REQ_CLIENT
        //     .to_owned()
        //     .post(url)
        //     .headers(headers)
        //     .body(body)
        //     .send()
        //     .await?
        //     .json::<T>()
        //     .await?)
    }
}
