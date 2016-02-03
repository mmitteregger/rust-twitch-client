#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct ImageLinks {
    large: String,
    medium: String,
    small: String,
    template: String,
}
