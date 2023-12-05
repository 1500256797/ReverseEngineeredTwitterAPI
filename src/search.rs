use super::{
    types::login::{parse_legacy_tweet, Data, Tweet},
    ReAPI,
};
use crate::{
    types::{
        search::Search,
        tweets_analysis::{self, TweetsAnalysis},
    },
    BEARER_TOKEN,
};
use anyhow::Ok;
use serde_json::json;

const SEARCH_URL: &str = "https://twitter.com/i/api/graphql/lMv4QkY3vpla38q9tiD-tA/SearchTimeline";

impl ReAPI {
    pub async fn search_tweets_analysis(
        &self,
        query: &str,
        cursor: &str,
    ) -> std::result::Result<Vec<TweetsAnalysis>, anyhow::Error> {
        let search_result = self.search(query, cursor).await.unwrap();
        let instructions = search_result
            .data
            .search_by_raw_query
            .search_timeline
            .timeline
            .instructions
            .unwrap();

        let mut tweets_analysis: Vec<TweetsAnalysis> = vec![];

        for instruction in instructions {
            if instruction.entries.is_none() {
                continue;
            }
            let entrys = instruction.entries.unwrap();
            for entry in entrys {
                if entry.content.item_content.is_none() {
                    continue;
                }
                let tweets_results = entry.content.item_content.unwrap().tweet_results;

                let views = tweets_results.result.views;

                let legacy = tweets_results.result.legacy;
                if entry.content.items.is_none() {
                    continue;
                }
                let item_element = entry.content.items.unwrap();
                let first_item = item_element.first().unwrap();
                let advertiser_results_result =
                    first_item.item.item_content.user_results.result.clone();

                let tweets_analysis_item = TweetsAnalysis {
                    views,
                    legacy,
                    user_result: advertiser_results_result,
                };
                tweets_analysis.push(tweets_analysis_item);
            }
        }

        Ok(tweets_analysis)
    }

    pub async fn search(
        &self,
        query: &str,
        cursor: &str,
    ) -> std::result::Result<Search, anyhow::Error> {
        let mut variables = json!(
            {
                "rawQuery":     query.to_string(),
                "count":        20,
                "querySource":  "recent_search_click",
                "product":      "Top"
            }
        );
        let features = json!(
            {"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"responsive_web_home_pinned_timelines_enabled":true,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"c9s_tweet_anatomy_moderator_badge_enabled":true,"tweetypie_unmention_optimization_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":false,"tweet_awards_web_tipping_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_media_download_video_enabled":false,"responsive_web_enhance_cards_enabled":false}
        );
        if cursor.ne("") {
            variables["cursor"] = cursor.to_string().into();
        }
        // variables["product"] = "Latest".into();
        let query_params = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
        ];
        let req = self
            .client
            .get(SEARCH_URL)
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .query(&query_params)
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
        println!("{}", text);
        let res: Search = serde_json::from_str(&text).unwrap();
        return Ok(res);
    }
}
