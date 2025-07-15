use std::{
    io,
    num::{ParseFloatError, ParseIntError},
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
pub enum TrkReadError {
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
    #[error("{0}")]
    FromUTF8Error(#[from] FromUtf8Error),
    // TODO maybe remove this
    #[error("Invalid value for `{name}`: {value}")]
    InvalidData { name: String, value: String },
}
