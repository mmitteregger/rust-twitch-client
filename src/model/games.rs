use std::collections::BTreeMap;

pub use model::TwitchLinks;
pub use model::paging::Paged;
pub use model::image::ImageLinks;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct GameInfo {
    viewers: u32,
    channels: u32,
    game: Game,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Game {
    #[serde(rename="_links")]
    links: BTreeMap<String, String>,
    #[serde(rename="_id")]
    id: u64,
    giantbomb_id: u64,
    name: String,
    #[serde(rename="box")]
    box_image_links: ImageLinks,
    #[serde(rename="logo")]
    logo_image_links: ImageLinks,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct TopGames {
    #[serde(rename="_links")]
    links: BTreeMap<String, String>,
    #[serde(rename="_total")]
    total: u32,
    top: Vec<GameInfo>,
}


impl GameInfo {
    pub fn viewers(&self) -> u32 {
        self.viewers
    }
    pub fn channels(&self) -> u32 {
        self.channels
    }
    pub fn game(&self) -> &Game {
        &self.game
    }
}

impl TwitchLinks for Game {
    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

impl Game {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn giantbomb_id(&self) -> u64 {
        self.giantbomb_id
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn box_image_links(&self) -> &ImageLinks {
        &self.box_image_links
    }
    pub fn logo_image_links(&self) -> &ImageLinks {
        &self.logo_image_links
    }
}

impl TwitchLinks for TopGames {
    fn links(&self) -> &BTreeMap<String, String> {
        &self.links
    }
}

impl Paged for TopGames {}

//impl_paged!(TopGames);
impl TopGames {
    pub fn total(&self) -> u32 {
        self.total
    }
    pub fn top(&self) -> &Vec<GameInfo> {
        &self.top
    }
}