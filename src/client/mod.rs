use hyper;
use serde_json;

pub use model::paging::Paging;
use self::http::TwitchHttpClient;
use error::Result;
use model;

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
    pub fn get_top_games(&self, paging: &Paging) -> Result<model::games::TopGames> {
        let response = try!(self.http_client.get_paged_content("/games/top", paging));
        let top_games: model::games::TopGames = try!(serde_json::from_str(&response));
        Ok(top_games)
    }

    pub fn get_ingests(&self) -> Result<Vec<model::ingests::Ingest>> {
        let response = try!(self.http_client.get_content("/ingests"));
        let ingests: model::ingests::Ingests = try!(serde_json::from_str(&response));
        Ok(ingests.ingests())
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
    use model::paging::Paged;
    use std::fs::File;
    use std::io::Read;
    use serde_json;

    #[test]
    fn test_get_top_games() {
        let client = create_test_twitch_client();
        let paging = Paging::new(0, 2);
        let top_games = client.get_top_games(&paging).unwrap();
        assert_eq!(top_games.current_page_link(), "https://api.twitch.tv/kraken/games/top?limit=2&offset=0");
        assert_eq!(top_games.next_page_link(), "https://api.twitch.tv/kraken/games/top?limit=2&offset=2");
        assert_eq!(top_games.paging(), paging);
        assert!(top_games.total() > 0, "top_games.total() = {} > 0", top_games.total());
        assert_eq!(top_games.top().len(), 2);
    }

    #[test]
    fn test_get_ingests() {
        let client = create_test_twitch_client();
        let ingests = client.get_ingests().unwrap();
        assert!(ingests.len() > 0, "ingests.len() = {} > 0", ingests.len());
    }

    #[test]
    fn test_get_basic_info() {
        let client = create_test_twitch_client();
        let basic_info = client.get_basic_info().unwrap();
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
