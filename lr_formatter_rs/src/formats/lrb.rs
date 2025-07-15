//! Format proposed to unify feature sets across versions, see [LRBSpec](https://github.com/lrbspec)

mod base;
mod common;
mod error;
mod mod_flags;
mod reader;
mod writer;

pub use error::{LrbReadError, LrbWriteError};
pub use reader::read;
pub use writer::write;

use crate::track::{Track, TrackBuilder};
use base::{GRIDVER, LABEL, SCNLINE, SIMLINE, STARTOFFSET};
use once_cell::sync::Lazy;
use std::{collections::HashMap, io::Cursor};

type ReadLambda =
    Box<dyn Fn(&mut Cursor<Vec<u8>>, &mut TrackBuilder) -> Result<(), LrbReadError> + Send + Sync>;

type WriteLambda =
    Box<dyn Fn(&mut Cursor<Vec<u8>>, &Track) -> Result<(), LrbWriteError> + Send + Sync>;

struct ModHandler {
    flags: u8,
    read: ReadLambda,
    write: WriteLambda,
}

static SUPPORTED_MODS: Lazy<HashMap<(&'static str, u16), &'static Lazy<ModHandler>>> =
    Lazy::new(|| {
        HashMap::from([
            (("base.gridver", 0), &GRIDVER),
            (("base.label", 0), &LABEL),
            (("base.scnline", 0), &SCNLINE),
            (("base.simline", 0), &SIMLINE),
            (("base.startoffset", 0), &STARTOFFSET),
        ])
    });
