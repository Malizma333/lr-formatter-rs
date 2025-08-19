//! <https://github.com/KallDrexx/rust-media-libs>
//! License: See ./LICENSE-APACHE and ./LICENSE-MIT
//! Modifications Copyright 2025 LRBSpec

mod deserialization;
mod errors;
mod serialization;

pub(super) use deserialization::deserialize;
pub use errors::{Amf0DeserializationError, Amf0SerializationError};
pub(super) use serialization::serialize;

use std::collections::HashMap;

// An Enum representing the different supported types of Amf0 values
#[derive(PartialEq, Debug, Clone)]
pub(super) enum Amf0Value {
    Number(f64),
    Boolean(bool),
    Utf8String(String),
    Object(HashMap<String, Amf0Value>),
    ECMAArray(HashMap<String, Amf0Value>),
    StrictArray(Vec<Amf0Value>),
    Null,
    Undefined,
}

impl Amf0Value {
    pub fn get_number(self) -> Option<f64> {
        match self {
            Amf0Value::Number(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_boolean(self) -> Option<bool> {
        match self {
            Amf0Value::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_string(self) -> Option<String> {
        match self {
            Amf0Value::Utf8String(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_object_properties(self) -> Option<HashMap<String, Amf0Value>> {
        match self {
            Amf0Value::Object(properties) => Some(properties),
            _ => None,
        }
    }
}

mod markers {
    pub const NUMBER_MARKER: u8 = 0;
    pub const BOOLEAN_MARKER: u8 = 1;
    pub const STRING_MARKER: u8 = 2;
    pub const OBJECT_MARKER: u8 = 3;
    pub const NULL_MARKER: u8 = 5;
    pub const UNDEFINED_MARKER: u8 = 6;
    pub const REFERENCE_MARKER: u8 = 7;
    pub const ECMA_ARRAY_MARKER: u8 = 8;
    pub const OBJECT_END_MARKER: u8 = 9;
    pub const STRICT_ARRAY_MARKER: u8 = 10;
    pub const UTF_8_EMPTY_MARKER: u16 = 0;
}
