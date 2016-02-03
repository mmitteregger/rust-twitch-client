use std::io::Read;
use hyper;
use hyper::Url;
use hyper::header::{Headers, Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::{StatusCode, StatusClass};

use model::paging::Paging;
use error::{Result, Error};


header! { (ClientId, "Client-ID") => [String] }

const BASE_URL: &'static str = "https://api.twitch.tv/kraken";


pub struct TwitchHttpClient {
    client_id: Option<String>,
    hyper_client: hyper::Client,
}

impl TwitchHttpClient {
    pub fn new(client_id: Option<String>, hyper_client: hyper::Client) -> TwitchHttpClient {
        TwitchHttpClient {
            client_id: client_id,
            hyper_client: hyper_client
        }
    }

    pub fn get_content(&self, relative_url: &str) -> Result<String> {
        let url_string = self.create_url_string(&relative_url);
        let url = Url::parse(&url_string).unwrap();
        self.get_content_from_url(url)
    }

    pub fn get_paged_content(&self, relative_url: &str, paging: &Paging) -> Result<String> {
        let mut url_string = self.create_url_string(&relative_url);
        self.add_paging(&mut url_string, paging);
        let url = Url::parse(&url_string).unwrap();
        self.get_content_from_url(url)
    }

    pub fn create_url_string(&self, relative_url: &str) -> String {
        let mut url_string = String::from(BASE_URL);
        url_string.push_str(relative_url);
        url_string
    }

    pub fn add_paging(&self, url_string: &mut String, paging: &Paging) {
        if !paging.is_default() {
            url_string.push_str("?limit=");
            url_string.push_str(&paging.limit().unwrap().to_string());
            url_string.push_str("?offset=");
            url_string.push_str(&paging.offset().unwrap().to_string());
        }
    }

    fn get_content_from_url(&self, url: Url) -> Result<String> {
        let headers = self.create_default_headers();
        let request = self.hyper_client.get(url.clone()).headers(headers);
        let mut response = try!(request.send());

        match response.status.class() {
            StatusClass::Success => {
                if response.status == StatusCode::Ok {
                    let mut response_body = String::new();
                    try!(response.read_to_string(&mut response_body));
                    Ok(response_body)
                } else {
                    panic!("Unhandled success response: {:#?}", response);
                }
            }
            StatusClass::ClientError => {
                if response.status == StatusCode::Unauthorized {
                    Err(Error::Unauthorized(url.clone()))
                } else {
                    panic!("Unhandled client error response: {:#?}", response);
                }
            }
            StatusClass::ServerError => {
                Err(Error::Twitch(response))
            }
            _ => panic!("Unhandled response status: {}", response.status)
        }
    }

    pub fn create_default_headers(&self) -> Headers {
        let mut headers = Headers::new();

        headers.set(Accept(vec![
            qitem(Mime(TopLevel::Application, SubLevel::Ext("vnd.twitchtv.v3+json".to_owned()), vec![])),
        ]));
        match self.client_id {
            Some(ref client_id) => headers.set(ClientId(client_id.to_owned())),
            None => {},
        };

        headers
    }

}
