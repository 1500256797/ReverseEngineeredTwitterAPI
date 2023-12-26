use async_trait::async_trait;
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
        let req = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
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
