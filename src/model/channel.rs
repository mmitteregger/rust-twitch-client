use std::collections::BTreeMap;

pub use model::TwitchLinks;
pub use model::UrlString;
pub use model::DateString;
pub use model::LocaleString;


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
    pub fn link_self(&self) -> &String {
        self.get_expected_link("self")
    }
    pub fn link_follows(&self) -> &String {
        self.get_expected_link("follows")
    }
    pub fn link_commercial(&self) -> &String {
        self.get_expected_link("commercial")
    }
    pub fn link_stream_key(&self) -> &String {
        self.get_expected_link("stream_key")
    }
    pub fn link_chat(&self) -> &String {
        self.get_expected_link("chat")
    }
    pub fn link_features(&self) -> &String {
        self.get_expected_link("features")
    }
    pub fn link_subscriptions(&self) -> &String {
        self.get_expected_link("subscriptions")
    }
    pub fn link_editors(&self) -> &String {
        self.get_expected_link("editors")
    }
    pub fn link_teams(&self) -> &String {
        self.get_expected_link("teams")
    }
    pub fn link_videos(&self) -> &String {
        self.get_expected_link("videos")
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn display_name(&self) -> &String {
        &self.display_name
    }
    pub fn game(&self) -> &Option<String> {
        &self.game
    }
    pub fn status(&self) -> &Option<String> {
        &self.status
    }
    pub fn mature(&self) -> Option<bool> {
        self.mature
    }
    pub fn delay(&self) -> Option<u32> {
        self.delay
    }
    pub fn language(&self) -> &LocaleString {
        &self.language
    }
    pub fn broadcaster_language(&self) -> &Option<LocaleString> {
        &self.broadcaster_language
    }
    pub fn created_at(&self) -> &DateString {
        &self.created_at
    }
    pub fn updated_at(&self) -> &DateString {
        &self.updated_at
    }
    pub fn logo(&self) -> &Option<UrlString> {
        &self.logo
    }
    pub fn banner(&self) -> &Option<UrlString> {
        &self.banner
    }
    pub fn video_banner(&self) -> &Option<UrlString> {
        &self.video_banner
    }
    pub fn background(&self) -> &Option<UrlString> {
        &self.background
    }
    pub fn profile_banner(&self) -> &Option<UrlString> {
        &self.profile_banner
    }
    pub fn profile_banner_background_color(&self) -> &Option<UrlString> {
        &self.profile_banner_background_color
    }
    pub fn partner(&self) -> bool {
        self.partner
    }
    pub fn url(&self) -> &UrlString {
        &self.url
    }
    pub fn views(&self) -> u32 {
        self.views
    }
    pub fn followers(&self) -> u32 {
        self.followers
    }
}
