//! Twitch streams.
//!
//! Streams are video broadcasts that are currently live.
//! They have a broadcaster and are part of a channel.

pub use model::DateString;
pub use model::UrlString;
pub use model::image::ImageLinks;
pub use model::channel::Channel;


/// Streams that are queried by a number of parameters sorted by number of viewers descending.
///
/// # Example in JSON
///
/// ```json
/// {
///   "_total": 12345,
///   "streams": [
///     {
///       // See `Stream` type
///     }
///   ],
///   "_links": {
///     "summary": "https://api.twitch.tv/kraken/streams/summary",
///     "followed": "https://api.twitch.tv/kraken/streams/followed",
///     "next": "https://api.twitch.tv/kraken/streams?channel=test_channel%2Ctest_channel2&game=StarCraft+II%3A+Heart+of+the+Swarm&limit=100&offset=100",
///     "featured": "https://api.twitch.tv/kraken/streams/featured",
///     "self": "https://api.twitch.tv/kraken/streams?channel=test_channel%2Ctest_channel2&game=StarCraft+II%3A+Heart+of+the+Swarm&limit=100&offset=0"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Streams {
    #[serde(rename="_total")]
    total: u32,
    streams: Vec<Stream>,
}

/// Featured (promoted) streams.
///
/// # Example in JSON
///
/// ```json
/// {
///   "_links": {
///      "self": "https://api.twitch.tv/kraken/streams/featured?limit=25&offset=0",
///      "next": "https://api.twitch.tv/kraken/streams/featured?limit=25&offset=25"
///   },
///   "featured": [
///     {
///       // See `FeaturedStream` type
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeaturedStreams {
    featured: Vec<FeaturedStream>,
}

/// Twitch stream information of a specific channel.
///
/// # Example in JSON if offline
///
/// ```json
/// {
///   "stream": null,
///   "_links": {
///     "self": "https://api.twitch.tv/kraken/streams/test_channel",
///     "channel": "https://api.twitch.tv/kraken/channels/test_channel"
///   }
/// }
/// ```
///
/// # Example in JSON if online
///
/// ```json
/// {
///   "_links": {
///     "channel": "https://api.twitch.tv/kraken/channels/test_channel",
///     "self": "https://api.twitch.tv/kraken/streams/test_channel"
///   },
///   "stream": {
///     // See `Stream` type
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelStream {
    stream: Option<Stream>,
}

/// Summary of current streams.
///
/// # Example in JSON
///
/// ```json
/// {
///   "viewers": 194774,
///   "channels": 4144,
///   "_links": {
///     "self": "https://api.twitch.tv/kraken/streams/summary"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamsSummary {
    viewers: u32,
    channels: u32,
}

/// Featured (promoted) stream.
///
/// # Example in JSON
///
/// ```json
/// {
///   "image": "http://s.jtvnw.net/jtv_user_pictures/hosted_images/TwitchPartnerSpotlight.png",
///   "text": "<p>some html to describe this featured stream</p>",
///   "title": "Twitch Partner Spotlight",
///   "sponsored": false,
///   "scheduled": true,
///   "stream": {
///     // See `Stream` type
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeaturedStream {
    text: String,
    image: UrlString,
    title: String,
    sponsored: bool,
    priority: u8,
    scheduled: bool,
    stream: Stream,
}

/// Basic Stream type.
///
/// # Example in JSON
///
/// ```json
/// {
///   "game": "StarCraft II: Heart of the Swarm",
///   "viewers": 2123,
///   "average_fps": 29.9880749574,
///   "delay": 0,
///   "video_height": 720,
///   "created_at": "2015-02-12T04:42:31Z",
///   "_id": 4989654544,
///   "channel": {
///     // See `Channel` type
///   },
///   "preview": {
///     "small": "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-80x45.jpg",
///     "medium": "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-320x180.jpg",
///     "large": "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-640x360.jpg",
///     "template": "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-{width}x{height}.jpg"
///   },
///   "_links": {
///     "self": "https://api.twitch.tv/kraken/streams/test_channel"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stream {
    #[serde(rename="_id")]
    id: u64,
    game: Option<String>,
    viewers: u32,
    average_fps: f64,
    delay: Option<u32>,
    video_height: u16,
    is_playlist: bool,
    created_at: DateString,
    channel: Channel,
    preview: ImageLinks,
}


impl Streams {
    /// Example value: 12345
    pub fn total(&self) -> u32 {
        self.total
    }
    /// Example value: See `Stream` type.
    pub fn streams(&self) -> &Vec<Stream> {
        &self.streams
    }
}

impl FeaturedStreams {
    /// Example value: See `FeaturedStream` type.
    pub fn featured(&self) -> &Vec<FeaturedStream> {
        &self.featured
    }
}

impl ChannelStream {
    /// Example value: See `Stream` type.
    pub fn stream(&self) -> &Option<Stream> {
        &self.stream
    }
}

impl StreamsSummary {
    /// Example value: 194774
    pub fn viewers(&self) -> u32 {
        self.viewers
    }
    /// Example value: 4144
    pub fn channels(&self) -> u32 {
        self.channels
    }
}

impl FeaturedStream {
    /// Example value: "<p>some html to describe this featured stream</p>"
    pub fn text(&self) -> &String {
        &self.text
    }
    /// Example value: "http://s.jtvnw.net/jtv_user_pictures/hosted_images/TwitchPartnerSpotlight.png"
    pub fn image(&self) -> &UrlString {
        &self.image
    }
    /// Example value: "Twitch Partner Spotlight"
    pub fn title(&self) -> &String {
        &self.title
    }
    /// Example value: false
    pub fn sponsored(&self) -> bool {
        self.sponsored
    }
    /// Example value: 3
    pub fn priority(&self) -> u8 {
        self.priority
    }
    /// Example value: true
    pub fn scheduled(&self) -> bool {
        self.scheduled
    }
    /// Example value: See `Stream` type.
    pub fn stream(&self) -> &Stream {
        &self.stream
    }
}

impl Stream {
    /// Example value: 4989654544
    pub fn id(&self) -> u64 {
        self.id
    }
    /// Example value: "StarCraft II: Heart of the Swarm"
    pub fn game(&self) -> &Option<String> {
        &self.game
    }
    /// Example value: 2123
    pub fn viewers(&self) -> u32 {
        self.viewers
    }
    /// Example value: 29.9880749574
    pub fn average_fps(&self) -> f64 {
        self.average_fps
    }
    /// Example value: 0
    pub fn delay(&self) -> Option<u32> {
        self.delay
    }
    /// Example value: 720
    pub fn video_height(&self) -> u16 {
        self.video_height
    }
    /// Example value: false
    pub fn is_playlist(&self) -> bool {
        self.is_playlist
    }
    /// Example value: "2015-02-12T04:42:31Z"
    pub fn created_at(&self) -> &DateString {
        &self.created_at
    }
    /// Example value: See `Channel` type.
    pub fn channel(&self) -> &Channel {
        &self.channel
    }
    /// Example value: See `ImageLinks` type.
    pub fn preview(&self) -> &ImageLinks {
        &self.preview
    }
}
