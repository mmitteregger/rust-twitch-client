#![doc(html_root_url = "http://mmitteregger.github.io/rust-twitch-client")]
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
//!     let twitch_client = TwitchClient::new().unwrap().with_client_id("<INSERT_YOU_CLIENT_ID_HERE>");
//!
//!     match twitch_client.top_games(TopGamesParams::default()) {
//!         Ok(top_games) => {
//!             println!("Total games: {}", top_games.total());
//!             println!("---");
//!             for game_info in top_games.top() {
//!                 println!("Game: {}, Viewers: {}", game_info.game().name(), game_info.viewers());
//!             }
//!             println!("---");
//!         },
//!         Err(err) => println!("Failed to retrieve top games: {}", err),
//!     }
//! }
//! ```

#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
extern crate native_tls;
extern crate url;
extern crate serde;
#[macro_use] extern crate serde_derive;
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
/// let twitch_client = TwitchClient::new().unwrap().with_client_id("<INSERT_YOU_CLIENT_ID_HERE>");
///
/// match twitch_client.top_games(TopGamesParams::default()) {
///     Ok(top_games) => println!("Total games: {}", top_games.total()),
///     Err(err) => println!("Failed to retrieve top games: {}", err),
/// }
/// ```
pub struct TwitchClient {
    http_client: TwitchHttpClient,
}

impl TwitchClient {

    /// Constructs a new instance without client id and with a default hyper client.
    ///
    /// It is highly recommended to specify a client id to avoid being rate limited by Twitch
    /// with the `with_client_id` method.
    pub fn new() -> Result<TwitchClient> {
        let http_client = try!(TwitchHttpClient::new());

        let twitch_client = TwitchClient {
            http_client: http_client,
        };
        Ok(twitch_client)
    }

    /// Sets the Twitch client id.
    ///
    /// See [https://github.com/justintv/twitch-api#rate-limits](https://github.com/justintv/twitch-api#rate-limits)
    /// for more information.
    pub fn with_client_id(mut self, client_id: &str) -> TwitchClient {
        self.http_client.set_client_id(client_id);
        self
    }

    /// Sets a custom configured hyper client.
    ///
    /// See [hyper::client::Client](http://hyper.rs/hyper/hyper/client/struct.Client.html)
    /// for more information.
    pub fn with_hyper_client(mut self, hyper_client: hyper::Client) -> TwitchClient {
        self.http_client.set_hyper_client(hyper_client);
        self
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
        assert!(top_games.total() > 0, "top_games.total() = {} > 0", top_games.total());
        assert_eq!(top_games.top().len(), 2);
    }

    #[test]
    fn test_ingests() {
        let client = create_test_twitch_client();
        let ingests = client.ingests().unwrap();
        assert!(ingests.ingests().len() > 0, "ingests.ingests().len() = {} > 0", ingests.ingests().len());
    }

    #[test]
    fn test_basic_info() {
        let client = create_test_twitch_client();
        let basic_info = client.basic_info().unwrap();
        let token = basic_info.token();
        assert!(!token.valid(), "expecting invalid token for unauthenticated access");
        assert!(token.user_name().is_none(), "expecting no user name for unauthenticated access");
        assert!(token.authorization().is_none(), "expecting no auth info for unauthenticated access");
    }

    #[test]
    fn test_stream() {
        let client = create_test_twitch_client();
        let channel_stream = client.stream("test_channel").unwrap();
        assert!(channel_stream.stream().is_none(), "expecting test channel stream to be offline");
    }

    #[test]
    fn test_streams_with_default_params() {
        let client = create_test_twitch_client();
        let streams = client.streams(StreamsParams::default()).unwrap();
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
        assert!(streams.total() > 0, "streams.total() = {} > 0", streams.total());
        assert_eq!(streams.streams().len(), 2);
    }

    #[test]
    fn test_featured_streams_with_default_params() {
        let client = create_test_twitch_client();
        let featured_streams = client.featured_streams(FeaturedStreamsParams::default()).unwrap();
        assert!(featured_streams.featured().len() > 0, "featured_streams.featured().len() = {} > 0", featured_streams.featured().len());
    }

    #[test]
    fn test_featured_streams_with_custom_params() {
        let client = create_test_twitch_client();
        let params = FeaturedStreamsParams::new()
                .with_offset(0)
                .with_limit(2);
        let featured_streams = client.featured_streams(params).unwrap();
        assert_eq!(featured_streams.featured().len(), 2);
    }

    #[test]
    fn test_streams_summary_with_default_params() {
        let client = create_test_twitch_client();
        let streams_summary = client.streams_summary(StreamsSummaryParams::default()).unwrap();
        assert!(streams_summary.channels() > 0, "streams_summary.channels() = {} > 0", streams_summary.channels());
        assert!(streams_summary.viewers() > 0, "streams_summary.viewers() = {} > 0", streams_summary.viewers());
    }

    #[test]
    fn test_streams_summary_with_custom_params() {
        let client = create_test_twitch_client();
        let params = StreamsSummaryParams::new()
                .with_game("Overwatch");
        client.streams_summary(params).unwrap();
    }

    #[test]
    fn test_channel() {
        let client = create_test_twitch_client();
        let channel = client.channel("test_channel").unwrap();
        assert_eq!(channel.name(), "test_channel");
        assert!(channel.url().find("test_channel").is_some(), "channel.url should contain \"test_channel\"");
        assert!(channel.views() > 0, "channel.views() = {} > 0", channel.views());
        assert!(channel.followers() > 0, "channel.followers() = {} > 0", channel.followers());
    }



    fn create_test_twitch_client() -> TwitchClient {
        let auth = read_auth();
        TwitchClient::new().unwrap().with_client_id(&auth.client_id)
    }

    fn read_auth() -> Auth {
        let mut auth_file = match File::open("twitch_auth.json") {
            Ok(file) => file,
            Err(_) => {
                panic!("File twitch_auth.json required at the crate root directory \
                        for the Twitch Client-ID. Have a look at twitch_auth_template.json");
            },
        };
        let mut auth_string = String::new();
        auth_file.read_to_string(&mut auth_string).unwrap();
        let auth: Auth = serde_json::from_str(&auth_string).unwrap();
        auth
    }

    #[derive(Deserialize, Debug)]
    struct Auth {
        name: String,
        client_id: String,
        redirect_uri: Option<String>,
        client_secret: Option<String>,
    }
}
