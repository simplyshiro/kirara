use reqwest::header::{COOKIE, HeaderMap, HeaderValue, REFERER, USER_AGENT};
use serde::Deserialize;
use std::error::Error;

use crate::game::Game;

const FIREFOX_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0";
const HOYOLAB_REFERER: &str = "https://act.hoyolab.com/";

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
    pub async fn claim_daily_reward(&self, game: Game) -> Result<Response, reqwest::Error> {
        self.client
            .post(game.reward_url())
            .header("x-rpc-signgame", game.id())
            .send()
            .await?
            .json::<Response>()
            .await
    }

    pub fn new(cookies: &str) -> Result<Self, Box<dyn Error>> {
        let mut headers = HeaderMap::new();

        headers.insert(COOKIE, HeaderValue::from_str(cookies)?);
        headers.insert(USER_AGENT, HeaderValue::from_static(FIREFOX_USER_AGENT));
        headers.insert(REFERER, HeaderValue::from_static(HOYOLAB_REFERER));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
    }
}
