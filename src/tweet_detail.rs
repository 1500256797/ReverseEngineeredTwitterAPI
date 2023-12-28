use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, ORIGIN, REFERER, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
extern crate serde_json;
use crate::{ReAPI, BEARER_TOKEN};

#[async_trait]
pub trait TweetDetail {
    async fn get_tweet_detail(
        &self,
        tweet_url: &String,
        referer: &String,
    ) -> Result<TweetDetailResp, Box<dyn Error>>;
}

#[async_trait]
impl TweetDetail for ReAPI {
    async fn get_tweet_detail(
        &self,
        tweet_url: &String,
        referer: &String,
    ) -> Result<TweetDetailResp, Box<dyn Error>> {
        let url =
            "https://twitter.com/i/api/graphql/-H4B_lJDEA-O_7_qWaRiyg/TweetDetail".to_string();
        // get focalTweetId from tweet_url
        let focal_tweet_id = tweet_url.split("/").last().unwrap();
        let variables = json!(
            {"focalTweetId":focal_tweet_id,"with_rux_injections":false,"includePromotedContent":true,"withCommunity":true,"withQuickPromoteEligibilityTweetFields":true,"withBirdwatchNotes":true,"withVoice":true,"withV2Timeline":true}
        );
        let features = json!(
            {"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"c9s_tweet_anatomy_moderator_badge_enabled":true,"tweetypie_unmention_optimization_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":false,"tweet_awards_web_tipping_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"rweb_video_timestamps_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_media_download_video_enabled":false,"responsive_web_enhance_cards_enabled":false}
        );

        let field_toggles = json!(
            {"withArticleRichContentState":false}
        );
        let query_param = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
            ("fieldToggles", field_toggles.to_string()),
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
            .get(url)
            .headers(headers)
            .query(&query_param)
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
        let resp: TweetDetailResp = serde_json::from_str(&text).unwrap();
        Ok(resp)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TweetDetailResp {
    #[serde(rename = "data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "threaded_conversation_with_injections_v2")]
    pub threaded_conversation_with_injections_v2: ThreadedConversationWithInjectionsV2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadedConversationWithInjectionsV2 {
    #[serde(rename = "instructions")]
    pub instructions: Vec<Instruction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instruction {
    #[serde(rename = "type")]
    pub instruction_type: Option<String>,

    #[serde(rename = "entries")]
    pub entries: Option<Vec<Entry>>,

    #[serde(rename = "direction")]
    pub direction: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(rename = "entryId")]
    pub entry_id: Option<String>,

    #[serde(rename = "sortIndex")]
    pub sort_index: Option<String>,

    #[serde(rename = "content")]
    pub content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "entryType")]
    pub entry_type: Option<String>,

    #[serde(rename = "__typename")]
    pub typename: Option<String>,

    #[serde(rename = "itemContent")]
    pub item_content: Option<ItemContent>,

    #[serde(rename = "items")]
    pub items: Option<Vec<Option<serde_json::Value>>>,

    #[serde(rename = "displayType")]
    pub display_type: Option<String>,

    #[serde(rename = "header")]
    pub header: Option<Header>,

    #[serde(rename = "clientEventInfo")]
    pub client_event_info: Option<ClientEventInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientEventInfo {
    #[serde(rename = "component")]
    pub component: Option<String>,

    #[serde(rename = "details")]
    pub details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    #[serde(rename = "conversationDetails")]
    pub conversation_details: ConversationDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConversationDetails {
    #[serde(rename = "conversationSection")]
    pub conversation_section: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    #[serde(rename = "displayType")]
    pub display_type: Option<String>,

    #[serde(rename = "text")]
    pub text: Option<String>,

    #[serde(rename = "socialContext")]
    pub social_context: SocialContext,

    #[serde(rename = "sticky")]
    pub sticky: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SocialContext {
    #[serde(rename = "type")]
    pub social_context_type: Option<String>,

    #[serde(rename = "contextType")]
    pub context_type: Option<String>,

    #[serde(rename = "text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemContent {
    #[serde(rename = "itemType")]
    pub item_type: Option<String>,

    #[serde(rename = "__typename")]
    pub typename: Option<String>,

    #[serde(rename = "tweet_results")]
    pub tweet_results: Option<TweetResults>,

    #[serde(rename = "tweetDisplayType")]
    pub tweet_display_type: Option<String>,

    #[serde(rename = "hasModeratedReplies")]
    pub has_moderated_replies: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TweetResults {
    #[serde(rename = "result")]
    pub result: TweetResultsResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TweetResultsResult {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,

    #[serde(rename = "rest_id")]
    pub rest_id: Option<String>,

    #[serde(rename = "has_birdwatch_notes")]
    pub has_birdwatch_notes: Option<bool>,

    #[serde(rename = "core")]
    pub core: Core,

    #[serde(rename = "unmention_data")]
    pub unmention_data: UnmentionData,

    #[serde(rename = "edit_control")]
    pub edit_control: EditControl,

    #[serde(rename = "is_translatable")]
    pub is_translatable: Option<bool>,

    #[serde(rename = "views")]
    pub views: Views,

    #[serde(rename = "source")]
    pub source: Option<String>,

    #[serde(rename = "legacy")]
    pub legacy: FluffyLegacy,

    #[serde(rename = "quick_promote_eligibility")]
    pub quick_promote_eligibility: QuickPromoteEligibility,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Core {
    #[serde(rename = "user_results")]
    pub user_results: UserResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResults {
    #[serde(rename = "result")]
    pub result: UserResultsResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResultsResult {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "rest_id")]
    pub rest_id: Option<String>,

    #[serde(rename = "affiliates_highlighted_label")]
    pub affiliates_highlighted_label: UnmentionData,

    #[serde(rename = "has_graduated_access")]
    pub has_graduated_access: Option<bool>,

    #[serde(rename = "is_blue_verified")]
    pub is_blue_verified: Option<bool>,

    #[serde(rename = "profile_image_shape")]
    pub profile_image_shape: Option<String>,

    #[serde(rename = "legacy")]
    pub legacy: PurpleLegacy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnmentionData {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleLegacy {
    #[serde(rename = "can_dm")]
    pub can_dm: Option<bool>,

    #[serde(rename = "can_media_tag")]
    pub can_media_tag: Option<bool>,

    #[serde(rename = "created_at")]
    pub created_at: Option<String>,

    #[serde(rename = "default_profile")]
    pub default_profile: Option<bool>,

    #[serde(rename = "default_profile_image")]
    pub default_profile_image: Option<bool>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "entities")]
    pub entities: PurpleEntities,

    #[serde(rename = "fast_followers_count")]
    pub fast_followers_count: i64,

    #[serde(rename = "favourites_count")]
    pub favourites_count: i64,

    #[serde(rename = "followers_count")]
    pub followers_count: i64,

    #[serde(rename = "friends_count")]
    pub friends_count: i64,

    #[serde(rename = "has_custom_timelines")]
    pub has_custom_timelines: Option<bool>,

    #[serde(rename = "is_translator")]
    pub is_translator: Option<bool>,

    #[serde(rename = "listed_count")]
    pub listed_count: i64,

    #[serde(rename = "location")]
    pub location: Option<String>,

    #[serde(rename = "media_count")]
    pub media_count: i64,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "needs_phone_verification")]
    pub needs_phone_verification: Option<bool>,

    #[serde(rename = "normal_followers_count")]
    pub normal_followers_count: i64,

    #[serde(rename = "pinned_tweet_ids_str")]
    pub pinned_tweet_ids_str: Vec<Option<serde_json::Value>>,

    #[serde(rename = "possibly_sensitive")]
    pub possibly_sensitive: Option<bool>,

    #[serde(rename = "profile_banner_url")]
    pub profile_banner_url: Option<String>,

    #[serde(rename = "profile_image_url_https")]
    pub profile_image_url_https: Option<String>,

    #[serde(rename = "profile_interstitial_type")]
    pub profile_interstitial_type: Option<String>,

    #[serde(rename = "screen_name")]
    pub screen_name: Option<String>,

    #[serde(rename = "statuses_count")]
    pub statuses_count: i64,

    #[serde(rename = "translator_type")]
    pub translator_type: Option<String>,

    #[serde(rename = "verified")]
    pub verified: Option<bool>,

    #[serde(rename = "want_retweets")]
    pub want_retweets: Option<bool>,

    #[serde(rename = "withheld_in_countries")]
    pub withheld_in_countries: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleEntities {
    #[serde(rename = "description")]
    pub description: Description,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    #[serde(rename = "urls")]
    pub urls: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditControl {
    #[serde(rename = "edit_tweet_ids")]
    pub edit_tweet_ids: Vec<Option<String>>,

    #[serde(rename = "editable_until_msecs")]
    pub editable_until_msecs: Option<String>,

    #[serde(rename = "is_edit_eligible")]
    pub is_edit_eligible: Option<bool>,

    #[serde(rename = "edits_remaining")]
    pub edits_remaining: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyLegacy {
    #[serde(rename = "bookmark_count")]
    pub bookmark_count: i64,

    #[serde(rename = "bookmarked")]
    pub bookmarked: Option<bool>,

    #[serde(rename = "created_at")]
    pub created_at: Option<String>,

    #[serde(rename = "conversation_id_str")]
    pub conversation_id_str: Option<String>,

    #[serde(rename = "display_text_range")]
    pub display_text_range: Vec<i64>,

    #[serde(rename = "entities")]
    pub entities: FluffyEntities,

    #[serde(rename = "favorite_count")]
    pub favorite_count: i64,

    #[serde(rename = "favorited")]
    pub favorited: Option<bool>,

    #[serde(rename = "full_text")]
    pub full_text: Option<String>,

    #[serde(rename = "is_quote_status")]
    pub is_quote_status: Option<bool>,

    #[serde(rename = "lang")]
    pub lang: Option<String>,

    #[serde(rename = "quote_count")]
    pub quote_count: i64,

    #[serde(rename = "reply_count")]
    pub reply_count: i64,

    #[serde(rename = "retweet_count")]
    pub retweet_count: i64,

    #[serde(rename = "retweeted")]
    pub retweeted: Option<bool>,

    #[serde(rename = "user_id_str")]
    pub user_id_str: Option<String>,

    #[serde(rename = "id_str")]
    pub id_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyEntities {
    #[serde(rename = "hashtags")]
    pub hashtags: Vec<Option<serde_json::Value>>,

    #[serde(rename = "symbols")]
    pub symbols: Vec<Option<serde_json::Value>>,

    #[serde(rename = "timestamps")]
    pub timestamps: Vec<Option<serde_json::Value>>,

    #[serde(rename = "urls")]
    pub urls: Vec<Option<serde_json::Value>>,

    #[serde(rename = "user_mentions")]
    pub user_mentions: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuickPromoteEligibility {
    #[serde(rename = "eligibility")]
    pub eligibility: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Views {
    #[serde(rename = "count")]
    pub count: Option<String>,

    #[serde(rename = "state")]
    pub state: Option<String>,
}
