pub use model::UrlString;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct ImageLinks {
    large: UrlString,
    medium: UrlString,
    small: UrlString,
    template: UrlString,
}
