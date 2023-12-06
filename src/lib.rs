pub mod auth;
pub mod relation;
pub mod search;
pub mod tweets;
pub mod types;
use reqwest::Client;

pub const LOGIN_URL: &str = "https://api.twitter.com/1.1/onboarding/task.json";
pub const LOGOUR_URL: &str = "https://api.twitter.com/1.1/account/logout.json";
pub const GUEST_ACTIVE_URL: &str = "https://api.twitter.com/1.1/guest/activate.json";
pub const VERIFY_CREDENTIALS_URL: &str =
    "https://api.twitter.com/1.1/account/verify_credentials.json";
pub const OAUTH_URL: &str = "https://api.twitter.com/oauth2/token";
pub const BEARER_TOKEN: &str = "AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA";
pub const APP_CONSUMER_KEY: &str = "3nVuSoBZnx6U4vzUxf5w";
pub const APP_CONSUMER_SECRET: &str = "Bcs59EFbbsdF6Sl9Ng71smgStWEGwXXKSjYvPVt7qys";

#[derive(Debug, Clone)]
pub struct ReAPI {
    pub client: Client,
    pub guest_token: String,
    pub csrf_token: String,
}

impl ReAPI {
    // set csrf_token
    pub fn set_csrf_token(&mut self, csrf_token: String) {
        self.csrf_token = csrf_token;
    }

    // set gust_token
    pub fn set_guest_token(&mut self, guest_token: String) {
        self.guest_token = guest_token;
    }

    // set client
    pub fn set_client(&mut self, client: Client) {
        self.client = client;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use reqwest::Url;
    use serde_json::json;

    use crate::{
        relation::Relation,
        tweets::UserTweets,
        types::{search::Search, tweets_analysis::TweetsAnalysis},
    };

    use super::{
        types::login::{Data, Tweet},
        *,
    };

    async fn login(api: &mut ReAPI) -> Result<String, String> {
        dotenv::dotenv().ok();
        let name = std::env::var("TWITTER_USER_NAME").unwrap();
        let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
        api.login(&name, &pwd, "").await
    }

    async fn search(api: &mut ReAPI) -> Result<Search, anyhow::Error> {
        let content = "#MemeLand";
        let cursor = "";
        api.search(content, cursor).await
    }

    async fn search_tweets_analysis(api: &mut ReAPI) -> Result<Vec<TweetsAnalysis>, anyhow::Error> {
        let content = "#MemeLand";
        let cursor = "";
        api.search_tweets_analysis(content, cursor).await
    }

    async fn check_msg(api: &mut ReAPI) -> Result<bool, Box<dyn std::error::Error>> {
        let content = "#rido".to_string();
        let uid = "1507631541303713793".to_string();
        api.get_user_latest_tweets(&uid, &content).await
    }

    #[tokio::test]
    async fn test() {
        let mut api = ReAPI::new();
        let loggined = login(&mut api).await;
        assert!(loggined.is_ok());

        let is_logged_in = api.is_logged_in().await;
        assert!(is_logged_in);

        // let result = search(&mut api).await;
        // assert!(result.is_ok());

        // let res = search_tweets(&mut api).await;
        // assert!(res.is_ok());

        // let (tweets, _) = res.unwrap();
        // assert!(tweets.len() == 0);
    }

    #[tokio::test]
    async fn test_new_from_cookies_file() {
        // let mut api = ReAPI::new();
        // let loggined = login(&mut api).await;

        let mut api = ReAPI::load_from_cookies_file().unwrap();
        let tweets_analysis_vec = search_tweets_analysis(&mut api).await;
        for tweets_analysis in tweets_analysis_vec.unwrap() {
            let pretty_json = serde_json::to_string_pretty(&tweets_analysis);
            print!("{}", pretty_json.unwrap());
        }
        // search(&mut api).await;
    }
}
