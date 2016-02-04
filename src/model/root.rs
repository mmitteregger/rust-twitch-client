use std::collections::BTreeMap;

pub use model::TwitchLinks;


#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct BasicInfo {
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
    created_at: String,
    updated_at: String,
}


impl TwitchLinks for BasicInfo {
    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

impl BasicInfo {
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
    pub fn created_at(&self) -> &String {
        &self.created_at
    }
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }
}
