use std::collections::BTreeMap;

pub use model::TwitchLinks;
pub use model::DateString;


#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct BasicInfoResponse {
    #[serde(rename="_links")]
    links: BTreeMap<String, String>,
    token: Token,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Token {
    valid: bool,
    user_name: Option<String>,
    authorization: Option<Authorization>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Authorization {
    scopes: Vec<String>,
    created_at: DateString,
    updated_at: DateString,
}


impl TwitchLinks for BasicInfoResponse {
    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

impl BasicInfoResponse {
    pub fn link_user(&self) -> &String {
        self.get_expected_link("user")
    }
    pub fn link_channel(&self) -> &String {
        self.get_expected_link("channel")
    }
    pub fn link_search(&self) -> &String {
        self.get_expected_link("search")
    }
    pub fn link_streams(&self) -> &String {
        self.get_expected_link("streams")
    }
    pub fn link_ingests(&self) -> &String {
        self.get_expected_link("ingests")
    }
    pub fn link_teams(&self) -> &String {
        self.get_expected_link("teams")
    }
    pub fn link_users(&self) -> Option<&String> {
        self.links.get("users")
    }
    pub fn link_channels(&self) -> Option<&String> {
        self.links.get("channels")
    }
    pub fn link_chat(&self) -> Option<&String> {
        self.links.get("chat")
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl Token {
    pub fn valid(&self) -> bool {
        self.valid
    }
    pub fn user_name(&self) -> &Option<String> {
        &self.user_name
    }
    pub fn authorization(&self) -> &Option<Authorization> {
        &self.authorization
    }
}

impl Authorization {
    pub fn scopes(&self) -> &Vec<String> {
        &self.scopes
    }
    pub fn created_at(&self) -> &DateString {
        &self.created_at
    }
    pub fn updated_at(&self) -> &DateString {
        &self.updated_at
    }
}
