use std::collections::BTreeMap;

pub mod paging;
pub mod image;
pub mod games;
pub mod ingests;

pub trait TwitchLinks {
    fn links(&self) -> &BTreeMap<String, String>;

    fn get_expected_link(&self, link_key: &str) -> &String {
        match self.links().get(link_key) {
            Some(link) => link,
            None => {
                panic!("Expected links to contain {} but got: {:?}", link_key, self.links());
            }
        }
    }
}
