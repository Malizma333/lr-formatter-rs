use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
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
pub enum LrbReadError {
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
    InvalidData { name: String, value: String },
    #[error("Required mod not supported: {name} v{version}")]
    UnsupportedRequiredMod { name: String, version: u16 },
}

#[derive(Error, Debug)]
pub enum LrbWriteError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    IntConversion(#[from] TryFromIntError),
}
