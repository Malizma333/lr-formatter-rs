use crate::{
    formats::lrb::{ModHandler, mod_flags},
    util::{StringLength, parse_string},
};
use byteorder::{LittleEndian, WriteBytesExt};
use once_cell::sync::Lazy;
use std::io::Write;

// label: u16 length string = the track's label

pub(in crate::formats::lrb) static LABEL: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: mod_flags::EXTRA_DATA,
    read: Box::new(|cursor, track_builder| {
        track_builder
            .metadata()
            .title(parse_string::<LittleEndian>(cursor, StringLength::U16)?);

        Ok(())
    }),
    write: Box::new(|cursor, track| {
        let title = track.metadata().title().clone().unwrap_or("".to_string());
        cursor.write_u16::<LittleEndian>(u16::try_from(title.len())?)?;
        cursor.write_all(title.as_bytes())?;

        Ok(())
    }),
});
