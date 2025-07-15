//! Format used by original flash editions of Line Rider, which includes multiple tracks within the same file

mod amf0;
mod error;
mod reader;
mod writer;

pub use amf0::{Amf0DeserializationError, Amf0SerializationError};
pub use error::{SolReadError, SolWriteError};
pub use reader::read;
pub use writer::write;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Seek};

/// Retrieve the number of tracks an sol file contains
pub fn get_track_count(data: &[u8]) -> u32 {
    let mut cursor = Cursor::new(data);

    // HACK: We assume header size is constant, and track list length will always be at 0x2C - 0x2F
    let _ = cursor.seek(std::io::SeekFrom::Start(0x2C));
    let num_tracks = cursor.read_u32::<BigEndian>().unwrap_or(0);

    num_tracks
}
