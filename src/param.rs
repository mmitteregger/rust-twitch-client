//! Parameters for `TwitchClient` methods.

use std::string::ToString;
use std::borrow::Cow;
use url::percent_encoding::utf8_percent_encode;
use url::percent_encoding::QUERY_ENCODE_SET;

use http::IntoQueryString;


/// Parameters for the top games.
///
/// # Examples
///
/// ```
/// use twitch_client::param::TopGamesParams;
///
/// let _default_params = TopGamesParams::default();
/// let _custom_params = TopGamesParams::new()
///         .with_offset(40)
///         .with_limit(20);
/// ```
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct TopGamesParams {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl TopGamesParams {
    /// Constructs a new instance.
    ///
    /// Synonym for TopGamesParams::default() but preferred if custom parameters are set.
    pub fn new() -> TopGamesParams {
        TopGamesParams::default()
    }
    /// Offset for pagination.
    ///
    /// Twitch defaults to 0 if not set.
    pub fn with_offset(mut self, offset: u32) -> TopGamesParams {
        self.offset = Some(offset);
        self
    }
    /// Maximum number of objects in array.
    ///
    /// Twitch defaults to 10 if not set. Maximum is 100.
    pub fn with_limit(mut self, limit: u8) -> TopGamesParams {
        self.limit = Some(limit);
        self
    }
}

impl IntoQueryString for TopGamesParams {
    fn into_query_string(self) -> String {
        params_into_query_string(vec![
            ("offset", self.offset.map(|offset| offset.to_string())),
            ("limit", self.limit.map(|limit| limit.to_string())),
        ])
    }
}

/// `StreamType` for `StreamsParams` to only show streams from a certain type.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum StreamType {
    /// Show all streams.
    All,
    /// Show only playlists.
    Playlist,
    /// Show only live streams.
    Live,
}

impl StreamType {
    fn to_query_string_value(&self) -> String {
        format!("{:?}", &self).to_lowercase()
    }
}

/// Parameters for the streams.
///
/// # Examples
///
/// ```
/// use twitch_client::param::StreamsParams;
/// use twitch_client::param::StreamType;
///
/// let _default_params = StreamsParams::default();
/// let _custom_params = StreamsParams::new()
///         .with_offset(40)
///         .with_limit(20)
///         .with_game("StarCraft II: Heart of the Swarm")
///         .with_stream_type(StreamType::Live);
/// ```
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct StreamsParams {
    game: Option<String>,
    channels: Vec<String>,
    offset: Option<u32>,
    limit: Option<u8>,
    client_id: Option<String>,
    stream_type: Option<StreamType>,
}

impl StreamsParams {
    /// Constructs a new instance.
    ///
    /// Synonym for StreamsParams::default() but preferred if custom parameters are set.
    pub fn new() -> StreamsParams {
        StreamsParams::default()
    }
    /// Streams categorized under game.
    ///
    /// Twitch defaults to all games if not set.
    pub fn with_game(mut self, game: &str) -> StreamsParams {
        self.game = Some(game.to_owned());
        self
    }
    /// Streams from a channel.
    /// Can be called multiple times to specify a list of channels.
    ///
    /// Twitch defaults to all channels if not set.
    pub fn with_channel(mut self, channel: &str) -> StreamsParams {
        self.channels.push(channel.to_owned());
        self
    }
    /// Streams from a list of channels.
    /// Can be called with an empty Vec to clear the list and use the default again.
    ///
    /// Twitch defaults to all channels if not set or empty.
    pub fn with_channels(mut self, channels: Vec<String>) -> StreamsParams {
        self.channels = channels;
        self
    }
    /// Offset for pagination.
    ///
    /// Twitch defaults to 0 if not set.
    pub fn with_offset(mut self, offset: u32) -> StreamsParams {
        self.offset = Some(offset);
        self
    }
    /// Maximum number of objects in array.
    ///
    /// Twitch defaults to 25 if not set. Maximum is 100.
    pub fn with_limit(mut self, limit: u8) -> StreamsParams {
        self.limit = Some(limit);
        self
    }
    /// Only shows streams from applications of `client_id`.
    ///
    /// Twitch defaults to all applications if not set.
    pub fn with_client_id(mut self, client_id: &str) -> StreamsParams {
        self.client_id = Some(client_id.to_owned());
        self
    }
    /// Only shows streams from a certain type.
    ///
    /// Twitch defaults to all if not set.
    pub fn with_stream_type(mut self, stream_type: StreamType) -> StreamsParams {
        self.stream_type = Some(stream_type);
        self
    }
}

impl IntoQueryString for StreamsParams {
    fn into_query_string(self) -> String {
        params_into_query_string(vec![
            ("game", self.game.map(|game| game)),
            (
                "channel",
                if self.channels.is_empty() {
                    None
                } else {
                    Some(self.channels.join(","))
                }
            ),
            ("offset", self.offset.map(|offset| offset.to_string())),
            ("limit", self.limit.map(|limit| limit.to_string())),
            ("client_id", self.client_id.map(|client_id| client_id)),
            ("stream_type", self.stream_type.map(|stream_type| stream_type.to_query_string_value())),
        ])
    }
}

/// Parameters for the featured streams.
///
/// Note that the number of promoted streams varies from day to day,
/// and there is no guarantee on how many streams will be promoted at a given time.
///
/// # Examples
///
/// ```
/// use twitch_client::param::FeaturedStreamsParams;
///
/// let _default_params = FeaturedStreamsParams::default();
/// let _custom_params = FeaturedStreamsParams::new()
///         .with_offset(5)
///         .with_limit(5);
/// ```
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct FeaturedStreamsParams {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl FeaturedStreamsParams {
    /// Constructs a new instance.
    ///
    /// Synonym for FeaturedStreamsParams::default() but preferred if custom parameters are set.
    pub fn new() -> FeaturedStreamsParams {
        FeaturedStreamsParams::default()
    }
    /// Offset for pagination.
    ///
    /// Twitch defaults to 0 if not set.
    pub fn with_offset(mut self, offset: u32) -> FeaturedStreamsParams {
        self.offset = Some(offset);
        self
    }
    /// Maximum number of objects in array.
    ///
    /// Twitch defaults to 25 if not set. Maximum is 100.
    pub fn with_limit(mut self, limit: u8) -> FeaturedStreamsParams {
        self.limit = Some(limit);
        self
    }
}

impl IntoQueryString for FeaturedStreamsParams {
    fn into_query_string(self) -> String {
        params_into_query_string(vec![
            ("offset", self.offset.map(|offset| offset.to_string())),
            ("limit", self.limit.map(|limit| limit.to_string())),
        ])
    }
}

/// Parameters for the streams summary.
///
/// # Examples
///
/// ```
/// use twitch_client::param::StreamsSummaryParams;
///
/// let _default_params = StreamsSummaryParams::default();
/// let _custom_params = StreamsSummaryParams::new()
///         .with_game("StarCraft II: Heart of the Swarm");
/// ```
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct StreamsSummaryParams {
    game: Option<String>,
}

impl StreamsSummaryParams {
    /// Constructs a new instance.
    ///
    /// Synonym for StreamsSummaryParams::default() but preferred if custom parameters are set.
    pub fn new() -> StreamsSummaryParams {
        StreamsSummaryParams::default()
    }
    /// Streams categorized under game.
    ///
    /// Twitch defaults to all games if not set.
    pub fn with_game(mut self, game: &str) -> StreamsSummaryParams {
        self.game = Some(game.to_owned());
        self
    }
}

impl IntoQueryString for StreamsSummaryParams {
    fn into_query_string(self) -> String {
        params_into_query_string(vec![
            ("game", self.game.map(|game| game)),
        ])
    }
}



fn params_into_query_string(params: Vec<(&str, Option<String>)>) -> String {
    let mut query_string = String::new();

    for (param_name, param_value) in params {
        match param_value {
            Some(ref value) => {
                if query_string.is_empty() {
                    query_string.push('?');
                } else {
                    query_string.push('&');
                }

                query_string.push_str(param_name);
                query_string.push('=');
                query_string.push_str(&encode(value));
            },
            None => {},
        }
    }

    query_string
}

fn encode(param_value: &str) -> Cow<str> {
    utf8_percent_encode(param_value, QUERY_ENCODE_SET).collect()
}



#[cfg(test)]
mod tests {
    use super::*;
    use http::IntoQueryString;

    #[test]
    fn test_default_params_query_string_should_be_empty_to_use_twitch_default() {
        let params = TopGamesParams::default();
        assert_eq!(params.into_query_string(), "");
    }

    #[test]
    fn test_one_param_should_only_set_one_query_value() {
        let params = TopGamesParams::new()
                .with_limit(10);
        assert_eq!(params.into_query_string(), "?limit=10");
    }

    #[test]
    fn test_multiple_params_should_concatenate_query_string_values_correctly() {
        let params = TopGamesParams::new()
                .with_offset(5)
                .with_limit(10);
        assert_eq!(params.into_query_string(), "?offset=5&limit=10");
    }

    #[test]
    fn test_string_params_should_be_escaped_correctly() {
        let params = StreamsParams::new()
                .with_game("StarCraft II: Heart of the Swarm");
        assert_eq!(params.into_query_string(), "?game=StarCraft%20II:%20Heart%20of%20the%20Swarm");
    }

    #[test]
    fn test_empty_vec_query_string_should_be_empty_to_use_twitch_default() {
        // StreamsParams contains channels with type Vec<String>
        let params = StreamsParams::default();
        assert_eq!(params.into_query_string(), "");
    }

    #[test]
    fn test_strings_in_vec_should_be_concatenated_and_escaped_correctly() {
        let params = StreamsParams::new()
                .with_channel("StarCraft I")
                .with_channel("StarCraft II");
        assert_eq!(params.into_query_string(), "?channel=StarCraft%20I,StarCraft%20II");
    }

    #[test]
    fn test_stream_type_should_set_correctly() {
        let params = StreamsParams::new()
                .with_stream_type(StreamType::All);
        assert_eq!(params.into_query_string(), "?stream_type=all");
    }
}
