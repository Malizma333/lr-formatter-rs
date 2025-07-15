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
    IntConversion(#[from] ParseIntError),
    #[error("{0}")]
    FloatConversion(#[from] ParseFloatError),
    #[error("{0}")]
    StringParsing(#[from] ParseLengthPrefixedStringError),
    #[error("{0}")]
    TrackBuilderError(#[from] TrackBuilderError),
    #[error("{0}")]
    LineGroupBuilderError(#[from] LineGroupBuilderError),
    #[error("{0}")]
    RiderGroupBuilderError(#[from] RiderGroupBuilderError),
    #[error("{0}")]
    LayerGroupBuilderError(#[from] LayerGroupBuilderError),
    // TODO maybe remove this
    #[error("Invalid value for `{name}`: {value}")]
    InvalidData { name: String, value: String },
    #[error("{0}")]
    FromUTF8Error(#[from] FromUtf8Error),
    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum JsonWriteError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    IntConversion(#[from] TryFromIntError),
    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),
}
