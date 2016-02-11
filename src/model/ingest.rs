use std::collections::BTreeMap;

pub use model::TwitchLinks;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingests {
    #[serde(rename="_links")]
    links: BTreeMap<String, String>,
    ingests: Vec<Ingest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingest {
    name: String,
    availability: f64,
    #[serde(rename="_id")]
    id: u64,
    default: bool,
    url_template: String,
}


impl TwitchLinks for Ingests {
    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

impl Ingests {
    pub fn link_self(&self) -> &String {
        self.get_expected_link("self")
    }
    pub fn ingests(&self) -> &Vec<Ingest> {
        &self.ingests
    }
}

impl Ingest {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn availability(&self) -> f64 {
        self.availability
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn default(&self) -> bool {
        self.default
    }
    pub fn url_template(&self) -> &String {
        &self.url_template
    }
}
