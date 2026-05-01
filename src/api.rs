use crate::routes::Game;

use reqwest::header::{self, COOKIE, HeaderMap, HeaderValue};
use serde::Deserialize;
use std::error::Error;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0";

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
}

#[derive(Deserialize)]
pub struct Response {
    pub message: String,
    pub retcode: i32,
}

impl Client {
    pub async fn claim_daily_reward(&self, game: Game) -> Result<self::Response, reqwest::Error> {
        self.client
            .post(game.reward_url())
            .header("x-rpc-signgame", game.id())
            .send()
            .await?
            .json::<self::Response>()
            .await
    }

    pub fn new(cookies: &str) -> Result<Self, Box<dyn Error>> {
        let mut headers = HeaderMap::new();

        headers.insert(COOKIE, HeaderValue::from_str(cookies)?);

        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_static(self::USER_AGENT),
        );

        headers.insert(
            "referer",
            HeaderValue::from_static("https://act.hoyolab.com/"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
    }
}
