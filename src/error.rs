//! Error and Result module.

use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use hyper::Url;
use hyper::client::response::Response;
use hyper::error::Error as HyperError;
use hyper_native_tls::ServerError as HyperNativeTlsServerError;
use native_tls::Error as NativeTlsError;
use serde_json::error::Error as JsonError;

/// Result type from methods that can have Twitch Client Errors.
pub type Result<T> = ::std::result::Result<T, Error>;

use self::Error::{
    Twitch,
    Unauthorized,
    Io,
    Hyper,
    Tls,
    Deserialization,
};


/// Twitch Client error.
///
/// Wraps errors that may occur during communication with Twitch
/// or when trying to deserialize the JSON response.
///
/// This list is intended to grow over time
/// and it is not recommended to exhaustively match against it.
#[derive(Debug)]
pub enum Error {
    /// An twitch server error that is indicated by the response status 5xx (Server Error)
    Twitch(Response),
    /// Tried to access an secured resource prior to authentication
    Unauthorized(Url),
    /// An `io::Error` that occurred while trying to read or write to a network stream.
    Io(IoError),
    /// An `hyper::error::Error` that occurred while trying to use the hyper library.
    Hyper(HyperError),
    /// An `hyper::error::Error` that occurred while trying to use the hyper library.
    Tls(NativeTlsError),
    /// An `serde_json::error::Error` that occurred while trying to deserialize a json response string.
    Deserialization(JsonError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Twitch(ref _response) => "An twitch server error that is indicated by the response status 5xx (Server Error)",
            Unauthorized(ref _url) => "Tried to access an secured resource prior to authentication",
            Io(ref e) => e.description(),
            Hyper(ref e) => e.description(),
            Tls(ref e) => e.description(),
            Deserialization(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Io(ref error) => Some(error),
            Hyper(ref error) => Some(error),
            Deserialization(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Io(err)
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        match err {
            HyperError::Io(e) => Io(e),
            _ => Hyper(err)
        }
    }
}

impl From<HyperNativeTlsServerError> for Error {
    fn from(err: HyperNativeTlsServerError) -> Error {
        match err {
            HyperNativeTlsServerError::Io(e) => Io(e),
            HyperNativeTlsServerError::Tls(e) => Tls(e),
        }
    }
}

impl From<NativeTlsError> for Error {
    fn from(err: NativeTlsError) -> Error {
        Tls(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Deserialization(err)
    }
}
