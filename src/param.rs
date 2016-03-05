use std::string::ToString;
use url::percent_encoding::utf8_percent_encode;
use url::percent_encoding::FORM_URLENCODED_ENCODE_SET;

use http::IntoQueryString;

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct TopGamesParams {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl TopGamesParams {
    pub fn offset(&self) -> Option<u32> {
        self.offset
    }
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

#[derive(Default)]
pub struct TopGamesParamsBuilder {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl TopGamesParamsBuilder {
    pub fn offset(mut self, offset: u32) -> TopGamesParamsBuilder {
        self.offset = Some(offset);
        self
    }
    pub fn limit(mut self, limit: u8) -> TopGamesParamsBuilder {
        self.limit = Some(limit);
        self
    }
    pub fn build(self) -> TopGamesParams {
        TopGamesParams {
            offset: self.offset,
            limit: self.limit,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum StreamType {
    All,
    Playlist,
    Live,
}

impl StreamType {
    fn to_query_string_value(&self) -> String {
        format!("{:?}", &self).to_lowercase()
    }
}

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
    pub fn game(&self) -> &Option<String> {
        &self.game
    }
    pub fn channels(&self) -> &Vec<String> {
        &self.channels
    }
    pub fn offset(&self) -> Option<u32> {
        self.offset
    }
    pub fn limit(&self) -> Option<u8> {
        self.limit
    }
    pub fn client_id(&self) -> &Option<String> {
        &self.client_id
    }
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
    pub fn game(mut self, game: &str) -> StreamsParamsBuilder {
        self.game = Some(game.to_owned());
        self
    }
    pub fn channel(mut self, channel: &str) -> StreamsParamsBuilder {
        self.channels.push(channel.to_owned());
        self
    }
    pub fn channels(mut self, channels: Vec<String>) -> StreamsParamsBuilder {
        self.channels = channels;
        self
    }
    pub fn offset(mut self, offset: u32) -> StreamsParamsBuilder {
        self.offset = Some(offset);
        self
    }
    pub fn limit(mut self, limit: u8) -> StreamsParamsBuilder {
        self.limit = Some(limit);
        self
    }
    pub fn client_id(mut self, client_id: &str) -> StreamsParamsBuilder {
        self.client_id = Some(client_id.to_owned());
        self
    }
    pub fn stream_type(mut self, stream_type: StreamType) -> StreamsParamsBuilder {
        self.stream_type = Some(stream_type);
        self
    }
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

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct FeaturedStreamsParams {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl FeaturedStreamsParams {
    pub fn offset(&self) -> Option<u32> {
        self.offset
    }
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

#[derive(Default)]
pub struct FeaturedStreamsParamsBuilder {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl FeaturedStreamsParamsBuilder {
    pub fn offset(mut self, offset: u32) -> FeaturedStreamsParamsBuilder {
        self.offset = Some(offset);
        self
    }
    pub fn limit(mut self, limit: u8) -> FeaturedStreamsParamsBuilder {
        self.limit = Some(limit);
        self
    }
    pub fn build(self) -> FeaturedStreamsParams {
        FeaturedStreamsParams {
            offset: self.offset,
            limit: self.limit,
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
