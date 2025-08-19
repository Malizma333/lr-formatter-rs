use super::{SUPPORTED_MODS, mod_flags};
use crate::{formats::lrb::LrbWriteError, track::Track};
use byteorder::{LittleEndian, WriteBytesExt};
use std::{
    collections::HashMap,
    io::{Cursor, Seek, SeekFrom, Write},
};

pub fn write(internal: &Track) -> Result<Vec<u8>, LrbWriteError> {
    let mut cursor = Cursor::new(Vec::new());
    let mut mod_table_entry_offsets: HashMap<String, u64> = HashMap::new();

    // Magic number
    cursor.write_all(b"LRB")?;

    // Version
    cursor.write_u8(0)?;

    // Number of mods
    cursor.write_u16::<LittleEndian>(5)?;

    // Mod table
    for mod_identifier in SUPPORTED_MODS.keys() {
        // Name
        let name = mod_identifier.0;
        cursor.write_u8(name.len().try_into()?)?;
        cursor.write_all(name.as_bytes())?;

        // Version
        let version = mod_identifier.1;
        cursor.write_u16::<LittleEndian>(version)?;

        // Flags
        let flags = SUPPORTED_MODS[mod_identifier].flags;
        cursor.write_u8(flags)?;

        // Data address
        if flags & mod_flags::EXTRA_DATA != 0 {
            mod_table_entry_offsets.insert(name.to_string(), cursor.stream_position()?);

            // Allocate space for data address information
            cursor.write_u64::<LittleEndian>(0)?;
            cursor.write_u64::<LittleEndian>(0)?;
        }
    }

    for mod_identifer in SUPPORTED_MODS.keys() {
        let section_start = cursor.stream_position()?;
        let name = mod_identifer.0;
        let writer = &SUPPORTED_MODS[mod_identifer].write;
        (writer)(&mut cursor, internal)?;

        let section_end = cursor.stream_position()?;
        let section_length = section_end - section_start;

        if let Some(&offset) = mod_table_entry_offsets.get(name) {
            cursor.seek(SeekFrom::Start(offset))?;
            cursor.write_u64::<LittleEndian>(section_start)?;
            cursor.write_u64::<LittleEndian>(section_length)?;
            cursor.seek(SeekFrom::Start(section_end))?;
        }
    }

    Ok(cursor.into_inner())
}
