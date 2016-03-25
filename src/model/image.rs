//! Twitch images.
//!
//! Image types used in other models.


pub use model::UrlString;

/// Twitch (preview) image links.
///
/// Various image links with different resolutions.
///
/// # Example in JSON
///
/// ```json
/// {
///   "small": "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-80x45.jpg",
///   "medium": "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-320x180.jpg",
///   "large": "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-640x360.jpg",
///   "template": "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-{width}x{height}.jpg"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageLinks {
    template: UrlString,
    small: UrlString,
    medium: UrlString,
    large: UrlString,
}


impl ImageLinks {
    /// Example value: "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-{width}x{height}.jpg"
    pub fn template(&self) -> &UrlString {
        &self.template
    }
    /// Example value: "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-80x45.jpg"
    pub fn small(&self) -> &UrlString {
        &self.small
    }
    /// Example value: "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-320x180.jpg"
    pub fn medium(&self) -> &UrlString {
        &self.medium
    }
    /// Example value: "http://static-cdn.jtvnw.net/previews-ttv/live_user_test_channel-640x360.jpg"
    pub fn large(&self) -> &UrlString {
        &self.large
    }
}
