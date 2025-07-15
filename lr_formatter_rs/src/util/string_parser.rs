use byteorder::{ByteOrder, ReadBytesExt};
use std::io::{self, Read};
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseLengthPrefixedStringError {
    #[error("IO error while reading string: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid UTF-8 while parsing string of length {length}: {source}")]
    Utf8 {
        length: usize,
        #[source]
        source: FromUtf8Error,
    },
}

pub(crate) enum StringLength {
    U8,
    U16,
    Fixed(usize),
}

/// Generalized function for reading binary length-prefixed strings
pub(crate) fn parse_string<B: ByteOrder>(
    cursor: &mut io::Cursor<Vec<u8>>,
    length_type: StringLength,
) -> Result<String, ParseLengthPrefixedStringError> {
    let length = match length_type {
        StringLength::U8 => usize::from(cursor.read_u8()?),
        StringLength::U16 => usize::from(cursor.read_u16::<B>()?),
        StringLength::Fixed(size) => size,
    };

    let mut buffer = vec![0; length];
    cursor.read_exact(&mut buffer)?;
    let string = String::from_utf8(buffer)
        .map_err(|e| ParseLengthPrefixedStringError::Utf8 { length, source: e })?;

    Ok(string)
}
