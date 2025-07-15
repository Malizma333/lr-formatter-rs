use byteorder::{BigEndian, WriteBytesExt};
use std::{
    collections::HashMap,
    io::{Cursor, Seek, Write},
};

use crate::{
    formats::sol::{
        SolWriteError,
        amf0::{Amf0Value, serialize},
    },
    track::{GridVersion, Track, Vec2},
};

pub fn write(track: &Track) -> Result<Vec<u8>, SolWriteError> {
    let mut cursor = Cursor::new(Vec::new());

    cursor.write_all(b"\x00\xBF")?;
    cursor.write_all(b"\x00\x00\x00\x00")?;
    cursor.write_all(b"TCSO")?;
    cursor.write_all(b"\x00\x04\x00\x00\x00\x00")?;
    cursor.write_all(b"\x00\x0AsavedLines")?;
    cursor.write_all(b"\x00\x00\x00\x00")?;
    cursor.write_all(b"\x00\x09trackList")?;

    let mut lines_vec = vec![];

    for line in track.line_group().standard_lines() {
        let ext = if line.left_extension() { 1.0 } else { 0.0 }
            + if line.right_extension() { 2.0 } else { 0.0 };
        let inv = if line.flipped() { 1.0 } else { 0.0 };
        let numeric_line_type = 0.0;
        let mut line_object = HashMap::new();
        line_object.insert("0".to_string(), Amf0Value::Number(line.x1()));
        line_object.insert("1".to_string(), Amf0Value::Number(line.y1()));
        line_object.insert("2".to_string(), Amf0Value::Number(line.x2()));
        line_object.insert("3".to_string(), Amf0Value::Number(line.y2()));
        line_object.insert("4".to_string(), Amf0Value::Number(ext));
        line_object.insert("5".to_string(), Amf0Value::Number(inv));
        line_object.insert("6".to_string(), Amf0Value::Number(0.0));
        line_object.insert("7".to_string(), Amf0Value::Number(0.0));
        line_object.insert("8".to_string(), Amf0Value::Number(line.id() as f64));
        line_object.insert("9".to_string(), Amf0Value::Number(numeric_line_type));
        lines_vec.push(line_object);
    }

    for line in track.line_group().acceleration_lines() {
        let ext = if line.left_extension() { 1.0 } else { 0.0 }
            + if line.right_extension() { 2.0 } else { 0.0 };
        let inv = if line.flipped() { 1.0 } else { 0.0 };
        let numeric_line_type = 1.0;
        let mut line_object = HashMap::new();
        line_object.insert("0".to_string(), Amf0Value::Number(line.x1()));
        line_object.insert("1".to_string(), Amf0Value::Number(line.y1()));
        line_object.insert("2".to_string(), Amf0Value::Number(line.x2()));
        line_object.insert("3".to_string(), Amf0Value::Number(line.y2()));
        line_object.insert("4".to_string(), Amf0Value::Number(ext));
        line_object.insert("5".to_string(), Amf0Value::Number(inv));
        line_object.insert("6".to_string(), Amf0Value::Number(0.0));
        line_object.insert("7".to_string(), Amf0Value::Number(0.0));
        line_object.insert("8".to_string(), Amf0Value::Number(line.id() as f64));
        line_object.insert("9".to_string(), Amf0Value::Number(numeric_line_type));
        lines_vec.push(line_object);
    }

    for line in track.line_group().scenery_lines() {
        let mut line_object = HashMap::new();
        line_object.insert("0".to_string(), Amf0Value::Number(line.x1()));
        line_object.insert("1".to_string(), Amf0Value::Number(line.y1()));
        line_object.insert("2".to_string(), Amf0Value::Number(line.x2()));
        line_object.insert("3".to_string(), Amf0Value::Number(line.y2()));
        line_object.insert("4".to_string(), Amf0Value::Number(0.0));
        line_object.insert("5".to_string(), Amf0Value::Number(0.0));
        line_object.insert("6".to_string(), Amf0Value::Number(0.0));
        line_object.insert("7".to_string(), Amf0Value::Number(0.0));
        line_object.insert("8".to_string(), Amf0Value::Number(line.id() as f64));
        line_object.insert("9".to_string(), Amf0Value::Number(2.0));
        lines_vec.push(line_object);
    }

    lines_vec.sort_unstable_by(|line_a, line_b| {
        let id_a = line_a.get("8").unwrap().clone().get_number().unwrap();
        let id_b = line_b.get("8").unwrap().clone().get_number().unwrap();
        id_b.partial_cmp(&id_a).unwrap()
    });

    let mut line_array_object = HashMap::new();

    for (index, line_object) in lines_vec.iter().enumerate() {
        line_array_object.insert(index.to_string(), Amf0Value::ECMAArray(line_object.clone()));
    }

    let string_grid_version = match track.metadata().grid_version() {
        GridVersion::V6_0 => "6.0",
        GridVersion::V6_1 => "6.1",
        GridVersion::V6_2 => "6.2",
    }
    .to_string();

    let line_count = (track.line_group().standard_lines().len()
        + track.line_group().acceleration_lines().len()
        + track.line_group().scenery_lines().len()) as f64;

    let start_position = if let Some(start_pos) = track.metadata().start_position() {
        start_pos
    } else {
        Vec2::new(0.0, 0.0)
    };

    let mut array_start_position = HashMap::new();
    array_start_position.insert("0".to_string(), Amf0Value::Number(start_position.x()));
    array_start_position.insert("1".to_string(), Amf0Value::Number(start_position.y()));

    let mut first_null_array = HashMap::new();
    first_null_array.insert("0".to_string(), Amf0Value::Null);
    first_null_array.insert("1".to_string(), Amf0Value::Null);
    first_null_array.insert("2".to_string(), Amf0Value::Null);

    let mut second_null_array = HashMap::new();
    second_null_array.insert("0".to_string(), Amf0Value::Null);
    second_null_array.insert("1".to_string(), Amf0Value::Null);
    second_null_array.insert("2".to_string(), Amf0Value::Null);
    second_null_array.insert("3".to_string(), Amf0Value::Null);
    second_null_array.insert("4".to_string(), Amf0Value::Boolean(true));

    let mut track_data = HashMap::new();
    track_data.insert("0".to_string(), Amf0Value::Null);
    track_data.insert("1".to_string(), Amf0Value::ECMAArray(first_null_array));
    track_data.insert("2".to_string(), Amf0Value::ECMAArray(second_null_array));

    let label = if let Some(title) = track.metadata().title() {
        title
    } else {
        ""
    };

    let mut sol_track = HashMap::new();
    sol_track.insert(
        "label".to_string(),
        Amf0Value::Utf8String(label.to_string()),
    );
    sol_track.insert(
        "version".to_string(),
        Amf0Value::Utf8String(string_grid_version),
    );
    sol_track.insert(
        "startLine".to_string(),
        Amf0Value::ECMAArray(array_start_position),
    );
    sol_track.insert("level".to_string(), Amf0Value::Number(line_count));
    sol_track.insert("data".to_string(), Amf0Value::ECMAArray(line_array_object));

    if track.metadata().zero_velocity_start_riders() {
        sol_track.insert("trackData".to_string(), Amf0Value::ECMAArray(track_data));
    }

    let mut track_list = HashMap::new();
    track_list.insert("0".to_string(), Amf0Value::Object(sol_track));

    // Serialize and write the data
    let data = vec![Amf0Value::ECMAArray(track_list)];
    let buffer = serialize(&data)?;
    cursor.write_all(&buffer)?;
    cursor.write_u8(0x00)?;

    // Go back to write file size
    let file_size = u32::try_from(cursor.position() - 6)?;
    cursor.seek(std::io::SeekFrom::Start(2))?;
    cursor.write_u32::<BigEndian>(file_size)?;

    Ok(cursor.into_inner())
}
