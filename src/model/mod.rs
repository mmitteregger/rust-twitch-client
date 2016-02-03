use std::fmt;
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::cmp::Eq;
use std::collections::BTreeMap;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::value::Value as JsonValue;

/*#[macro_use]*/ pub mod paging;
pub mod image;
pub mod games;
pub mod ingests;


/// Wrapper type for a `serde_json::value::Value` with a `std::hash::Hash` implementation
#[derive(PartialEq, Clone)]
struct TwitchJsonValue {
    json_value: JsonValue,
}

impl From<JsonValue> for TwitchJsonValue {
    fn from(json_value: JsonValue) -> TwitchJsonValue {
        TwitchJsonValue {
            json_value: json_value,
        }
    }
}

impl TwitchJsonValue {
    fn hash_json_value<H: Hasher>(json_value: &JsonValue, state: &mut H) {
        match *json_value {
            JsonValue::Null => 0.hash(state),
            JsonValue::Bool(ref b) => b.hash(state),
            JsonValue::I64(ref i) => i.hash(state),
            JsonValue::U64(ref u) => u.hash(state),
            JsonValue::F64(ref _f) => panic!("Cannot hash floating point values"),
            JsonValue::String(ref s) => s.hash(state),
            JsonValue::Array(ref json_values) => {
                for json_value in json_values {
                    TwitchJsonValue::hash_json_value(json_value, state);
                }
            },
            JsonValue::Object(ref json_values_map) => {
                for (key, value) in json_values_map {
                    key.hash(state);
                    TwitchJsonValue::hash_json_value(value, state);
                }
            },
        }
    }
}

impl Hash for TwitchJsonValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        TwitchJsonValue::hash_json_value(&self.json_value, state);
    }
}

impl Eq for TwitchJsonValue {}

impl Serialize for TwitchJsonValue {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        self.json_value.serialize(serializer)
    }
}

impl Deserialize for TwitchJsonValue {
    fn deserialize<D>(deserializer: &mut D) -> Result<TwitchJsonValue, D::Error> where D: Deserializer {
        let json_value = try!(JsonValue::deserialize(deserializer));
        let twitch_json_value = TwitchJsonValue {
            json_value: json_value,
        };
        Ok(twitch_json_value)
    }
}

impl fmt::Debug for TwitchJsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.json_value)
    }
}

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
