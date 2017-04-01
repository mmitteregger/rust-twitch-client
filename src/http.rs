use std::io::Read;
use hyper;
use hyper::net::HttpsConnector;
use hyper::Url;
use hyper::header::{Headers, Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusClass;
use hyper_native_tls::NativeTlsClient;

use error::{Result, Error};


header! { (ClientId, "Client-ID") => [String] }

const BASE_URL: &'static str = "https://api.twitch.tv/kraken";


pub trait IntoQueryString {
    fn into_query_string(self) -> String;
}

pub struct TwitchHttpClient {
    client_id: Option<String>,
    hyper_client: hyper::Client,
}

impl TwitchHttpClient {

    pub fn new() -> Result<TwitchHttpClient> {
        let ssl = try!(NativeTlsClient::new());
        let connector = HttpsConnector::new(ssl);
        let hyper_client = hyper::Client::with_connector(connector);

        let twitch_http_client = TwitchHttpClient {
            client_id: None,
            hyper_client: hyper_client,
        };
        Ok(twitch_http_client)
    }

    pub fn set_client_id(&mut self, client_id: &str) {
        self.client_id = Some(client_id.to_owned());
    }

    pub fn set_hyper_client(&mut self, hyper_client: hyper::Client) {
        self.hyper_client = hyper_client;
    }

    pub fn get_content(&self, relative_url: &str) -> Result<String> {
        let url_string = self.create_url_string(&relative_url);
        let url = Url::parse(&url_string).unwrap();
        self.get_content_from_url(url)
    }

    pub fn get_content_with_params<Q: IntoQueryString>(&self, relative_url: &str, params: Q) -> Result<String> {
        let mut url_string = self.create_url_string(&relative_url);
        url_string.push_str(&params.into_query_string());
        let url = Url::parse(&url_string).unwrap();
        self.get_content_from_url(url)
    }

    pub fn create_url_string(&self, relative_url: &str) -> String {
        let mut url_string = String::from(BASE_URL);
        url_string.push_str(relative_url);
        url_string
    }

    fn get_content_from_url(&self, url: Url) -> Result<String> {
        let headers = self.create_default_headers();
        let request = self.hyper_client.get(url.clone()).headers(headers);
        let mut response = try!(request.send());

        match response.status.class() {
            StatusClass::Success => {
                let mut response_body = String::new();
                try!(response.read_to_string(&mut response_body));
                Ok(response_body)
            }
            _ => Err(Error::Http(response))
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
