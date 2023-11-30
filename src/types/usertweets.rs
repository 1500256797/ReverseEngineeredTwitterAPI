extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserTweetsResp {
    #[serde(rename = "data")]
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "user")]
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "result")]
    result: UserResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    #[serde(rename = "__typename")]
    typename: Typename,

    #[serde(rename = "timeline_v2")]
    timeline_v2: TimelineV2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimelineV2 {
    #[serde(rename = "timeline")]
    timeline: Timeline,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timeline {
    #[serde(rename = "instructions")]
    instructions: Vec<Instruction>,

    #[serde(rename = "metadata")]
    metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instruction {
    #[serde(rename = "type")]
    instruction_type: String,

    #[serde(rename = "entry")]
    entry: Option<PurpleEntry>,

    #[serde(rename = "entries")]
    entries: Option<Vec<EntryElement>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryElement {
    #[serde(rename = "entryId")]
    entry_id: String,

    #[serde(rename = "sortIndex")]
    sort_index: String,

    #[serde(rename = "content")]
    content: PurpleContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleContent {
    #[serde(rename = "entryType")]
    entry_type: EntryTypeEnum,

    #[serde(rename = "__typename")]
    typename: EntryTypeEnum,

    #[serde(rename = "itemContent")]
    item_content: Option<PurpleItemContent>,

    #[serde(rename = "clientEventInfo")]
    client_event_info: Option<PurpleClientEventInfo>,

    #[serde(rename = "value")]
    value: Option<String>,

    #[serde(rename = "cursorType")]
    cursor_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleClientEventInfo {
    #[serde(rename = "component")]
    component: Ent,

    #[serde(rename = "element")]
    element: Ent,

    #[serde(rename = "details")]
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    #[serde(rename = "timelinesDetails")]
    timelines_details: TimelinesDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimelinesDetails {
    #[serde(rename = "injectionType")]
    injection_type: InjectionType,

    #[serde(rename = "controllerData")]
    controller_data: ControllerData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleItemContent {
    #[serde(rename = "itemType")]
    item_type: ItemTypeEnum,

    #[serde(rename = "__typename")]
    typename: ItemTypeEnum,

    #[serde(rename = "tweet_results")]
    tweet_results: PurpleTweetResults,

    #[serde(rename = "tweetDisplayType")]
    tweet_display_type: TweetDisplayType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleTweetResults {
    #[serde(rename = "result")]
    result: PurpleResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleResult {
    #[serde(rename = "__typename")]
    typename: TweetDisplayType,

    #[serde(rename = "rest_id")]
    rest_id: String,

    #[serde(rename = "core")]
    core: PurpleCore,

    #[serde(rename = "unmention_data")]
    unmention_data: UnmentionData,

    #[serde(rename = "edit_control")]
    edit_control: PurpleEditControl,

    #[serde(rename = "is_translatable")]
    is_translatable: bool,

    #[serde(rename = "views")]
    views: Views,

    #[serde(rename = "source")]
    source: String,

    #[serde(rename = "legacy")]
    legacy: FluffyLegacy,

    #[serde(rename = "quick_promote_eligibility")]
    quick_promote_eligibility: QuickPromoteEligibility,

    #[serde(rename = "unified_card")]
    unified_card: Option<UnifiedCard>,

    #[serde(rename = "previous_counts")]
    previous_counts: Option<PreviousCounts>,

    #[serde(rename = "note_tweet")]
    note_tweet: Option<FluffyNoteTweet>,

    #[serde(rename = "quoted_status_result")]
    quoted_status_result: Option<QuotedStatusResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleCore {
    #[serde(rename = "user_results")]
    user_results: PurpleUserResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleUserResults {
    #[serde(rename = "result")]
    result: FluffyResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyResult {
    #[serde(rename = "__typename")]
    typename: Typename,

    #[serde(rename = "id")]
    id: Id,

    #[serde(rename = "rest_id")]
    rest_id: String,

    #[serde(rename = "affiliates_highlighted_label")]
    affiliates_highlighted_label: UnmentionData,

    #[serde(rename = "has_graduated_access")]
    has_graduated_access: bool,

    #[serde(rename = "is_blue_verified")]
    is_blue_verified: bool,

    #[serde(rename = "profile_image_shape")]
    profile_image_shape: ProfileImageShape,

    #[serde(rename = "legacy")]
    legacy: PurpleLegacy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnmentionData {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleLegacy {
    #[serde(rename = "following")]
    following: Option<bool>,

    #[serde(rename = "can_dm")]
    can_dm: bool,

    #[serde(rename = "can_media_tag")]
    can_media_tag: bool,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "default_profile")]
    default_profile: bool,

    #[serde(rename = "default_profile_image")]
    default_profile_image: bool,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "entities")]
    entities: Entities,

    #[serde(rename = "fast_followers_count")]
    fast_followers_count: i64,

    #[serde(rename = "favourites_count")]
    favourites_count: i64,

    #[serde(rename = "followers_count")]
    followers_count: i64,

    #[serde(rename = "friends_count")]
    friends_count: i64,

    #[serde(rename = "has_custom_timelines")]
    has_custom_timelines: bool,

    #[serde(rename = "is_translator")]
    is_translator: bool,

    #[serde(rename = "listed_count")]
    listed_count: i64,

    #[serde(rename = "location")]
    location: String,

    #[serde(rename = "media_count")]
    media_count: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "normal_followers_count")]
    normal_followers_count: i64,

    #[serde(rename = "pinned_tweet_ids_str")]
    pinned_tweet_ids_str: Vec<String>,

    #[serde(rename = "possibly_sensitive")]
    possibly_sensitive: bool,

    #[serde(rename = "profile_banner_url")]
    profile_banner_url: Option<String>,

    #[serde(rename = "profile_image_url_https")]
    profile_image_url_https: String,

    #[serde(rename = "profile_interstitial_type")]
    profile_interstitial_type: String,

    #[serde(rename = "screen_name")]
    screen_name: String,

    #[serde(rename = "statuses_count")]
    statuses_count: i64,

    #[serde(rename = "translator_type")]
    translator_type: TranslatorType,

    #[serde(rename = "url")]
    url: Option<String>,

    #[serde(rename = "verified")]
    verified: bool,

    #[serde(rename = "want_retweets")]
    want_retweets: bool,

    #[serde(rename = "withheld_in_countries")]
    withheld_in_countries: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entities {
    #[serde(rename = "description")]
    description: Description,

    #[serde(rename = "url")]
    url: Option<Description>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    #[serde(rename = "urls")]
    urls: Vec<Url>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    #[serde(rename = "display_url")]
    display_url: String,

    #[serde(rename = "expanded_url")]
    expanded_url: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "indices")]
    indices: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleEditControl {
    #[serde(rename = "edit_tweet_ids")]
    edit_tweet_ids: Option<Vec<String>>,

    #[serde(rename = "editable_until_msecs")]
    editable_until_msecs: Option<String>,

    #[serde(rename = "is_edit_eligible")]
    is_edit_eligible: Option<bool>,

    #[serde(rename = "edits_remaining")]
    edits_remaining: Option<String>,

    #[serde(rename = "initial_tweet_id")]
    initial_tweet_id: Option<String>,

    #[serde(rename = "edit_control_initial")]
    edit_control_initial: Option<EditControlInitialClass>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditControlInitialClass {
    #[serde(rename = "edit_tweet_ids")]
    edit_tweet_ids: Vec<String>,

    #[serde(rename = "editable_until_msecs")]
    editable_until_msecs: String,

    #[serde(rename = "is_edit_eligible")]
    is_edit_eligible: bool,

    #[serde(rename = "edits_remaining")]
    edits_remaining: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyLegacy {
    #[serde(rename = "bookmark_count")]
    bookmark_count: i64,

    #[serde(rename = "bookmarked")]
    bookmarked: bool,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "conversation_id_str")]
    conversation_id_str: String,

    #[serde(rename = "display_text_range")]
    display_text_range: Vec<i64>,

    #[serde(rename = "entities")]
    entities: PurpleEntit,

    #[serde(rename = "favorite_count")]
    favorite_count: i64,

    #[serde(rename = "favorited")]
    favorited: bool,

    #[serde(rename = "full_text")]
    full_text: String,

    #[serde(rename = "is_quote_status")]
    is_quote_status: bool,

    #[serde(rename = "lang")]
    lang: Lang,

    #[serde(rename = "quote_count")]
    quote_count: i64,

    #[serde(rename = "reply_count")]
    reply_count: i64,

    #[serde(rename = "retweet_count")]
    retweet_count: i64,

    #[serde(rename = "retweeted")]
    retweeted: bool,

    #[serde(rename = "user_id_str")]
    user_id_str: String,

    #[serde(rename = "id_str")]
    id_str: String,

    #[serde(rename = "retweeted_status_result")]
    retweeted_status_result: Option<RetweetedStatusResult>,

    #[serde(rename = "extended_entities")]
    extended_entities: Option<PurpleExtendedEntities>,

    #[serde(rename = "possibly_sensitive")]
    possibly_sensitive: Option<bool>,

    #[serde(rename = "possibly_sensitive_editable")]
    possibly_sensitive_editable: Option<bool>,

    #[serde(rename = "quoted_status_id_str")]
    quoted_status_id_str: Option<String>,

    #[serde(rename = "quoted_status_permalink")]
    quoted_status_permalink: Option<QuotedStatusPermalink>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleEntit {
    #[serde(rename = "user_mentions")]
    user_mentions: Vec<UserMention>,

    #[serde(rename = "urls")]
    urls: Vec<Url>,

    #[serde(rename = "hashtags")]
    hashtags: Vec<Hashtag>,

    #[serde(rename = "symbols")]
    symbols: Vec<Hashtag>,

    #[serde(rename = "media")]
    media: Option<Vec<PurpleMedia>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hashtag {
    #[serde(rename = "indices")]
    indices: Vec<i64>,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleMedia {
    #[serde(rename = "display_url")]
    display_url: String,

    #[serde(rename = "expanded_url")]
    expanded_url: String,

    #[serde(rename = "id_str")]
    id_str: String,

    #[serde(rename = "indices")]
    indices: Vec<i64>,

    #[serde(rename = "media_key")]
    media_key: String,

    #[serde(rename = "media_url_https")]
    media_url_https: String,

    #[serde(rename = "source_status_id_str")]
    source_status_id_str: Option<String>,

    #[serde(rename = "source_user_id_str")]
    source_user_id_str: Option<String>,

    #[serde(rename = "type")]
    media_type: Type,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "ext_media_availability")]
    ext_media_availability: ExtMediaAvailability,

    #[serde(rename = "features")]
    features: Option<Features>,

    #[serde(rename = "sizes")]
    sizes: Sizes,

    #[serde(rename = "original_info")]
    original_info: OriginalInfo,

    #[serde(rename = "additional_media_info")]
    additional_media_info: Option<AdditionalMediaInfo>,

    #[serde(rename = "video_info")]
    video_info: Option<VideoInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalMediaInfo {
    #[serde(rename = "monetizable")]
    monetizable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtMediaAvailability {
    #[serde(rename = "status")]
    status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Features {
    #[serde(rename = "large")]
    large: OrigClass,

    #[serde(rename = "medium")]
    medium: OrigClass,

    #[serde(rename = "small")]
    small: OrigClass,

    #[serde(rename = "orig")]
    orig: OrigClass,

    #[serde(rename = "all")]
    all: Option<All>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct All {
    #[serde(rename = "tags")]
    tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    #[serde(rename = "user_id")]
    user_id: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "screen_name")]
    screen_name: String,

    #[serde(rename = "type")]
    tag_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrigClass {
    #[serde(rename = "faces")]
    faces: Vec<FocusRect>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FocusRect {
    #[serde(rename = "x")]
    x: i64,

    #[serde(rename = "y")]
    y: i64,

    #[serde(rename = "w")]
    w: i64,

    #[serde(rename = "h")]
    h: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalInfo {
    #[serde(rename = "height")]
    height: i64,

    #[serde(rename = "width")]
    width: i64,

    #[serde(rename = "focus_rects")]
    focus_rects: Vec<FocusRect>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sizes {
    #[serde(rename = "large")]
    large: ThumbClass,

    #[serde(rename = "medium")]
    medium: ThumbClass,

    #[serde(rename = "small")]
    small: ThumbClass,

    #[serde(rename = "thumb")]
    thumb: ThumbClass,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThumbClass {
    #[serde(rename = "h")]
    h: i64,

    #[serde(rename = "w")]
    w: i64,

    #[serde(rename = "resize")]
    resize: Resize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoInfo {
    #[serde(rename = "aspect_ratio")]
    aspect_ratio: Vec<i64>,

    #[serde(rename = "duration_millis")]
    duration_millis: Option<i64>,

    #[serde(rename = "variants")]
    variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variant {
    #[serde(rename = "bitrate")]
    bitrate: Option<i64>,

    #[serde(rename = "content_type")]
    content_type: ContentType,

    #[serde(rename = "url")]
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMention {
    #[serde(rename = "id_str")]
    id_str: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "screen_name")]
    screen_name: String,

    #[serde(rename = "indices")]
    indices: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleExtendedEntities {
    #[serde(rename = "media")]
    media: Vec<PurpleMedia>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuotedStatusPermalink {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "expanded")]
    expanded: String,

    #[serde(rename = "display")]
    display: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RetweetedStatusResult {
    #[serde(rename = "result")]
    result: RetweetedStatusResultResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RetweetedStatusResultResult {
    #[serde(rename = "__typename")]
    typename: TweetDisplayType,

    #[serde(rename = "rest_id")]
    rest_id: String,

    #[serde(rename = "core")]
    core: FluffyCore,

    #[serde(rename = "unmention_data")]
    unmention_data: UnmentionData,

    #[serde(rename = "unified_card")]
    unified_card: Option<UnifiedCard>,

    #[serde(rename = "edit_control")]
    edit_control: EditControlInitialClass,

    #[serde(rename = "is_translatable")]
    is_translatable: bool,

    #[serde(rename = "views")]
    views: Views,

    #[serde(rename = "source")]
    source: String,

    #[serde(rename = "legacy")]
    legacy: TentacledLegacy,

    #[serde(rename = "note_tweet")]
    note_tweet: Option<PurpleNoteTweet>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyCore {
    #[serde(rename = "user_results")]
    user_results: FluffyUserResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyUserResults {
    #[serde(rename = "result")]
    result: TentacledResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TentacledResult {
    #[serde(rename = "__typename")]
    typename: Typename,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "rest_id")]
    rest_id: String,

    #[serde(rename = "affiliates_highlighted_label")]
    affiliates_highlighted_label: UnmentionData,

    #[serde(rename = "has_graduated_access")]
    has_graduated_access: bool,

    #[serde(rename = "is_blue_verified")]
    is_blue_verified: bool,

    #[serde(rename = "profile_image_shape")]
    profile_image_shape: ProfileImageShape,

    #[serde(rename = "legacy")]
    legacy: PurpleLegacy,

    #[serde(rename = "professional")]
    professional: Option<Professional>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Professional {
    #[serde(rename = "rest_id")]
    rest_id: String,

    #[serde(rename = "professional_type")]
    professional_type: String,

    #[serde(rename = "category")]
    category: Vec<Category>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "icon_name")]
    icon_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TentacledLegacy {
    #[serde(rename = "bookmark_count")]
    bookmark_count: i64,

    #[serde(rename = "bookmarked")]
    bookmarked: bool,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "conversation_id_str")]
    conversation_id_str: String,

    #[serde(rename = "display_text_range")]
    display_text_range: Vec<i64>,

    #[serde(rename = "entities")]
    entities: FluffyEntit,

    #[serde(rename = "extended_entities")]
    extended_entities: FluffyExtendedEntities,

    #[serde(rename = "favorite_count")]
    favorite_count: i64,

    #[serde(rename = "favorited")]
    favorited: bool,

    #[serde(rename = "full_text")]
    full_text: String,

    #[serde(rename = "is_quote_status")]
    is_quote_status: bool,

    #[serde(rename = "lang")]
    lang: Lang,

    #[serde(rename = "possibly_sensitive")]
    possibly_sensitive: bool,

    #[serde(rename = "possibly_sensitive_editable")]
    possibly_sensitive_editable: bool,

    #[serde(rename = "quote_count")]
    quote_count: i64,

    #[serde(rename = "reply_count")]
    reply_count: i64,

    #[serde(rename = "retweet_count")]
    retweet_count: i64,

    #[serde(rename = "retweeted")]
    retweeted: bool,

    #[serde(rename = "user_id_str")]
    user_id_str: String,

    #[serde(rename = "id_str")]
    id_str: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyEntit {
    #[serde(rename = "media")]
    media: Option<Vec<FluffyMedia>>,

    #[serde(rename = "user_mentions")]
    user_mentions: Vec<UserMention>,

    #[serde(rename = "urls")]
    urls: Vec<Url>,

    #[serde(rename = "hashtags")]
    hashtags: Vec<Hashtag>,

    #[serde(rename = "symbols")]
    symbols: Vec<Hashtag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyMedia {
    #[serde(rename = "display_url")]
    display_url: String,

    #[serde(rename = "expanded_url")]
    expanded_url: String,

    #[serde(rename = "id_str")]
    id_str: String,

    #[serde(rename = "indices")]
    indices: Vec<i64>,

    #[serde(rename = "media_key")]
    media_key: String,

    #[serde(rename = "media_url_https")]
    media_url_https: String,

    #[serde(rename = "type")]
    media_type: Type,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "ext_media_availability")]
    ext_media_availability: ExtMediaAvailability,

    #[serde(rename = "features")]
    features: Features,

    #[serde(rename = "sizes")]
    sizes: Sizes,

    #[serde(rename = "original_info")]
    original_info: OriginalInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyExtendedEntities {
    #[serde(rename = "media")]
    media: Vec<FluffyMedia>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleNoteTweet {
    #[serde(rename = "is_expandable")]
    is_expandable: bool,

    #[serde(rename = "note_tweet_results")]
    note_tweet_results: PurpleNoteTweetResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleNoteTweetResults {
    #[serde(rename = "result")]
    result: StickyResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StickyResult {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "entity_set")]
    entity_set: PurpleEntit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiedCard {
    #[serde(rename = "card_fetch_state")]
    card_fetch_state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Views {
    #[serde(rename = "count")]
    count: Option<String>,

    #[serde(rename = "state")]
    state: State,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyNoteTweet {
    #[serde(rename = "is_expandable")]
    is_expandable: bool,

    #[serde(rename = "note_tweet_results")]
    note_tweet_results: FluffyNoteTweetResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyNoteTweetResults {
    #[serde(rename = "result")]
    result: IndigoResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndigoResult {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "text")]
    text: String,

    #[serde(rename = "entity_set")]
    entity_set: FluffyEntit,

    #[serde(rename = "richtext")]
    richtext: Option<Richtext>,

    #[serde(rename = "media")]
    media: Option<ResultMedia>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultMedia {
    #[serde(rename = "inline_media")]
    inline_media: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Richtext {
    #[serde(rename = "richtext_tags")]
    richtext_tags: Vec<RichtextTag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RichtextTag {
    #[serde(rename = "from_index")]
    from_index: i64,

    #[serde(rename = "to_index")]
    to_index: i64,

    #[serde(rename = "richtext_types")]
    richtext_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousCounts {
    #[serde(rename = "bookmark_count")]
    bookmark_count: i64,

    #[serde(rename = "favorite_count")]
    favorite_count: i64,

    #[serde(rename = "quote_count")]
    quote_count: i64,

    #[serde(rename = "reply_count")]
    reply_count: i64,

    #[serde(rename = "retweet_count")]
    retweet_count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuickPromoteEligibility {
    #[serde(rename = "eligibility")]
    eligibility: Eligibility,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuotedStatusResult {
    #[serde(rename = "result")]
    result: QuotedStatusResultResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuotedStatusResultResult {
    #[serde(rename = "__typename")]
    typename: TweetDisplayType,

    #[serde(rename = "rest_id")]
    rest_id: String,

    #[serde(rename = "core")]
    core: TentacledCore,

    #[serde(rename = "unmention_data")]
    unmention_data: UnmentionData,

    #[serde(rename = "unified_card")]
    unified_card: UnifiedCard,

    #[serde(rename = "edit_control")]
    edit_control: EditControlInitialClass,

    #[serde(rename = "is_translatable")]
    is_translatable: bool,

    #[serde(rename = "views")]
    views: Views,

    #[serde(rename = "source")]
    source: String,

    #[serde(rename = "note_tweet")]
    note_tweet: FluffyNoteTweet,

    #[serde(rename = "legacy")]
    legacy: TentacledLegacy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TentacledCore {
    #[serde(rename = "user_results")]
    user_results: TentacledUserResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TentacledUserResults {
    #[serde(rename = "result")]
    result: IndecentResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndecentResult {
    #[serde(rename = "__typename")]
    typename: Typename,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "rest_id")]
    rest_id: String,

    #[serde(rename = "affiliates_highlighted_label")]
    affiliates_highlighted_label: UnmentionData,

    #[serde(rename = "has_graduated_access")]
    has_graduated_access: bool,

    #[serde(rename = "is_blue_verified")]
    is_blue_verified: bool,

    #[serde(rename = "profile_image_shape")]
    profile_image_shape: ProfileImageShape,

    #[serde(rename = "legacy")]
    legacy: PurpleLegacy,

    #[serde(rename = "professional")]
    professional: Professional,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurpleEntry {
    #[serde(rename = "entryId")]
    entry_id: String,

    #[serde(rename = "sortIndex")]
    sort_index: String,

    #[serde(rename = "content")]
    content: FluffyContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyContent {
    #[serde(rename = "entryType")]
    entry_type: EntryTypeEnum,

    #[serde(rename = "__typename")]
    typename: EntryTypeEnum,

    #[serde(rename = "itemContent")]
    item_content: FluffyItemContent,

    #[serde(rename = "clientEventInfo")]
    client_event_info: FluffyClientEventInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyClientEventInfo {
    #[serde(rename = "component")]
    component: String,

    #[serde(rename = "element")]
    element: Ent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyItemContent {
    #[serde(rename = "itemType")]
    item_type: ItemTypeEnum,

    #[serde(rename = "__typename")]
    typename: ItemTypeEnum,

    #[serde(rename = "tweet_results")]
    tweet_results: FluffyTweetResults,

    #[serde(rename = "tweetDisplayType")]
    tweet_display_type: TweetDisplayType,

    #[serde(rename = "socialContext")]
    social_context: SocialContext,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SocialContext {
    #[serde(rename = "type")]
    social_context_type: String,

    #[serde(rename = "contextType")]
    context_type: String,

    #[serde(rename = "text")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FluffyTweetResults {
    #[serde(rename = "result")]
    result: HilariousResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HilariousResult {
    #[serde(rename = "__typename")]
    typename: TweetDisplayType,

    #[serde(rename = "rest_id")]
    rest_id: String,

    #[serde(rename = "core")]
    core: PurpleCore,

    #[serde(rename = "unmention_data")]
    unmention_data: UnmentionData,

    #[serde(rename = "unified_card")]
    unified_card: UnifiedCard,

    #[serde(rename = "edit_control")]
    edit_control: EditControlInitialClass,

    #[serde(rename = "is_translatable")]
    is_translatable: bool,

    #[serde(rename = "views")]
    views: Views,

    #[serde(rename = "source")]
    source: String,

    #[serde(rename = "note_tweet")]
    note_tweet: FluffyNoteTweet,

    #[serde(rename = "legacy")]
    legacy: TentacledLegacy,

    #[serde(rename = "quick_promote_eligibility")]
    quick_promote_eligibility: QuickPromoteEligibility,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    #[serde(rename = "scribeConfig")]
    scribe_config: ScribeConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScribeConfig {
    #[serde(rename = "page")]
    page: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Ent {
    #[serde(rename = "tweet")]
    Tweet,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ControllerData {
    #[serde(rename = "DAACDAABDAABCgABAAAAAAAAAAAKAAkT+Npf5xWAAwAAAAA=")]
    DaacdaabdaabCgAbaaaaaaaaaaakaAkTNpf5XWaAwAaaaa,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InjectionType {
    #[serde(rename = "RankedOrganicTweet")]
    RankedOrganicTweet,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EntryTypeEnum {
    #[serde(rename = "TimelineTimelineCursor")]
    TimelineTimelineCursor,

    #[serde(rename = "TimelineTimelineItem")]
    TimelineTimelineItem,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ItemTypeEnum {
    #[serde(rename = "TimelineTweet")]
    TimelineTweet,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TweetDisplayType {
    #[serde(rename = "Tweet")]
    Tweet,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Id {
    #[serde(rename = "VXNlcjoxNTY5ODg2NzgzMDg5MDI5MTIy")]
    VxNlcjoxNty5ODg2NzgzMDg5Mdi5MtIy,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TranslatorType {
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProfileImageShape {
    #[serde(rename = "Circle")]
    Circle,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Typename {
    #[serde(rename = "User")]
    User,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "Available")]
    Available,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    #[serde(rename = "animated_gif")]
    AnimatedGif,

    #[serde(rename = "photo")]
    Photo,

    #[serde(rename = "video")]
    Video,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Resize {
    #[serde(rename = "crop")]
    Crop,

    #[serde(rename = "fit")]
    Fit,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ContentType {
    #[serde(rename = "application/x-mpegURL")]
    ApplicationXMpegUrl,

    #[serde(rename = "video/mp4")]
    VideoMp4,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Lang {
    #[serde(rename = "en")]
    En,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    #[serde(rename = "Enabled")]
    Enabled,

    #[serde(rename = "EnabledWithCount")]
    EnabledWithCount,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Eligibility {
    #[serde(rename = "IneligibleNotProfessional")]
    IneligibleNotProfessional,
}
