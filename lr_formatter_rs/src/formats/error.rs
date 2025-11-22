use crate::formats::json::{JsonReadError, JsonWriteError};
use crate::formats::sol::{SolReadError, SolWriteError};
use crate::formats::trk::TrkReadError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrackReadError {
    #[error("{0}")]
    Trk(#[from] TrkReadError),
    #[error("{0}")]
    Json(#[from] JsonReadError),
    #[error("{0}")]
    Sol(#[from] SolReadError),
}

#[derive(Error, Debug)]
pub enum TrackWriteError {
    #[error("{0}")]
    Json(#[from] JsonWriteError),
    #[error("{0}")]
    Sol(#[from] SolWriteError),
}
