pub use model::UrlString;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct ImageLinks {
    template: UrlString,
    small: UrlString,
    medium: UrlString,
    large: UrlString,
}


impl ImageLinks {
    pub fn template(&self) -> &UrlString {
        &self.template
    }
    pub fn small(&self) -> &UrlString {
        &self.small
    }
    pub fn medium(&self) -> &UrlString {
        &self.medium
    }
    pub fn large(&self) -> &UrlString {
        &self.large
    }
}
