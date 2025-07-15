use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
};

use thiserror::Error;

use crate::{
    formats::sol::{Amf0DeserializationError, Amf0SerializationError},
    track::{
        TrackBuilderError, layer::layer_group::LayerGroupBuilderError,
        line::line_group::LineGroupBuilderError, rider::rider_group::RiderGroupBuilderError,
    },
    util::ParseLengthPrefixedStringError,
};

#[derive(Error, Debug)]
pub enum SolReadError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    TryFromInt(#[from] TryFromIntError),
    #[error("{0}")]
    IntConversion(#[from] ParseIntError),
    #[error("{0}")]
    FloatConversion(#[from] ParseFloatError),
    #[error("{0}")]
    StringParsing(#[from] ParseLengthPrefixedStringError),
    #[error("{0}")]
    TrackGroup(#[from] TrackBuilderError),
    #[error("{0}")]
    LineGroup(#[from] LineGroupBuilderError),
    #[error("{0}")]
    RiderGroup(#[from] RiderGroupBuilderError),
    #[error("{0}")]
    LayerGroup(#[from] LayerGroupBuilderError),
    #[error("Invalid value for `{name}`: {value}")]
    InvalidData { name: String, value: String },
    #[error("{0}")]
    Amf0Deserialization(#[from] Amf0DeserializationError),
}

#[derive(Error, Debug)]
pub enum SolWriteError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    IntConversion(#[from] TryFromIntError),
    #[error("{0}")]
    Amf0Serialization(#[from] Amf0SerializationError),
}
