use hyper;
use serde_json;

pub use self::param::*;
use self::http::TwitchHttpClient;
use error::Result;
use model;

mod param;
mod http;


pub struct TwitchClient {
    http_client: TwitchHttpClient,
}

impl TwitchClient {
    pub fn new() -> TwitchClient {
        TwitchClientBuilder::new().build()
    }
}

pub struct TwitchClientBuilder {
    client_id: Option<String>,
    hyper_client: Option<hyper::Client>,
}

impl TwitchClientBuilder {
    pub fn new() -> TwitchClientBuilder {
        TwitchClientBuilder {
            client_id: None,
            hyper_client: None,
        }
    }

    pub fn client_id(mut self, client_id: String) -> TwitchClientBuilder {
        self.client_id = Some(client_id);
        self
    }

    pub fn hyper_client(mut self, hyper_client: hyper::Client) -> TwitchClientBuilder {
        self.hyper_client = Some(hyper_client);
        self
    }

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
    pub fn get_top_games(&self, params: &TopGamesParams) -> Result<model::game::TopGames> {
        let response = try!(self.http_client.get_content_with_params("/games/top", params));
        let top_games: model::game::TopGames = try!(serde_json::from_str(&response));
        Ok(top_games)
    }

    pub fn get_ingests(&self) -> Result<model::ingest::Ingests> {
        let response = try!(self.http_client.get_content("/ingests"));
        let ingests: model::ingest::Ingests = try!(serde_json::from_str(&response));
        Ok(ingests)
    }

    pub fn get_basic_info(&self) -> Result<model::root::BasicInfo> {
        let response = try!(self.http_client.get_content("/"));
        let basic_info: model::root::BasicInfo = try!(serde_json::from_str(&response));
        Ok(basic_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use serde_json;

    #[test]
    fn test_get_top_games_default_params() {
        let client = create_test_twitch_client();
        let top_games = client.get_top_games(&TopGamesParams::default()).unwrap();
        assert_eq!(top_games.link_self(), "https://api.twitch.tv/kraken/games/top?limit=10&offset=0");
        assert_eq!(top_games.link_next(), "https://api.twitch.tv/kraken/games/top?limit=10&offset=10");
        assert!(top_games.total() > 0, "top_games.total() = {} > 0", top_games.total());
        assert_eq!(top_games.top().len(), 10);
    }

    #[test]
    fn test_get_top_games_custom_params() {
        let client = create_test_twitch_client();
        let params = TopGamesParamsBuilder::default()
                .offset(0)
                .limit(2)
                .build();
        let top_games = client.get_top_games(&params).unwrap();
        assert_eq!(top_games.link_self(), "https://api.twitch.tv/kraken/games/top?limit=2&offset=0");
        assert_eq!(top_games.link_next(), "https://api.twitch.tv/kraken/games/top?limit=2&offset=2");
        assert!(top_games.total() > 0, "top_games.total() = {} > 0", top_games.total());
        assert_eq!(top_games.top().len(), 2);
    }

    #[test]
    fn test_get_ingests() {
        let client = create_test_twitch_client();
        let ingests = client.get_ingests().unwrap();
        assert_eq!(ingests.link_self(), "https://api.twitch.tv/kraken/ingests");
        assert!(ingests.ingests().len() > 0, "ingests.ingests().len() = {} > 0", ingests.ingests().len());
    }

    #[test]
    fn test_get_basic_info() {
        let client = create_test_twitch_client();
        let basic_info = client.get_basic_info().unwrap();
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


    fn create_test_twitch_client() -> TwitchClient {
        let auth = read_auth();
        let twitch_client = TwitchClientBuilder::new()
            .client_id(auth.client_id)
            .build();
        twitch_client
    }

    fn read_auth() -> Auth {
        let mut auth_file = match File::open("twitch_auth.json") {
            Ok(file) => file,
            Err(err) => panic!("
            Tests create a new TwitchClient with info specified in 'twitch_auth.json'.
            As this info should be kept private it is not committed to the vcs.
            You need to create the file in order to run these tests.
            An template named 'twitch_auth_template.json' is provided at the project root.
            Original error: {}
            ", err),
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
