#![doc(html_root_url = "http://mmitteregger.github.io/rust-twitch-client")]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![warn(missing_docs)]
#![cfg_attr(test, deny(missing_docs))]
#![cfg_attr(test, deny(warnings))]

//! # Overview
//!
//! Rust Twitch Client is a library for the [Twitch REST API](https://github.com/justintv/Twitch-API) written in Rust!
//!
//! It uses [hyper](https://github.com/hyperium/hyper) as http client
//! and [serde](https://github.com/serde-rs/serde) for the serialization and deserialization of the REST entities.
//!
//! # Examples
//!
//! ```
//! use twitch_client::*;
//!
//! fn main() {
//!     let twitch_client = TwitchClient::new();
//!     let top_games = twitch_client.top_games(TopGamesParams::default()).unwrap();
//!
//!     println!("Total games: {}", top_games.total());
//!     println!("---");
//!     for game_info in top_games.top() {
//!         println!("Game: {}, Viewers: {}", game_info.game().name(), game_info.viewers());
//!     }
//!     println!("---");
//! }
//! ```

#[macro_use] extern crate hyper;
extern crate url;
extern crate serde;
extern crate serde_json;

pub mod model;
pub mod error;
mod http;
pub mod param;

pub use param::*;
use http::TwitchHttpClient;
use error::Result;

/// Readonly client for the [Twitch API](https://github.com/justintv/twitch-api).
///
/// Currently Twitch API version 3 is used.
///
/// By using the Twitch Client you agree to follow the
/// [Twitch API Terms of Service](https://www.twitch.tv/user/legal?page=api_terms_of_service)
/// and the [Twitch Terms of Service](https://www.twitch.tv/p/api_terms_of_service).
/// This library is in no way affiliated with, authorized, maintained, sponsored
/// or endorsed by Twitch or any of its affiliates or subsidiaries
///
/// # Examples
///
/// ```
/// use twitch_client::*;
///
/// let twitch_client = TwitchClient::new();
/// let top_games = twitch_client.top_games(TopGamesParams::default()).unwrap();
/// println!("Total games: {}", top_games.total());
/// ```
pub struct TwitchClient {
    http_client: TwitchHttpClient,
}

impl TwitchClient {

    /// Constructs a new instance without client id and with a default hyper client.
    ///
    /// To specify a client id (highly recommended to avoid being rate limited by Twitch)
    /// and/or specify a custom configured hyper client use the `TwitchClientBuilder` instead.
    pub fn new() -> TwitchClient {
        TwitchClientBuilder::new().build()
    }

}

/// Builder for the `TwitchClient`.
pub struct TwitchClientBuilder {
    client_id: Option<String>,
    hyper_client: Option<hyper::Client>,
}

impl TwitchClientBuilder {

    /// Constructs a new builder instance without client id and with a default hyper client.
    pub fn new() -> TwitchClientBuilder {
        TwitchClientBuilder {
            client_id: None,
            hyper_client: None,
        }
    }

    /// Sets the Twitch client id.
    /// See [https://github.com/justintv/twitch-api#rate-limits](https://github.com/justintv/twitch-api#rate-limits)
    /// for more information.
    pub fn client_id(mut self, client_id: &str) -> TwitchClientBuilder {
        self.client_id = Some(client_id.to_owned());
        self
    }

    /// Sets a custom configured hyper client.
    /// See [hyper::client::Client](http://hyper.rs/hyper/hyper/client/struct.Client.html)
    /// for more information.
    pub fn hyper_client(mut self, hyper_client: hyper::Client) -> TwitchClientBuilder {
        self.hyper_client = Some(hyper_client);
        self
    }

    /// Constructs the `TwitchClient` with the specified options.
    pub fn build(self) -> TwitchClient {
        let client = match self.hyper_client {
            Some(client) => client,
            None => hyper::Client::new(),
        };

        TwitchClient {
            http_client: TwitchHttpClient::new(self.client_id, client),
        }
    }
}




impl TwitchClient {

    /// Get games by number of viewers.
    ///
    /// Returns a list of games objects sorted by number of current viewers on Twitch, most popular first.
    pub fn top_games(&self, params: TopGamesParams) -> Result<model::game::TopGames> {
        let response = try!(self.http_client.get_content_with_params("/games/top", params));
        let top_games: model::game::TopGames = try!(serde_json::from_str(&response));
        Ok(top_games)
    }

    /// Get list of ingests.
    ///
    /// Returns a list of ingest objects.
    pub fn ingests(&self) -> Result<model::ingest::Ingests> {
        let response = try!(self.http_client.get_content("/ingests"));
        let ingests: model::ingest::Ingests = try!(serde_json::from_str(&response));
        Ok(ingests)
    }

    /// Get top level links object and authorization status.
    ///
    /// Basic information about the API and authentication status.
    /// If you are authenticated, the response includes the status of your token and links to other related resources.
    pub fn basic_info(&self) -> Result<model::root::BasicInfo> {
        let response = try!(self.http_client.get_content("/"));
        let basic_info: model::root::BasicInfo = try!(serde_json::from_str(&response));
        Ok(basic_info)
    }

    /// Get stream object.
    ///
    /// Returns a stream object if live.
    pub fn stream(&self, channel: &str) -> Result<model::stream::ChannelStream> {
        let url = format!("/streams/{}", channel);
        let response = try!(self.http_client.get_content(&url));
        let channel_stream: model::stream::ChannelStream = try!(serde_json::from_str(&response));
        Ok(channel_stream)
    }

    /// Get stream object.
    ///
    /// Returns a list of stream objects that are queried by a number of parameters
    /// sorted by number of viewers descending.
    pub fn streams(&self, params: StreamsParams) -> Result<model::stream::Streams> {
        let response = try!(self.http_client.get_content_with_params("/streams", params));
        let streams: model::stream::Streams = try!(serde_json::from_str(&response));
        Ok(streams)
    }

    /// Get a list of featured streams.
    ///
    /// Returns a list of featured (promoted) stream objects.
    pub fn featured_streams(&self, params: FeaturedStreamsParams) -> Result<model::stream::FeaturedStreams> {
        let response = try!(self.http_client.get_content_with_params("/streams/featured", params));
        let featured_streams: model::stream::FeaturedStreams = try!(serde_json::from_str(&response));
        Ok(featured_streams)
    }

    /// Get a summary of streams.
    ///
    /// Returns a summary of current streams.
    pub fn streams_summary(&self, params: StreamsSummaryParams) -> Result<model::stream::StreamsSummary> {
        let response = try!(self.http_client.get_content_with_params("/streams/summary", params));
        let streams_summary: model::stream::StreamsSummary = try!(serde_json::from_str(&response));
        Ok(streams_summary)
    }

    /// Get channel object.
    ///
    /// Returns a channel object.
    pub fn channel(&self, channel: &str) -> Result<model::channel::Channel> {
        let url = format!("/channels/{}", channel);
        let response = try!(self.http_client.get_content(&url));
        let channel: model::channel::Channel = try!(serde_json::from_str(&response));
        Ok(channel)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use serde_json;

    #[test]
    fn test_top_games_with_default_params() {
        let client = create_test_twitch_client();
        let top_games = client.top_games(TopGamesParams::default()).unwrap();
        assert_eq!(top_games.link_self(), "https://api.twitch.tv/kraken/games/top?limit=10&offset=0");
        assert_eq!(top_games.link_next(), "https://api.twitch.tv/kraken/games/top?limit=10&offset=10");
        assert!(top_games.total() > 0, "top_games.total() = {} > 0", top_games.total());
        assert_eq!(top_games.top().len(), 10);
    }

    #[test]
    fn test_top_games_with_custom_params() {
        let client = create_test_twitch_client();
        let params = TopGamesParams::new()
                .with_offset(0)
                .with_limit(2);
        let top_games = client.top_games(params).unwrap();
        assert_eq!(top_games.link_self(), "https://api.twitch.tv/kraken/games/top?limit=2&offset=0");
        assert_eq!(top_games.link_next(), "https://api.twitch.tv/kraken/games/top?limit=2&offset=2");
        assert!(top_games.total() > 0, "top_games.total() = {} > 0", top_games.total());
        assert_eq!(top_games.top().len(), 2);
    }

    #[test]
    fn test_ingests() {
        let client = create_test_twitch_client();
        let ingests = client.ingests().unwrap();
        assert_eq!(ingests.link_self(), "https://api.twitch.tv/kraken/ingests");
        assert!(ingests.ingests().len() > 0, "ingests.ingests().len() = {} > 0", ingests.ingests().len());
    }

    #[test]
    fn test_basic_info() {
        let client = create_test_twitch_client();
        let basic_info = client.basic_info().unwrap();
        assert_eq!(basic_info.link_user(), "https://api.twitch.tv/kraken/user");
        assert_eq!(basic_info.link_channel(), "https://api.twitch.tv/kraken/channel");
        assert_eq!(basic_info.link_search(), "https://api.twitch.tv/kraken/search");
        assert_eq!(basic_info.link_streams(), "https://api.twitch.tv/kraken/streams");
        assert_eq!(basic_info.link_ingests(), "https://api.twitch.tv/kraken/ingests");
        assert_eq!(basic_info.link_teams(), "https://api.twitch.tv/kraken/teams");
        assert!(basic_info.link_users().is_none(), "expecting no link for unauthenticated access");
        assert!(basic_info.link_channels().is_none(), "expecting no link for unauthenticated access");
        assert!(basic_info.link_chat().is_none(), "expecting no link for unauthenticated access");
        let token = basic_info.token();
        assert!(!token.valid(), "expecting invalid token for unauthenticated access");
        assert!(token.user_name().is_none(), "expecting no user name for unauthenticated access");
        assert!(token.authorization().is_none(), "expecting no auth info for unauthenticated access");
    }

    #[test]
    fn test_stream() {
        let client = create_test_twitch_client();
        let channel_stream = client.stream("test_channel").unwrap();
        assert_eq!(channel_stream.link_self(), "https://api.twitch.tv/kraken/streams/test_channel");
        assert_eq!(channel_stream.link_channel(), "https://api.twitch.tv/kraken/channels/test_channel");
        assert!(channel_stream.stream().is_none(), "expecting test channel stream to be offline");
    }

    #[test]
    fn test_streams_with_default_params() {
        let client = create_test_twitch_client();
        let streams = client.streams(StreamsParams::default()).unwrap();
        assert_eq!(streams.link_self(), "https://api.twitch.tv/kraken/streams?limit=25&offset=0");
        assert_eq!(streams.link_next(), "https://api.twitch.tv/kraken/streams?limit=25&offset=25");
        assert_eq!(streams.link_featured(), "https://api.twitch.tv/kraken/streams/featured");
        assert_eq!(streams.link_summary(), "https://api.twitch.tv/kraken/streams/summary");
        assert_eq!(streams.link_followed(), "https://api.twitch.tv/kraken/streams/followed");
        assert!(streams.total() > 0, "streams.total() = {} > 0", streams.total());
    }

    #[test]
    fn test_streams_with_custom_params() {
        let client = create_test_twitch_client();
        let params = StreamsParams::new()
                .with_offset(0)
                .with_limit(2)
                .with_stream_type(StreamType::Live);
        let streams = client.streams(params).unwrap();
        assert_eq!(streams.link_self(), "https://api.twitch.tv/kraken/streams?limit=2&offset=0&stream_type=live");
        assert_eq!(streams.link_next(), "https://api.twitch.tv/kraken/streams?limit=2&offset=2&stream_type=live");
        assert_eq!(streams.link_featured(), "https://api.twitch.tv/kraken/streams/featured");
        assert_eq!(streams.link_summary(), "https://api.twitch.tv/kraken/streams/summary");
        assert_eq!(streams.link_followed(), "https://api.twitch.tv/kraken/streams/followed");
        assert!(streams.total() > 0, "streams.total() = {} > 0", streams.total());
        assert_eq!(streams.streams().len(), 2);
    }

    #[test]
    fn test_featured_streams_with_default_params() {
        let client = create_test_twitch_client();
        let featured_streams = client.featured_streams(FeaturedStreamsParams::default()).unwrap();
        assert_eq!(featured_streams.link_self(), "https://api.twitch.tv/kraken/streams/featured?limit=25&offset=0");
        assert_eq!(featured_streams.link_next(), "https://api.twitch.tv/kraken/streams/featured?limit=25&offset=25");
        assert!(featured_streams.featured().len() > 0, "featured_streams.featured().len() = {} > 0", featured_streams.featured().len());
    }

    #[test]
    fn test_featured_streams_with_custom_params() {
        let client = create_test_twitch_client();
        let params = FeaturedStreamsParams::new()
                .with_offset(0)
                .with_limit(2);
        let featured_streams = client.featured_streams(params).unwrap();
        assert_eq!(featured_streams.link_self(), "https://api.twitch.tv/kraken/streams/featured?limit=2&offset=0");
        assert_eq!(featured_streams.link_next(), "https://api.twitch.tv/kraken/streams/featured?limit=2&offset=2");
        assert_eq!(featured_streams.featured().len(), 2);
    }

    #[test]
    fn test_streams_summary_with_default_params() {
        let client = create_test_twitch_client();
        let streams_summary = client.streams_summary(StreamsSummaryParams::default()).unwrap();
        assert_eq!(streams_summary.link_self(), "https://api.twitch.tv/kraken/streams/summary");
        assert!(streams_summary.channels() > 0, "streams_summary.channels() = {} > 0", streams_summary.channels());
        assert!(streams_summary.viewers() > 0, "streams_summary.viewers() = {} > 0", streams_summary.viewers());
    }

    #[test]
    fn test_streams_summary_with_custom_params() {
        let client = create_test_twitch_client();
        let params = StreamsSummaryParams::new()
                .with_game("StarCraft II: Heart of the Swarm");
        let streams_summary = client.streams_summary(params).unwrap();
        assert_eq!(streams_summary.link_self(), "https://api.twitch.tv/kraken/streams/summary?game=StarCraft+II%3A+Heart+of+the+Swarm");
    }

    #[test]
    fn test_channel() {
        let client = create_test_twitch_client();
        let channel = client.channel("test_channel").unwrap();
        assert_eq!(channel.link_self(), "https://api.twitch.tv/kraken/channels/test_channel");
        assert_eq!(channel.link_follows(), "https://api.twitch.tv/kraken/channels/test_channel/follows");
        assert_eq!(channel.link_commercial(), "https://api.twitch.tv/kraken/channels/test_channel/commercial");
        assert_eq!(channel.link_stream_key(), "https://api.twitch.tv/kraken/channels/test_channel/stream_key");
        assert_eq!(channel.link_chat(), "https://api.twitch.tv/kraken/chat/test_channel");
        assert_eq!(channel.link_features(), "https://api.twitch.tv/kraken/channels/test_channel/features");
        assert_eq!(channel.link_subscriptions(), "https://api.twitch.tv/kraken/channels/test_channel/subscriptions");
        assert_eq!(channel.link_editors(), "https://api.twitch.tv/kraken/channels/test_channel/editors");
        assert_eq!(channel.link_teams(), "https://api.twitch.tv/kraken/channels/test_channel/teams");
        assert_eq!(channel.link_videos(), "https://api.twitch.tv/kraken/channels/test_channel/videos");
        assert_eq!(channel.name(), "test_channel");
        assert_eq!(channel.url(), "https://secure.twitch.tv/test_channel");
        assert!(channel.views() > 0, "channel.views() = {} > 0", channel.views());
        assert!(channel.followers() > 0, "channel.followers() = {} > 0", channel.followers());
    }



    fn create_test_twitch_client() -> TwitchClient {
        let auth = read_auth();
        let mut twitch_client_builder = TwitchClientBuilder::new();

        match auth {
            Some(auth) => twitch_client_builder = twitch_client_builder.client_id(&auth.client_id),
            None => {},
        }

        let twitch_client = twitch_client_builder.build();
        twitch_client
    }

    fn read_auth() -> Option<Auth> {
        let mut auth_file = match File::open("twitch_auth.json") {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut auth_string = String::new();
        auth_file.read_to_string(&mut auth_string).unwrap();
        let auth: Auth = serde_json::from_str(&auth_string).unwrap();
        Some(auth)
    }

    #[derive(Deserialize, Debug)]
    struct Auth {
        name: String,
        client_id: String,
        redirect_uri: Option<String>,
        client_secret: Option<String>,
    }
}
