//! Twitch return types.

pub mod image;
pub mod game;
pub mod ingest;
pub mod root;
pub mod stream;
pub mod channel;


/// Strings that contain a hyperlink (e.g.: "http://static-cdn.jtvnw.net/jtv_user_pictures/test_channel-profile_image-94a42b3a13c31c02-300x300.jpeg").
///
/// Is subject to be changed to a real hyperlink type in the future.
pub type UrlString = String;

/// Strings that contain a date in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format (e.g.: "2015-02-12T04:42:31Z").
///
/// Is subject to be changed to a real datetime type in the future.
pub type DateString = String;

/// Strings that contain a locale in [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1) codes format (2 letter locales e.g.: "en").
///
/// Is subject to be changed to a real locale type in the future.
pub type LocaleString = String;

