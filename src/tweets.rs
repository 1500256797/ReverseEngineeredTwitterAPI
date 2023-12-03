use async_trait::async_trait;
use serde_json::json;
use std::error::Error;

use crate::{
    types::{homepage::UserHomePage, usertweets::UserTweetsResp},
    ReAPI, BEARER_TOKEN,
};

#[async_trait]
pub trait UserTweets {
    async fn get_user_homepage(&self, uid: &String) -> Result<UserHomePage, Box<dyn Error>>;
    async fn get_user_tweets(&self, uid: &String) -> Result<UserTweetsResp, Box<dyn Error>>;
    async fn get_user_latest_tweets(
        &self,
        uid: &String,
        msg: &String,
    ) -> Result<bool, Box<dyn Error>>;
}

#[async_trait]
impl UserTweets for ReAPI {
    async fn get_user_homepage(&self, uid: &String) -> Result<UserHomePage, Box<dyn Error>> {
        let variables = json!(
            {"userId":uid.to_string(),"count":20,"includePromotedContent":true,"withQuickPromoteEligibilityTweetFields":true,"withVoice":true,"withV2Timeline":true}
        );
        let features = json!(
            {"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"responsive_web_home_pinned_timelines_enabled":true,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"c9s_tweet_anatomy_moderator_badge_enabled":true,"tweetypie_unmention_optimization_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":false,"tweet_awards_web_tipping_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_media_download_video_enabled":false,"responsive_web_enhance_cards_enabled":false}
        );
        let query_param = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
        ];
        let req = self
            .client
            .get("https://twitter.com/i/api/graphql/VgitpdpNZ-RUIp5D1Z_D-A/UserTweets")
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
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
        let res: UserHomePage = serde_json::from_str(&text).unwrap();
        return Ok(res);
    }
    async fn get_user_latest_tweets(
        &self,
        uid: &String,
        msg: &String,
    ) -> Result<bool, Box<dyn Error>> {
        let variables = json!(
            {"userId":uid.to_string(),"count":20,"includePromotedContent":true,"withQuickPromoteEligibilityTweetFields":true,"withVoice":true,"withV2Timeline":true}
        );
        let features = json!(
            {"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"responsive_web_home_pinned_timelines_enabled":true,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"c9s_tweet_anatomy_moderator_badge_enabled":true,"tweetypie_unmention_optimization_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":false,"tweet_awards_web_tipping_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_media_download_video_enabled":false,"responsive_web_enhance_cards_enabled":false}
        );
        let query_param = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
        ];
        let req = self
            .client
            .get("https://twitter.com/i/api/graphql/VgitpdpNZ-RUIp5D1Z_D-A/UserTweets")
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
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
        let res: UserHomePage = serde_json::from_str(&text).unwrap();

        let timeline = res.data.user.result.timeline_v2;
        let instructions = timeline.timeline.instructions;
        let timeline_add_entries = instructions;

        for entry in timeline_add_entries {
            let entries = entry.entries;
            // if no entries, continue
            if entries.is_none() {
                continue;
            }
            let entries = entries.unwrap();
            for e in entries {
                let twitter_id = e.entry_id;
                let content = e.content.item_content;
                // if content is some
                if let Some(item) = content {
                    let full_text = item.tweet_results.result.unwrap().legacy.full_text;
                    println!("{}: {}", twitter_id, full_text);
                    // check if content contains the content
                    if full_text.contains(msg.as_str()) {
                        return Ok(true);
                    } else {
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }
    async fn get_user_tweets(&self, uid: &String) -> Result<UserTweetsResp, Box<dyn Error>> {
        let variables = json!(
            {"userId":uid.to_string(),"count":20,"cursor":"DAABCgABGABBxmh__9wKAAIX37Ng-Box5ggAAwAAAAIAAA".to_string(),"includePromotedContent":true,"withQuickPromoteEligibilityTweetFields":true,"withVoice":true,"withV2Timeline":true}
        );
        let features = json!(
            {"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"responsive_web_home_pinned_timelines_enabled":true,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"c9s_tweet_anatomy_moderator_badge_enabled":true,"tweetypie_unmention_optimization_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":false,"tweet_awards_web_tipping_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_media_download_video_enabled":false,"responsive_web_enhance_cards_enabled":false}
        );
        let query_param = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
        ];
        let req = self
            .client
            .get("https://twitter.com/i/api/graphql/VgitpdpNZ-RUIp5D1Z_D-A/UserTweets")
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
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
        let res: UserTweetsResp = serde_json::from_str(&text).unwrap();
        return Ok(res);
    }
}

#[cfg(test)]
mod test_telation {
    use crate::{tweets::UserTweets, ReAPI};

    async fn login(api: &mut ReAPI) -> Result<String, String> {
        dotenv::dotenv().ok();
        let name = std::env::var("TWITTER_USER_NAME").unwrap();
        let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
        api.login(&name, &pwd, "").await
    }

    #[tokio::test]
    async fn test_get_user_tweets() {
        let mut api = ReAPI::new();
        let _loggined = login(&mut api).await;
        let uid = "1439140186378567683".to_string();
        let result = api.get_user_tweets(&uid).await;
        println!("result {:?}", result);
    }

    #[tokio::test]
    async fn test_get_user_homepage() {
        let mut api = ReAPI::new();
        let _loggined = login(&mut api).await;
        let uid = "1507631541303713793".to_string();
        let result = api.get_user_homepage(&uid).await;
        println!("result {:?}", result);
    }

    #[tokio::test]
    async fn test_get_user_latest_tweets() {
        let mut api = ReAPI::new();
        let _loggined = login(&mut api).await;
        let uid = "1439140186378567683".to_string();
        let message = "#RIDO".to_string();
        let result = api.get_user_latest_tweets(&uid, &message).await;
        println!("result {:?}", result);
    }
}
