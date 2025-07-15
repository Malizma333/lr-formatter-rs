use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
    string::FromUtf8Error,
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
pub enum LrbReadError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    IntConversion(#[from] ParseIntError),
    #[error("{0}")]
    FloatConversion(#[from] ParseFloatError),
    #[error("{0}")]
    StringParsing(#[from] ParseLengthPrefixedStringError),
    #[error("{0}")]
    Amf0Deserialization(#[from] Amf0DeserializationError),
    #[error("{0}")]
    TrackBuilderError(#[from] TrackBuilderError),
    #[error("{0}")]
    LineGroupBuilderError(#[from] LineGroupBuilderError),
    #[error("{0}")]
    RiderGroupBuilderError(#[from] RiderGroupBuilderError),
    #[error("{0}")]
    LayerGroupBuilderError(#[from] LayerGroupBuilderError),
    #[error("{0}")]
    FromUTF8Error(#[from] FromUtf8Error),
    #[error("Required mod not supported: {name} v{version}")]
    UnsupportedRequiredMod { name: String, version: u16 },
    // TODO maybe remove this
    #[error("Invalid value for `{name}`: {value}")]
    InvalidData { name: String, value: String },
}

#[derive(Error, Debug)]
pub enum LrbWriteError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    IntConversion(#[from] TryFromIntError),
    #[error("{0}")]
    Amf0Serialization(#[from] Amf0SerializationError),
}
