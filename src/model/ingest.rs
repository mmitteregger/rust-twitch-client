//! Twitch ingests.
//!
//! These are RTMP ingest points.
//! By directing an RTMP stream with your `stream_key` injected into the `url_template`,
//! you will broadcast your content live on Twitch.

use std::collections::BTreeMap;

pub use model::TwitchLinks;

/// List of ingests.
///
/// # Example in JSON
///
/// ```json
/// {
///   "_links": {
///     "self": "https://api.twitch.tv/kraken/ingests"
///   },
///   "ingests": [
///     {
///       // See `Ingest` type
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingests {
    #[serde(rename="_links")]
    links: BTreeMap<String, String>,
    ingests: Vec<Ingest>,
}

/// Ingest point.
///
/// # Example in JSON
///
/// ```json
/// {
///   "name": "EU: Amsterdam, NL",
///   "default": false,
///   "_id": 24,
///   "url_template": "rtmp://live-ams.twitch.tv/app/{stream_key}",
///   "availability": 1.0
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingest {
    name: String,
    availability: f64,
    #[serde(rename="_id")]
    id: u64,
    default: bool,
    url_template: String,
}


impl TwitchLinks for Ingests {
    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

impl Ingests {
    /// Link with key "self".
    ///
    /// Example value: "https://api.twitch.tv/kraken/ingests"
    pub fn link_self(&self) -> &String {
        self.get_expected_link("self")
    }
    /// Example value: See `Ingest` type.
    pub fn ingests(&self) -> &Vec<Ingest> {
        &self.ingests
    }
}

impl Ingest {
    /// Example value: "EU: Amsterdam, NL"
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Example value: 1.0
    pub fn availability(&self) -> f64 {
        self.availability
    }
    /// Example value: 24
    pub fn id(&self) -> u64 {
        self.id
    }
    /// Example value: false
    pub fn default(&self) -> bool {
        self.default
    }
    /// Example value: "rtmp://live-ams.twitch.tv/app/{stream_key}"
    pub fn url_template(&self) -> &String {
        &self.url_template
    }
}
