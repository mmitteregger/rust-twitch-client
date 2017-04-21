#![doc(html_root_url = "http://mmitteregger.github.io/rust-twitch-client")]
#![warn(missing_docs)]
#![cfg_attr(test, deny(missing_docs))]
#![cfg_attr(test, deny(warnings))]

//! # Overview
//!
//! Rust Twitch Client is a library for the [Twitch REST API] written in Rust!
//!
//! It uses [hyper] with [native_tls] as https client
//! and [serde] for the serialization and deserialization of the REST requests and responses.
//!
//! # Examples
//!
//! ```
//! use twitch_client::*;
//!
//! fn main() {
//!     let twitch_client = TwitchClient::new("<YOUR_TWITCH_CLIENT_ID>").unwrap();
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
//!
//! [Twitch REST API]: https://dev.twitch.tv/docs
//! [hyper]: https://hyper.rs/
//! [native_tls]: https://docs.rs/crate/native-tls
//! [serde]: https://serde.rs/

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

/// Readonly client for the [Twitch REST API].
///
/// Currently [Twitch API version 3] is used.
///
/// By using the Twitch Client you agree to follow the
/// [Twitch Developer Services Agreement] and the [Twitch Terms of Service].
/// This library is in no way affiliated with, authorized, maintained, sponsored
/// or endorsed by Twitch or any of its affiliates or subsidiaries
///
/// # Examples
///
/// ```
/// use twitch_client::*;
///
/// let twitch_client = TwitchClient::new("<YOUR_TWITCH_CLIENT_ID>").unwrap();
///
/// match twitch_client.top_games(TopGamesParams::default()) {
///     Ok(top_games) => println!("Total games: {}", top_games.total()),
///     Err(err) => println!("Failed to retrieve top games: {}", err),
/// }
/// ```
///
/// [Twitch REST API]: https://dev.twitch.tv/docs
/// [Twitch API version 3]: https://dev.twitch.tv/docs/v3
/// [Twitch Developer Services Agreement]: https://www.twitch.tv/p/developer-agreement
/// [Twitch Terms of Service]: https://help.twitch.tv/customer/portal/articles/735191-terms-of-service
pub struct TwitchClient {
    http_client: TwitchHttpClient,
}

impl TwitchClient {

    /// Constructs a new client instance with a new hyper https client using native tls.
    ///
    /// Since [2016-08-06] a Twitch Client ID is required.
    /// Instructions for obtaining it can be found at the [Twitch API Documentation].
    ///
    /// [2016-08-06]: https://blog.twitch.tv/client-id-required-for-kraken-api-calls-afbb8e95f843
    /// [Twitch API Documentation]: https://dev.twitch.tv/docs/v5/guides/using-the-twitch-api/#getting-a-client-id
    pub fn new<S: Into<String>>(client_id: S) -> Result<TwitchClient> {
        let http_client = try!(TwitchHttpClient::new(client_id));

        let twitch_client = TwitchClient {
            http_client: http_client,
        };
        Ok(twitch_client)
    }

    /// Constructs a new client instance using the provided hyper client.
    ///
    /// Note that the provided hyper client needs to use a tls connection.
    ///
    /// Since [2016-08-06] a Twitch Client ID is required.
    /// Instructions for obtaining it can be found at the [Twitch API Documentation].
    ///
    /// [2016-08-06]: https://blog.twitch.tv/client-id-required-for-kraken-api-calls-afbb8e95f843
    /// [Twitch API Documentation]: https://dev.twitch.tv/docs/v5/guides/using-the-twitch-api/#getting-a-client-id
    pub fn with_hyper_client<S: Into<String>>(client_id: S, hyper_client: hyper::Client) -> TwitchClient {
        let http_client = TwitchHttpClient::with_hyper_client(client_id, hyper_client);

        let twitch_client = TwitchClient {
            http_client: http_client,
        };
        twitch_client
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
    use std::env;
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
        TwitchClient::new(auth.client_id).unwrap()
    }

    fn read_auth() -> Auth {
        match env::var("TWITCH_CLIENT_ID") {
            Ok(twitch_client_id) => {
                return Auth {
                    name: "Rust Twitch Client".to_owned(),
                    client_id: twitch_client_id,
                    redirect_uri: None,
                    client_secret: None,
                };
            },
            Err(var_error) => {
                match var_error {
                    env::VarError::NotPresent => {},
                    env::VarError::NotUnicode(var) => {
                        panic!("environment variable TWITCH_CLIENT_ID \
                                did not contain valid unicode data: {:?}", var)
                    },
                }
            }
        }

        let mut auth_file = match File::open("twitch_auth.json") {
            Ok(file) => file,
            Err(_) => {
                panic!("Either the environment variable TWITCH_CLIENT_ID needs to be set \
                        or a file named twitch_auth.json is required at the crate root directory. \
                        Have a look at twitch_auth_template.json");
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
