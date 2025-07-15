use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
    string::FromUtf8Error,
};

use thiserror::Error;

use crate::{
    track::{
        TrackBuilderError, layer::layer_group::LayerGroupBuilderError,
        line::line_group::LineGroupBuilderError, rider::rider_group::RiderGroupBuilderError,
    },
    util::ParseLengthPrefixedStringError,
};

#[derive(Error, Debug)]
pub enum JsonReadError {
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
    // TODO maybe remove this
    #[error("Invalid value for `{name}`: {value}")]
    InvalidData { name: &'static str, value: String },
    #[error("{0}")]
    FromUTF8(#[from] FromUtf8Error),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum JsonWriteError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    IntConversion(#[from] TryFromIntError),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
}
