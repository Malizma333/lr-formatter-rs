use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::{
    formats::sol::{SolReadError, amf0::deserialize},
    track::{GridVersion, LineType, Track, TrackBuilder, Vec2},
    util::{StringLength, bytes_to_hex_string, parse_string},
};

pub fn read(data: Vec<u8>, track_index: Option<u32>) -> Result<Track, SolReadError> {
    let track_builder = &mut TrackBuilder::default();
    let data_size = u64::try_from(data.len())?;
    let mut cursor = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 2];
    cursor.read_exact(&mut magic_number)?;

    if magic_number != [0x00, 0xBF] {
        return Err(SolReadError::InvalidData {
            name: "magic number".to_string(),
            value: bytes_to_hex_string(&magic_number),
        });
    }

    // Header
    let _file_size = cursor.read_u32::<BigEndian>()? + 6;

    let mut tag = [0u8; 4];
    cursor.read_exact(&mut tag)?;

    if tag != [b'T', b'C', b'S', b'O'] {
        return Err(SolReadError::InvalidData {
            name: "header tag".to_string(),
            value: bytes_to_hex_string(&tag),
        });
    }

    let mut marker = [0u8; 6];
    cursor.read_exact(&mut marker)?;
    if marker != [0x00, 0x04, 0x00, 0x00, 0x00, 0x00] {
        return Err(SolReadError::InvalidData {
            name: "header marker".to_string(),
            value: bytes_to_hex_string(&marker),
        });
    }

    let sol_name = parse_string::<BigEndian>(&mut cursor, StringLength::U16)?;
    if sol_name.as_str() != "savedLines" {
        return Err(SolReadError::InvalidData {
            name: "sol name".to_string(),
            value: sol_name.to_string(),
        });
    }

    let _padding = cursor.read_u32::<BigEndian>()?;

    let data_name = parse_string::<BigEndian>(&mut cursor, StringLength::U16)?;
    if data_name.as_str() != "trackList" {
        return Err(SolReadError::InvalidData {
            name: "data name".to_string(),
            value: data_name.to_string(),
        });
    }

    // Track Data
    let current_pos = cursor.position();
    // Slice from current position to last byte - 1 contains valid AMF0 format
    let mut trimmed_cursor = cursor.take(data_size.saturating_sub(1) - current_pos);
    let result = &deserialize(&mut trimmed_cursor)?;

    let track_list_amf = &result[0];
    let track_list =
        track_list_amf
            .clone()
            .get_object_properties()
            .ok_or(SolReadError::InvalidData {
                name: "track list".to_string(),
                value: format!("{:?}", track_list_amf),
            })?;

    let target_track_index = match track_index {
        Some(index) => &index.to_string(),
        None => "0",
    };

    let target_track_amf = track_list
        .get(target_track_index)
        .ok_or(SolReadError::InvalidData {
            name: "track index".to_string(),
            value: format!("{:?}", target_track_index),
        })?;

    let target_track =
        target_track_amf
            .clone()
            .get_object_properties()
            .ok_or(SolReadError::InvalidData {
                name: "track".to_string(),
                value: format!("{:?}", target_track_amf),
            })?;

    if let Some(val) = target_track.get("label") {
        let title = val.clone().get_string().ok_or(SolReadError::InvalidData {
            name: "label".to_string(),
            value: format!("{:?}", val),
        })?;
        track_builder.metadata().title(title);
    }

    if let Some(val) = target_track.get("version") {
        let version_string = val.clone().get_string().ok_or(SolReadError::InvalidData {
            name: "grid version".to_string(),
            value: format!("{:?}", val),
        })?;

        let grid_version = match version_string.as_str() {
            "6.0" => GridVersion::V6_0,
            "6.1" => GridVersion::V6_1,
            "6.2" => GridVersion::V6_2,
            other => {
                return Err(SolReadError::InvalidData {
                    name: "grid version".to_string(),
                    value: other.to_string(),
                });
            }
        };
        track_builder.metadata().grid_version(grid_version);
    } else {
        track_builder.metadata().grid_version(GridVersion::V6_0);
    }

    if let Some(val) = target_track.get("startLine") {
        let start_position =
            val.clone()
                .get_object_properties()
                .ok_or(SolReadError::InvalidData {
                    name: "start line".to_string(),
                    value: format!("{:?}", val),
                })?;

        let start_x_amf = start_position.get("0").ok_or(SolReadError::InvalidData {
            name: "start line x".to_string(),
            value: format!("{:?}", start_position),
        })?;
        let start_pos_x = start_x_amf
            .clone()
            .get_number()
            .ok_or(SolReadError::InvalidData {
                name: "start x value".to_string(),
                value: format!("{:?}", start_x_amf),
            })?;

        let start_y_amf = start_position.get("1").ok_or(SolReadError::InvalidData {
            name: "start line y".to_string(),
            value: format!("{:?}", start_position),
        })?;
        let start_pos_y = start_y_amf
            .clone()
            .get_number()
            .ok_or(SolReadError::InvalidData {
                name: "start y value".to_string(),
                value: format!("{:?}", start_y_amf),
            })?;

        track_builder
            .metadata()
            .start_position(Vec2::new(start_pos_x, start_pos_y));
    }

    if target_track.contains_key("trackData") {
        track_builder.metadata().zero_velocity_start_riders(true);
    }

    if let Some(val) = target_track.get("data") {
        let lines_list = val
            .clone()
            .get_object_properties()
            .ok_or(SolReadError::InvalidData {
                name: "lines list".to_string(),
                value: format!("{:?}", val),
            })?;

        for line_amf in lines_list.values() {
            let line =
                line_amf
                    .clone()
                    .get_object_properties()
                    .ok_or(SolReadError::InvalidData {
                        name: "line".to_string(),
                        value: format!("{:?}", line_amf),
                    })?;

            let x1_amf = line.get("0").ok_or(SolReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let x1 = x1_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidData {
                    name: "line x1".to_string(),
                    value: format!("{:?}", x1_amf),
                })?;

            let y1_amf = line.get("1").ok_or(SolReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let y1 = y1_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidData {
                    name: "line y1".to_string(),
                    value: format!("{:?}", y1_amf),
                })?;

            let x2_amf = line.get("2").ok_or(SolReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let x2 = x2_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidData {
                    name: "line x2".to_string(),
                    value: format!("{:?}", x2_amf),
                })?;

            let y2_amf = line.get("3").ok_or(SolReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let y2 = y2_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidData {
                    name: "line y2".to_string(),
                    value: format!("{:?}", y2_amf),
                })?;

            let ext_amf = line.get("4").ok_or(SolReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let ext = ext_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidData {
                    name: "line extension".to_string(),
                    value: format!("{:?}", ext_amf),
                })?;

            let left_extension = ext == 1.0 || ext == 3.0;
            let right_extension = ext == 2.0 || ext == 3.0;

            let flipped_amf = line.get("5").ok_or(SolReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let flipped = flipped_amf
                .clone()
                .get_boolean()
                .or_else(|| flipped_amf.clone().get_number().map(|num| num == 1.0))
                .ok_or(SolReadError::InvalidData {
                    name: "line flipped".to_string(),
                    value: format!("{:?}", flipped_amf),
                })?;

            let id_amf = line.get("8").ok_or(SolReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let id_float = id_amf
                .clone()
                .get_number()
                .ok_or(SolReadError::InvalidData {
                    name: "line id".to_string(),
                    value: format!("{:?}", id_amf),
                })?;

            let unsafe_id =
                if id_float.is_finite() && id_float >= 0.0 && id_float <= f64::from(u32::MAX) {
                    Some(id_float as u32)
                } else {
                    None
                };

            let id = match unsafe_id {
                Some(val) => val,
                None => {
                    return Err(SolReadError::InvalidData {
                        name: "line id".to_string(),
                        value: id_float.to_string(),
                    });
                }
            };

            let line_type_amf = line.get("9").ok_or(SolReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let line_type_numeric =
                line_type_amf
                    .clone()
                    .get_number()
                    .ok_or(SolReadError::InvalidData {
                        name: "line type".to_string(),
                        value: format!("{:?}", line_type_amf),
                    })?;

            let line_type = match line_type_numeric {
                0.0 => LineType::Standard,
                1.0 => LineType::Acceleration,
                2.0 => LineType::Scenery,
                other => {
                    return Err(SolReadError::InvalidData {
                        name: "line type".to_string(),
                        value: other.to_string(),
                    });
                }
            };

            let endpoints = (Vec2::new(x1, y1), Vec2::new(x2, y2));

            match line_type {
                LineType::Standard => {
                    track_builder.line_group().add_standard_line(
                        id,
                        endpoints,
                        flipped,
                        left_extension,
                        right_extension,
                    );
                }
                LineType::Acceleration => {
                    track_builder.line_group().add_acceleration_line(
                        id,
                        endpoints,
                        flipped,
                        left_extension,
                        right_extension,
                    );
                }
                LineType::Scenery => {
                    track_builder.line_group().add_scenery_line(id, endpoints);
                }
            }
        }
    }

    Ok(track_builder.build()?)
}
