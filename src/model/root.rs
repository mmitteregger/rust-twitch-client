//! Top level Twitch models and authorization status.

use std::collections::BTreeMap;

pub use model::TwitchLinks;
pub use model::DateString;

/// Basic information about the API and authentication status.
///
/// # Example in JSON
///
/// ```json
/// {
///   "token": {
///     // See `Token` type
///   },
///   "_links": {
///     "channel": "https://api.twitch.tv/kraken/channel",
///     "users": "https://api.twitch.tv/kraken/users/test_user1",
///     "user": "https://api.twitch.tv/kraken/user",
///     "channels": "https://api.twitch.tv/kraken/channels/test_user1",
///     "chat": "https://api.twitch.tv/kraken/chat/test_user1",
///     "streams": "https://api.twitch.tv/kraken/streams",
///     "ingests":"https://api.twitch.tv/kraken/ingests",
///     "teams": "https://api.twitch.tv/kraken/teams",
///     "search": "https://api.twitch.tv/kraken/search"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicInfo {
    #[serde(rename="_links")]
    links: BTreeMap<String, String>,
    token: Token,
}

/// Authentication token.
///
/// # Example in JSON
///
/// ```json
/// {
///   "authorization": {
///     // See `Authorization` type
///   },
///   "user_name": "test_user1",
///   "valid": true
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    valid: bool,
    user_name: Option<String>,
    authorization: Option<Authorization>,
}

/// Authorization information.
///
/// # Example in JSON
///
/// ```json
/// {
///   "scopes": ["user_read", "channel_read", "channel_commercial", "user_read"],
///   "created_at": "2012-05-08T21:55:12Z",
///   "updated_at": "2012-05-17T21:32:13Z"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Authorization {
    scopes: Vec<String>,
    created_at: DateString,
    updated_at: DateString,
}


impl TwitchLinks for BasicInfo {
    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

impl BasicInfo {
    /// Link with key "user".
    ///
    /// Example value: "https://api.twitch.tv/kraken/user"
    pub fn link_user(&self) -> &String {
        self.get_expected_link("user")
    }
    /// Link with key "channel".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channel"
    pub fn link_channel(&self) -> &String {
        self.get_expected_link("channel")
    }
    /// Link with key "search".
    ///
    /// Example value: "https://api.twitch.tv/kraken/search"
    pub fn link_search(&self) -> &String {
        self.get_expected_link("search")
    }
    /// Link with key "streams".
    ///
    /// Example value: "https://api.twitch.tv/kraken/streams"
    pub fn link_streams(&self) -> &String {
        self.get_expected_link("streams")
    }
    /// Link with key "ingests".
    ///
    /// Example value: "https://api.twitch.tv/kraken/ingests"
    pub fn link_ingests(&self) -> &String {
        self.get_expected_link("ingests")
    }
    /// Link with key "teams".
    ///
    /// Example value: "https://api.twitch.tv/kraken/teams"
    pub fn link_teams(&self) -> &String {
        self.get_expected_link("teams")
    }
    /// Link with key "users".
    ///
    /// Example value: "https://api.twitch.tv/kraken/users/test_user1"
    pub fn link_users(&self) -> Option<&String> {
        self.links.get("users")
    }
    /// Link with key "channels".
    ///
    /// Example value: "https://api.twitch.tv/kraken/channels/test_user1"
    pub fn link_channels(&self) -> Option<&String> {
        self.links.get("channels")
    }
    /// Link with key "chat".
    ///
    /// Example value: "https://api.twitch.tv/kraken/chat/test_user1"
    pub fn link_chat(&self) -> Option<&String> {
        self.links.get("chat")
    }
    /// Example value: See `Token` type.
    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl Token {
    /// Example value: true
    pub fn valid(&self) -> bool {
        self.valid
    }
    /// Example value: "test_user1"
    pub fn user_name(&self) -> &Option<String> {
        &self.user_name
    }
    /// Example value: See `Authorization` type.
    pub fn authorization(&self) -> &Option<Authorization> {
        &self.authorization
    }
}

impl Authorization {
    /// Example values: ["user_read", "channel_read", "channel_commercial", "user_read"]
    pub fn scopes(&self) -> &Vec<String> {
        &self.scopes
    }
    /// Example value: "2012-05-08T21:55:12Z"
    pub fn created_at(&self) -> &DateString {
        &self.created_at
    }
    /// Example value: "2012-05-17T21:32:13Z"
    pub fn updated_at(&self) -> &DateString {
        &self.updated_at
    }
}
