//! Parameters for `TwitchClient` methods.

use std::string::ToString;
use url::percent_encoding::utf8_percent_encode;
use url::percent_encoding::FORM_URLENCODED_ENCODE_SET;

use http::IntoQueryString;


/// Parameters for the top games.
///
/// Use the `TopGamesParamsBuilder` to specify them
/// or use `TopGamesParams::default()` for the Twitch default.
///
/// # Examples
///
/// ```
/// use twitch_client::param::TopGamesParams;
///
/// let top_games_params = TopGamesParams::default();
/// ```
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct TopGamesParams {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl TopGamesParams {
    /// Object offset for pagination.
    ///
    /// Twitch defaults to 0 if `None`.
    pub fn offset(&self) -> Option<u32> {
        self.offset
    }
    /// Maximum number of objects in array.
    ///
    /// Twitch defaults to 10 if `None`. Maximum is 100.
    pub fn limit(&self) -> Option<u8> {
        self.limit
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

/// Builder for the `TopGamesParams`.
///
/// Use `TopGamesParamsBuilder::default()` to start specifying parameters.
///
/// # Examples
///
/// ```
/// use twitch_client::param::TopGamesParamsBuilder;
///
/// let top_games_params = TopGamesParamsBuilder::default()
///         .offset(40)
///         .limit(20)
///         .build();
/// ```
#[derive(Default)]
pub struct TopGamesParamsBuilder {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl TopGamesParamsBuilder {
    /// Object offset for pagination.
    ///
    /// Default is 0.
    pub fn offset(mut self, offset: u32) -> TopGamesParamsBuilder {
        self.offset = Some(offset);
        self
    }
    /// Maximum number of objects in array.
    ///
    /// Default is 10. Maximum is 100.
    pub fn limit(mut self, limit: u8) -> TopGamesParamsBuilder {
        self.limit = Some(limit);
        self
    }
    /// Constructs the `TopGamesParams` with the specified parameters.
    pub fn build(self) -> TopGamesParams {
        TopGamesParams {
            offset: self.offset,
            limit: self.limit,
        }
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
/// Use the `StreamsParamsBuilder` to specify them
/// or use `StreamsParams::default()` for the Twitch default.
///
/// # Examples
///
/// ```
/// use twitch_client::param::StreamsParams;
///
/// let streams_params = StreamsParams::default();
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
    /// Streams categorized under game.
    ///
    /// Twitch defaults to all games if `None`.
    pub fn game(&self) -> &Option<String> {
        &self.game
    }
    /// Streams from a list of channels.
    ///
    /// Twitch defaults to all channels if empty.
    pub fn channels(&self) -> &Vec<String> {
        &self.channels
    }
    /// Object offset for pagination.
    ///
    /// Twitch defaults to 0 if `None`.
    pub fn offset(&self) -> Option<u32> {
        self.offset
    }
    /// Maximum number of objects in array.
    ///
    /// Twitch defaults to 25 if `None`. Maximum is 100.
    pub fn limit(&self) -> Option<u8> {
        self.limit
    }
    /// Only shows streams from applications of client_id.
    ///
    /// Twitch defaults to all applications if `None`.
    pub fn client_id(&self) -> &Option<String> {
        &self.client_id
    }
    /// Only shows streams from a certain type.
    ///
    /// Twitch defaults to all if `None`.
    pub fn stream_type(&self) -> Option<StreamType> {
        self.stream_type
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

/// Builder for the `StreamsParams`.
///
/// Use `StreamsParamsBuilder::default()` to start specifying parameters.
///
/// # Examples
///
/// ```
/// use twitch_client::param::StreamsParamsBuilder;
/// use twitch_client::param::StreamType;
///
/// let streams_params = StreamsParamsBuilder::default()
///         .offset(40)
///         .limit(20)
///         .game("StarCraft II: Heart of the Swarm")
///         .stream_type(StreamType::Live)
///         .build();
/// ```
#[derive(Default)]
pub struct StreamsParamsBuilder {
    game: Option<String>,
    channels: Vec<String>,
    offset: Option<u32>,
    limit: Option<u8>,
    client_id: Option<String>,
    stream_type: Option<StreamType>,
}

impl StreamsParamsBuilder {
    /// Streams categorized under game.
    ///
    /// Default is all games.
    pub fn game(mut self, game: &str) -> StreamsParamsBuilder {
        self.game = Some(game.to_owned());
        self
    }
    /// Streams from a channel.
    /// Can be called multiple times to specify a list of channels.
    ///
    /// Default is all channels.
    pub fn channel(mut self, channel: &str) -> StreamsParamsBuilder {
        self.channels.push(channel.to_owned());
        self
    }
    /// Streams from a list of channels.
    /// Can be called with an empty Vec to clear the list and use the default again.
    ///
    /// Default is all channels.
    pub fn channels(mut self, channels: Vec<String>) -> StreamsParamsBuilder {
        self.channels = channels;
        self
    }
    /// Object offset for pagination.
    ///
    /// Default is 0.
    pub fn offset(mut self, offset: u32) -> StreamsParamsBuilder {
        self.offset = Some(offset);
        self
    }
    /// Maximum number of objects in array.
    ///
    /// Default is 25. Maximum is 100.
    pub fn limit(mut self, limit: u8) -> StreamsParamsBuilder {
        self.limit = Some(limit);
        self
    }
    /// Only shows streams from applications of `client_id`.
    ///
    /// Default is all applications.
    pub fn client_id(mut self, client_id: &str) -> StreamsParamsBuilder {
        self.client_id = Some(client_id.to_owned());
        self
    }
    /// Only shows streams from a certain type.
    ///
    /// Default is all types.
    pub fn stream_type(mut self, stream_type: StreamType) -> StreamsParamsBuilder {
        self.stream_type = Some(stream_type);
        self
    }
    /// Constructs the `StreamsParams` with the specified parameters.
    pub fn build(self) -> StreamsParams {
        StreamsParams {
            game: self.game,
            channels: self.channels,
            offset: self.offset,
            limit: self.limit,
            client_id: self.client_id,
            stream_type: self.stream_type,
        }
    }
}

/// Parameters for the featured streams.
///
/// Use the `FeaturedStreamsParamsBuilder` to specify them
/// or use `FeaturedStreamsParams::default()` for the Twitch default.
///
/// Note that the number of promoted streams varies from day to day,
/// and there is no guarantee on how many streams will be promoted at a given time.
///
/// # Examples
///
/// ```
/// use twitch_client::param::FeaturedStreamsParams;
///
/// let featured_streams_params = FeaturedStreamsParams::default();
/// ```
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct FeaturedStreamsParams {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl FeaturedStreamsParams {
    /// Object offset for pagination.
    ///
    /// Twitch defaults to 0 if `None`.
    pub fn offset(&self) -> Option<u32> {
        self.offset
    }
    /// Maximum number of objects in array.
    ///
    /// Twitch defaults to 25 if `None`. Maximum is 100.
    pub fn limit(&self) -> Option<u8> {
        self.limit
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

/// Builder for the `FeaturedStreamsParams`.
///
/// Use `FeaturedStreamsParamsBuilder::default()` to start specifying parameters.
///
/// # Examples
///
/// ```
/// use twitch_client::param::FeaturedStreamsParamsBuilder;
///
/// let featured_streams_params = FeaturedStreamsParamsBuilder::default()
///         .offset(5)
///         .limit(5)
///         .build();
/// ```
#[derive(Default)]
pub struct FeaturedStreamsParamsBuilder {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl FeaturedStreamsParamsBuilder {
    /// Object offset for pagination.
    ///
    /// Default is 0.
    pub fn offset(mut self, offset: u32) -> FeaturedStreamsParamsBuilder {
        self.offset = Some(offset);
        self
    }
    /// Maximum number of objects in array.
    ///
    /// Default is 25. Maximum is 100.
    pub fn limit(mut self, limit: u8) -> FeaturedStreamsParamsBuilder {
        self.limit = Some(limit);
        self
    }
    /// Constructs the `FeaturedStreamsParams` with the specified parameters.
    pub fn build(self) -> FeaturedStreamsParams {
        FeaturedStreamsParams {
            offset: self.offset,
            limit: self.limit,
        }
    }
}

/// Parameters for the streams summary.
///
/// Use the `StreamsSummaryParamsBuilder` to specify them
/// or use `StreamsSummaryParams::default()` for the Twitch default.
///
/// # Examples
///
/// ```
/// use twitch_client::param::StreamsSummaryParams;
///
/// let streams_summary_params = StreamsSummaryParams::default();
/// ```
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct StreamsSummaryParams {
    game: Option<String>,
}

impl StreamsSummaryParams {
    /// Streams categorized under game.
    ///
    /// Twitch defaults to all games if `None`.
    pub fn game(&self) -> &Option<String> {
        &self.game
    }
}

impl IntoQueryString for StreamsSummaryParams {
    fn into_query_string(self) -> String {
        params_into_query_string(vec![
            ("game", self.game.map(|game| game)),
        ])
    }
}

/// Builder for the `StreamsSummaryParams`.
///
/// Use `StreamsSummaryParamsBuilder::default()` to start specifying parameters.
///
/// # Examples
///
/// ```
/// use twitch_client::param::StreamsSummaryParamsBuilder;
///
/// let streams_summary_params = StreamsSummaryParamsBuilder::default()
///         .game("StarCraft II: Heart of the Swarm")
///         .build();
/// ```
#[derive(Default)]
pub struct StreamsSummaryParamsBuilder {
    game: Option<String>,
}

impl StreamsSummaryParamsBuilder {
    /// Streams categorized under game.
    ///
    /// Default is all games.
    pub fn game(mut self, game: &str) -> StreamsSummaryParamsBuilder {
        self.game = Some(game.to_owned());
        self
    }
    /// Constructs the `StreamsSummaryParams` with the specified parameters.
    pub fn build(self) -> StreamsSummaryParams {
        StreamsSummaryParams {
            game: self.game,
        }
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

fn encode(param_value: &str) -> String {
    utf8_percent_encode(param_value, FORM_URLENCODED_ENCODE_SET).replace("%20", "+")
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
        let params = TopGamesParamsBuilder::default()
                .limit(10)
                .build();
        assert_eq!(params.into_query_string(), "?limit=10");
    }

    #[test]
    fn test_multiple_params_should_concatenate_query_string_values_correctly() {
        let params = TopGamesParamsBuilder::default()
                .offset(5)
                .limit(10)
                .build();
        assert_eq!(params.into_query_string(), "?offset=5&limit=10");
    }

    #[test]
    fn test_string_params_should_be_escaped_correctly() {
        let params = StreamsParamsBuilder::default()
                .game("StarCraft II: Heart of the Swarm")
                .build();
        assert_eq!(params.into_query_string(), "?game=StarCraft+II%3A+Heart+of+the+Swarm");
    }

    #[test]
    fn test_empty_vec_query_string_should_be_empty_to_use_twitch_default() {
        // StreamsParams contains channels with type Vec<String>
        let params = StreamsParams::default();
        assert_eq!(params.into_query_string(), "");
    }

    #[test]
    fn test_strings_in_vec_should_be_concatenated_and_escaped_correctly() {
        let params = StreamsParamsBuilder::default()
                .channel("StarCraft I")
                .channel("StarCraft II")
                .build();
        assert_eq!(params.into_query_string(), "?channel=StarCraft+I%2CStarCraft+II");
    }

    #[test]
    fn test_stream_type_should_set_correctly() {
        let params = StreamsParamsBuilder::default()
                .stream_type(StreamType::All)
                .build();
        assert_eq!(params.into_query_string(), "?stream_type=all");
    }
}
