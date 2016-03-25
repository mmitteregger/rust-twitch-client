//! Twitch games.
//!
//! Games are categories (e.g. League of Legends, Diablo 3) used by streams and channels.
//! Games can be searched for by query.

use std::collections::BTreeMap;

pub use model::TwitchLinks;
pub use model::image::ImageLinks;


/// Games sorted by number of current viewers on Twitch, most popular first.
///
/// # Example in JSON
///
/// ```json
/// {
///   "_links": {
///     "self": "https://api.twitch.tv/kraken/games/top?limit=10&offset=0",
///     "next": "https://api.twitch.tv/kraken/games/top?limit=10&offset=10"
///   },
///   "_total": 322,
///   "top": [
///     {
///       // See `GameInfo` type
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopGames {
    #[serde(rename="_links")]
    links: BTreeMap<String, String>,
    #[serde(rename="_total")]
    total: u32,
    top: Vec<GameInfo>,
}

/// Current twitch stats about the game.
///
/// # Example in JSON
///
/// ```json
/// {
///   "game": {
///     // See `Game` type
///   },
///   "viewers": 23873,
///   "channels": 305
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInfo {
    viewers: u32,
    channels: u32,
    game: Game,
}

/// Information about the game itself.
///
/// # Example in JSON
///
/// ```json
/// {
///   "name": "Counter-Strike: Global Offensive",
///   "box": {
///     "large": "http://static-cdn.jtvnw.net/ttv-boxart/Counter-Strike:%20Global%20Offensive-272x380.jpg",
///     "medium": "http://static-cdn.jtvnw.net/ttv-boxart/Counter-Strike:%20Global%20Offensive-136x190.jpg",
///     "small": "http://static-cdn.jtvnw.net/ttv-boxart/Counter-Strike:%20Global%20Offensive-52x72.jpg",
///     "template": "http://static-cdn.jtvnw.net/ttv-boxart/Counter-Strike:%20Global%20Offensive-{width}x{height}.jpg"
///   },
///   "logo": {
///     "large": "http://static-cdn.jtvnw.net/ttv-logoart/Counter-Strike:%20Global%20Offensive-240x144.jpg",
///     "medium": "http://static-cdn.jtvnw.net/ttv-logoart/Counter-Strike:%20Global%20Offensive-120x72.jpg",
///     "small": "http://static-cdn.jtvnw.net/ttv-logoart/Counter-Strike:%20Global%20Offensive-60x36.jpg",
///     "template": "http://static-cdn.jtvnw.net/ttv-logoart/Counter-Strike:%20Global%20Offensive-{width}x{height}.jpg"
///   },
///   "_links": {},
///   "_id": 32399,
///   "giantbomb_id": 36113
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    #[serde(rename="_links")]
    links: BTreeMap<String, String>,
    #[serde(rename="_id")]
    id: u64,
    giantbomb_id: u64,
    name: String,
    #[serde(rename="box")]
    box_image_links: ImageLinks,
    #[serde(rename="logo")]
    logo_image_links: ImageLinks,
}


impl TwitchLinks for TopGames {
    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

impl TopGames {
    /// Link with key "self".
    ///
    /// Example value: "https://api.twitch.tv/kraken/games/top?limit=10&offset=0"
    pub fn link_self(&self) -> &String {
        self.get_expected_link("self")
    }
    /// Link with key "next".
    ///
    /// Example value: "https://api.twitch.tv/kraken/games/top?limit=10&offset=10"
    pub fn link_next(&self) -> &String {
        self.get_expected_link("next")
    }
    /// Example value: 322
    pub fn total(&self) -> u32 {
        self.total
    }
    /// Example value: See `GameInfo` type.
    pub fn top(&self) -> &Vec<GameInfo> {
        &self.top
    }
}

impl GameInfo {
    /// Example value: 23873
    pub fn viewers(&self) -> u32 {
        self.viewers
    }
    /// Example value: 305
    pub fn channels(&self) -> u32 {
        self.channels
    }
    /// Example value: See `Game` type.
    pub fn game(&self) -> &Game {
        &self.game
    }
}

impl TwitchLinks for Game {
    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

impl Game {
    /// Example value: 32399
    pub fn id(&self) -> u64 {
        self.id
    }
    /// Example value: 36113
    pub fn giantbomb_id(&self) -> u64 {
        self.giantbomb_id
    }
    /// Example value: "Counter-Strike: Global Offensive"
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Example value: See `ImageLinks` type.
    pub fn box_image_links(&self) -> &ImageLinks {
        &self.box_image_links
    }
    /// Example value: See `ImageLinks` type.
    pub fn logo_image_links(&self) -> &ImageLinks {
        &self.logo_image_links
    }
}
