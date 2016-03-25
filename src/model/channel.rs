//! Twitch channels.
//!
//! Channels serve as the home location for a user's content.
//! Channels have a stream, can run commercials, store videos, display information and status,
//! and have a customized page including banners and backgrounds.

use std::collections::BTreeMap;

pub use model::TwitchLinks;
pub use model::UrlString;
pub use model::DateString;
pub use model::LocaleString;


/// Channel information.
///
/// # Example in JSON
///
/// ```json
/// {
///   "mature": false,
///   "status": "test status",
///   "broadcaster_language": "en",
///   "display_name": "test_channel",
///   "game": "Gaming Talk Shows",
///   "delay": null,
///   "language": "en",
///   "_id": 12345,
///   "name": "test_channel",
///   "created_at": "2007-05-22T10:39:54Z",
///   "updated_at": "2015-02-12T04:15:49Z",
///   "logo": "http://static-cdn.jtvnw.net/jtv_user_pictures/test_channel-profile_image-94a42b3a13c31c02-300x300.jpeg",
///   "banner": "http://static-cdn.jtvnw.net/jtv_user_pictures/test_channel-channel_header_image-08dd874c17f39837-640x125.png",
///   "video_banner": "http://static-cdn.jtvnw.net/jtv_user_pictures/test_channel-channel_offline_image-b314c834d210dc1a-640x360.png",
///   "background": null,
///   "profile_banner": "http://static-cdn.jtvnw.net/jtv_user_pictures/test_channel-profile_banner-6936c61353e4aeed-480.png",
///   "profile_banner_background_color": "null",
///   "partner": true,
///   "url": "http://www.twitch.tv/test_channel",
///   "views": 49144894,
///   "followers": 215780,
///   "_links": {
///     "self": "https://api.twitch.tv/kraken/channels/test_channel",
///     "follows": "https://api.twitch.tv/kraken/channels/test_channel/follows",
///     "commercial": "https://api.twitch.tv/kraken/channels/test_channel/commercial",
///     "stream_key": "https://api.twitch.tv/kraken/channels/test_channel/stream_key",
///     "chat": "https://api.twitch.tv/kraken/chat/test_channel",
///     "features": "https://api.twitch.tv/kraken/channels/test_channel/features",
///     "subscriptions": "https://api.twitch.tv/kraken/channels/test_channel/subscriptions",
///     "editors": "https://api.twitch.tv/kraken/channels/test_channel/editors",
///     "teams": "https://api.twitch.tv/kraken/channels/test_channel/teams",
///     "videos": "https://api.twitch.tv/kraken/channels/test_channel/videos"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    #[serde(rename="_links")]
    links: BTreeMap<String, String>,
    #[serde(rename="_id")]
    id: u64,
    name: String,
    display_name: String,
    game: Option<String>,
    status: Option<String>,
    mature: Option<bool>,
    delay: Option<u32>,
    language: LocaleString,
    broadcaster_language: Option<LocaleString>,
    created_at: DateString,
    updated_at: DateString,
    logo: Option<UrlString>,
    banner: Option<UrlString>,
    video_banner: Option<UrlString>,
    background: Option<UrlString>,
    profile_banner: Option<UrlString>,
    profile_banner_background_color: Option<UrlString>,
    partner: bool,
    url: UrlString,
    views: u32,
    followers: u32,
}


impl TwitchLinks for Channel {
    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

impl Channel {
    /// Link with key "self".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channels/test_channel"
    pub fn link_self(&self) -> &String {
        self.get_expected_link("self")
    }
    /// Link with key "follows".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channels/test_channel/follows"
    pub fn link_follows(&self) -> &String {
        self.get_expected_link("follows")
    }
    /// Link with key "commercial".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channels/test_channel/commercial"
    pub fn link_commercial(&self) -> &String {
        self.get_expected_link("commercial")
    }
    /// Link with key "stream_key".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channels/test_channel/stream_key"
    pub fn link_stream_key(&self) -> &String {
        self.get_expected_link("stream_key")
    }
    /// Link with key "chat".
    ///
    /// Example value: "https://api.twitch.tv/kraken/chat/test_channel"
    pub fn link_chat(&self) -> &String {
        self.get_expected_link("chat")
    }
    /// Link with key "features".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channels/test_channel/features"
    pub fn link_features(&self) -> &String {
        self.get_expected_link("features")
    }
    /// Link with key "subscriptions".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channels/test_channel/subscriptions"
    pub fn link_subscriptions(&self) -> &String {
        self.get_expected_link("subscriptions")
    }
    /// Link with key "editors".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channels/test_channel/editors"
    pub fn link_editors(&self) -> &String {
        self.get_expected_link("editors")
    }
    /// Link with key "teams".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channels/test_channel/teams"
    pub fn link_teams(&self) -> &String {
        self.get_expected_link("teams")
    }
    /// Link with key "videos".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channels/test_channel/videos"
    pub fn link_videos(&self) -> &String {
        self.get_expected_link("videos")
    }
    /// Example value: 12345
    pub fn id(&self) -> u64 {
        self.id
    }
    /// Example value: "test_channel"
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Example value: "test_channel"
    pub fn display_name(&self) -> &String {
        &self.display_name
    }
    /// Example value: "Gaming Talk Shows"
    pub fn game(&self) -> &Option<String> {
        &self.game
    }
    /// Example value: "test status"
    pub fn status(&self) -> &Option<String> {
        &self.status
    }
    /// Example value: false
    pub fn mature(&self) -> Option<bool> {
        self.mature
    }
    /// Example value: 0
    pub fn delay(&self) -> Option<u32> {
        self.delay
    }
    /// Example value: "en"
    pub fn language(&self) -> &LocaleString {
        &self.language
    }
    /// Example value: "en"
    pub fn broadcaster_language(&self) -> &Option<LocaleString> {
        &self.broadcaster_language
    }
    /// Example value: "2007-05-22T10:39:54Z"
    pub fn created_at(&self) -> &DateString {
        &self.created_at
    }
    /// Example value: "2015-02-12T04:15:49Z"
    pub fn updated_at(&self) -> &DateString {
        &self.updated_at
    }
    /// Example value: "http://static-cdn.jtvnw.net/jtv_user_pictures/test_channel-profile_image-94a42b3a13c31c02-300x300.jpeg"
    pub fn logo(&self) -> &Option<UrlString> {
        &self.logo
    }
    /// Example value: "http://static-cdn.jtvnw.net/jtv_user_pictures/test_channel-channel_header_image-08dd874c17f39837-640x125.png"
    pub fn banner(&self) -> &Option<UrlString> {
        &self.banner
    }
    /// Example value: "http://static-cdn.jtvnw.net/jtv_user_pictures/test_channel-channel_offline_image-b314c834d210dc1a-640x360.png"
    pub fn video_banner(&self) -> &Option<UrlString> {
        &self.video_banner
    }
    /// Example value: `None`
    pub fn background(&self) -> &Option<UrlString> {
        &self.background
    }
    /// Example value: "http://static-cdn.jtvnw.net/jtv_user_pictures/test_channel-profile_banner-6936c61353e4aeed-480.png"
    pub fn profile_banner(&self) -> &Option<UrlString> {
        &self.profile_banner
    }
    /// Example value: "null"
    pub fn profile_banner_background_color(&self) -> &Option<UrlString> {
        &self.profile_banner_background_color
    }
    /// Example value: true
    pub fn partner(&self) -> bool {
        self.partner
    }
    /// Example value: "http://www.twitch.tv/test_channel"
    pub fn url(&self) -> &UrlString {
        &self.url
    }
    /// Example value: 49144894
    pub fn views(&self) -> u32 {
        self.views
    }
    /// Example value: 215780
    pub fn followers(&self) -> u32 {
        self.followers
    }
}
