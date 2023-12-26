use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, ORIGIN, REFERER, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

use crate::{ReAPI, BEARER_TOKEN};

#[async_trait]
pub trait FavoriteTweets {
    async fn like_tweet(&self, tweet_id: &String) -> Result<FavoriteTweetResp, Box<dyn Error>>;
}

#[async_trait]
impl FavoriteTweets for ReAPI {
    async fn like_tweet(&self, tweet_id: &String) -> Result<FavoriteTweetResp, Box<dyn Error>> {
        let url: String =
            format!("https://twitter.com/i/api/graphql/lI07N6Otwv1PhnEgXILM7A/FavoriteTweet");
        let variables = json!(
            {"variables":{"tweet_id":tweet_id.to_string()},"queryId":"lI07N6Otwv1PhnEgXILM7A"}
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", BEARER_TOKEN)).unwrap(),
        );
        headers.insert(
            "X-CSRF-Token",
            HeaderValue::from_str(&self.csrf_token).unwrap(),
        );
        headers.insert(ORIGIN, HeaderValue::from_static("https://twitter.com"));
        headers.insert(
            REFERER,
            HeaderValue::from_static(
                "https://twitter.com/MyHongKongDoll/status/1739460075801149771",
            ),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"));
        headers.insert(
            "Sec-Ch-Ua",
            HeaderValue::from_static(
                r#""Not_A Brand";v="8", "Chromium";v="120", "Google Chrome";v="120""#,
            ),
        );
        headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
        headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static(r#""macOS""#));
        headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
        headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
        headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
        let req = self
            .client
            .post(url)
            .headers(headers)
            .json(&variables)
            .build()
            .unwrap();
        let text = self
            .client
            .execute(req)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("text {:?}", text);
        let res: FavoriteTweetResp = serde_json::from_str(&text)?;
        return Ok(res);
    }
}

extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct FavoriteTweetResp {
    #[serde(rename = "data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "favorite_tweet")]
    pub favorite_tweet: String,
}
