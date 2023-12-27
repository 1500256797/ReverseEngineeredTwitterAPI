use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, ORIGIN, REFERER, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{ReAPI, BEARER_TOKEN};

#[async_trait]
pub trait FriendshipCreate {
    async fn friendship_create(
        &self,
        target_user_id: &String,
        referer: &String,
    ) -> Result<FriendshipCreateResp, Box<dyn Error>>;
}

#[async_trait]
impl FriendshipCreate for ReAPI {
    async fn friendship_create(
        &self,
        target_user_id: &String,
        referer: &String,
    ) -> Result<FriendshipCreateResp, Box<dyn Error>> {
        // post a form req
        let url = "https://twitter.com/i/api/1.1/friendships/create.json".to_string();
        let form_data = [
            ("include_profile_interstitial_type", "1"),
            ("include_blocking", "1"),
            ("include_blocked_by", "1"),
            ("include_followed_by", "1"),
            ("include_want_retweets", "1"),
            ("include_mute_edge", "1"),
            ("include_can_dm", "1"),
            ("include_can_media_tag", "1"),
            ("include_ext_has_nft_avatar", "1"),
            ("include_ext_is_blue_verified", "1"),
            ("include_ext_verified_type", "1"),
            ("include_ext_profile_image_shape", "1"),
            ("skip_status", "1"),
            ("user_id", target_user_id),
        ];
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
        headers.insert(REFERER, referer.parse().unwrap());
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
            .form(&form_data)
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
        let res: FriendshipCreateResp = serde_json::from_str(&text)?;
        Ok(res)
    }
}

extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct FriendshipCreateResp {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "id_str")]
    id_str: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "screen_name")]
    screen_name: String,

    #[serde(rename = "location")]
    location: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "entities")]
    entities: Entities,

    #[serde(rename = "protected")]
    protected: bool,

    #[serde(rename = "followers_count")]
    followers_count: i64,

    #[serde(rename = "fast_followers_count")]
    fast_followers_count: i64,

    #[serde(rename = "normal_followers_count")]
    normal_followers_count: i64,

    #[serde(rename = "friends_count")]
    friends_count: i64,

    #[serde(rename = "listed_count")]
    listed_count: i64,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "favourites_count")]
    favourites_count: i64,

    #[serde(rename = "utc_offset")]
    utc_offset: Option<serde_json::Value>,

    #[serde(rename = "time_zone")]
    time_zone: Option<serde_json::Value>,

    #[serde(rename = "geo_enabled")]
    geo_enabled: bool,

    #[serde(rename = "verified")]
    verified: bool,

    #[serde(rename = "statuses_count")]
    statuses_count: i64,

    #[serde(rename = "media_count")]
    media_count: i64,

    #[serde(rename = "lang")]
    lang: Option<serde_json::Value>,

    #[serde(rename = "contributors_enabled")]
    contributors_enabled: bool,

    #[serde(rename = "is_translator")]
    is_translator: bool,

    #[serde(rename = "is_translation_enabled")]
    is_translation_enabled: bool,

    #[serde(rename = "profile_background_color")]
    profile_background_color: String,

    #[serde(rename = "profile_background_image_url")]
    profile_background_image_url: Option<serde_json::Value>,

    #[serde(rename = "profile_background_image_url_https")]
    profile_background_image_url_https: Option<serde_json::Value>,

    #[serde(rename = "profile_background_tile")]
    profile_background_tile: bool,

    #[serde(rename = "profile_image_url")]
    profile_image_url: String,

    #[serde(rename = "profile_image_url_https")]
    profile_image_url_https: String,

    #[serde(rename = "profile_banner_url")]
    profile_banner_url: String,

    #[serde(rename = "profile_link_color")]
    profile_link_color: String,

    #[serde(rename = "profile_sidebar_border_color")]
    profile_sidebar_border_color: String,

    #[serde(rename = "profile_sidebar_fill_color")]
    profile_sidebar_fill_color: String,

    #[serde(rename = "profile_text_color")]
    profile_text_color: String,

    #[serde(rename = "profile_use_background_image")]
    profile_use_background_image: bool,

    #[serde(rename = "has_extended_profile")]
    has_extended_profile: bool,

    #[serde(rename = "default_profile")]
    default_profile: bool,

    #[serde(rename = "default_profile_image")]
    default_profile_image: bool,

    #[serde(rename = "pinned_tweet_ids")]
    pinned_tweet_ids: Vec<i64>,

    #[serde(rename = "pinned_tweet_ids_str")]
    pinned_tweet_ids_str: Vec<String>,

    #[serde(rename = "has_custom_timelines")]
    has_custom_timelines: bool,

    #[serde(rename = "can_dm")]
    can_dm: Option<serde_json::Value>,

    #[serde(rename = "can_media_tag")]
    can_media_tag: bool,

    #[serde(rename = "following")]
    following: bool,

    #[serde(rename = "follow_request_sent")]
    follow_request_sent: bool,

    #[serde(rename = "notifications")]
    notifications: bool,

    #[serde(rename = "muting")]
    muting: bool,

    #[serde(rename = "blocking")]
    blocking: bool,

    #[serde(rename = "blocked_by")]
    blocked_by: bool,

    #[serde(rename = "want_retweets")]
    want_retweets: bool,

    #[serde(rename = "advertiser_account_type")]
    advertiser_account_type: String,

    #[serde(rename = "advertiser_account_service_levels")]
    advertiser_account_service_levels: Vec<Option<serde_json::Value>>,

    #[serde(rename = "profile_interstitial_type")]
    profile_interstitial_type: String,

    #[serde(rename = "business_profile_state")]
    business_profile_state: String,

    #[serde(rename = "translator_type")]
    translator_type: String,

    #[serde(rename = "withheld_in_countries")]
    withheld_in_countries: Vec<Option<serde_json::Value>>,

    #[serde(rename = "followed_by")]
    followed_by: bool,

    #[serde(rename = "ext_is_blue_verified")]
    ext_is_blue_verified: bool,

    #[serde(rename = "ext_has_nft_avatar")]
    ext_has_nft_avatar: bool,

    #[serde(rename = "ext_profile_image_shape")]
    ext_profile_image_shape: String,

    #[serde(rename = "require_some_consent")]
    require_some_consent: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entities {
    #[serde(rename = "url")]
    url: Description,

    #[serde(rename = "description")]
    description: Description,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    #[serde(rename = "urls")]
    urls: Vec<Url>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "expanded_url")]
    expanded_url: String,

    #[serde(rename = "display_url")]
    display_url: String,

    #[serde(rename = "indices")]
    indices: Vec<i64>,
}
