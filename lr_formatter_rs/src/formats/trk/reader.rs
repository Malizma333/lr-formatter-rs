use std::{
    collections::HashSet,
    io::{Cursor, Read, Seek, SeekFrom},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
    formats::trk::{
        FEATURE_BACKGROUND_COLOR_B, FEATURE_BACKGROUND_COLOR_G, FEATURE_BACKGROUND_COLOR_R,
        FEATURE_FRICTIONLESS, FEATURE_GRAVITY_WELL_SIZE, FEATURE_LINE_COLOR_B,
        FEATURE_LINE_COLOR_G, FEATURE_REMOUNT, FEATURE_START_ZOOM, FEATURE_TRIGGERS,
        FEATURE_X_GRAVITY, FEATURE_Y_GRAVITY, FEATURE_ZERO_START, TrkReadError,
    },
    track::{
        BackgroundColorEvent, CameraZoomEvent, FrameBoundsTrigger, GridVersion, LineColorEvent,
        LineHitTrigger, LineType, RGBColor, Track, TrackBuilder, Vec2,
    },
    util::{StringLength, bytes_to_hex_string, parse_string},
};

use super::{
    FEATURE_6_1, FEATURE_IGNORABLE_TRIGGER, FEATURE_LINE_COLOR_R, FEATURE_RED_MULTIPLIER,
    FEATURE_SCENERY_WIDTH, FEATURE_SONG_INFO,
};

pub fn read(data: Vec<u8>) -> Result<Track, TrkReadError> {
    let track_builder = &mut TrackBuilder::default();

    let mut cursor = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 4];
    cursor.read_exact(&mut magic_number)?;

    if magic_number != [b'T', b'R', b'K', 0xF2] {
        return Err(TrkReadError::InvalidData {
            name: "magic number".to_string(),
            value: bytes_to_hex_string(&magic_number),
        });
    }

    // Version
    let version = cursor.read_u8()?;

    if version > 1 {
        return Err(TrkReadError::InvalidData {
            name: "version".to_string(),
            value: version.to_string(),
        });
    }

    let feature_string = parse_string::<LittleEndian>(&mut cursor, StringLength::U16)?;
    let mut included_features: HashSet<&str> = Default::default();

    for feature in feature_string.split(';').filter(|s| !s.is_empty()) {
        included_features.insert(feature);
        // TODO: Attach warning if feature not accounted for
    }

    let grid_version = if included_features.contains(FEATURE_6_1) {
        GridVersion::V6_1
    } else {
        GridVersion::V6_2
    };
    track_builder.metadata().grid_version(grid_version);

    if included_features.contains(FEATURE_SONG_INFO) {
        let mut song_string_length = 0;
        let mut bit_shift = 0;

        loop {
            // Read 7BitEncodedInt song string length
            let byte = cursor.read_u8()?;
            song_string_length |= usize::from(byte & 0x7F) << bit_shift;

            if byte & 0x80 == 0 {
                break;
            }

            bit_shift += 7;
        }

        let song_string =
            parse_string::<LittleEndian>(&mut cursor, StringLength::Fixed(song_string_length))?;
        let song_data: Vec<&str> = song_string
            .split("\r\n")
            .filter(|s| !s.is_empty())
            .collect();

        if song_data.len() != 2 {
            return Err(TrkReadError::InvalidData {
                name: "song data".to_string(),
                value: song_data.join(","),
            });
        }

        let name = song_data[0];
        let seconds_offset = song_data[1].parse::<f64>()?;
        track_builder
            .metadata()
            .audio_filename(name)
            .audio_offset_until_start(-seconds_offset);
    }

    let start_pos_x = cursor.read_f64::<LittleEndian>()?;
    let start_pos_y = cursor.read_f64::<LittleEndian>()?;
    track_builder
        .metadata()
        .start_position(Vec2::new(start_pos_x, start_pos_y));

    let line_count = cursor.read_u32::<LittleEndian>()?;

    let mut max_id = 0;

    for _ in 0..line_count {
        let mut line_id: u32 = 0;
        let flags = cursor.read_u8()?;

        let line_type = match flags & 0x1F {
            1 => LineType::Standard,
            2 => LineType::Acceleration,
            0 => LineType::Scenery,
            other => {
                return Err(TrkReadError::InvalidData {
                    name: "line type".to_string(),
                    value: other.to_string(),
                });
            }
        };

        let line_inv = (flags >> 7) != 0;
        let line_ext = (flags >> 5) & 0x3;

        let mut line_multiplier = 1.0;
        let mut line_scenery_width = 1.0;

        if line_type == LineType::Acceleration && included_features.contains(FEATURE_RED_MULTIPLIER)
        {
            line_multiplier = f64::from(cursor.read_u8()?);
        }

        if line_type == LineType::Scenery {
            if included_features.contains(FEATURE_SCENERY_WIDTH) {
                line_scenery_width = f64::from(cursor.read_u8()?) / 10.0;
            }
        } else {
            line_id = cursor.read_u32::<LittleEndian>()?;
            max_id = max_id.max(line_id);

            if line_ext != 0 {
                _ = cursor.read_i32::<LittleEndian>()?; // Prev line id or -1
                _ = cursor.read_i32::<LittleEndian>()?; // Next line id or -1
            }

            if included_features.contains(FEATURE_IGNORABLE_TRIGGER) {
                let has_zoom_trigger = cursor.read_u8()?;
                if has_zoom_trigger == 1 {
                    let target_zoom = f64::from(cursor.read_f32::<LittleEndian>()?);
                    let length = u32::try_from(cursor.read_i16::<LittleEndian>()?)?;
                    let zoom_event = CameraZoomEvent::new(target_zoom);
                    let line_hit = LineHitTrigger::new(line_id, length);
                    track_builder
                        .legacy_camera_zoom_group()
                        .add_trigger()
                        .trigger(line_hit)
                        .event(zoom_event);
                }
            }
        }

        let line_x1 = cursor.read_f64::<LittleEndian>()?;
        let line_y1 = cursor.read_f64::<LittleEndian>()?;
        let line_x2 = cursor.read_f64::<LittleEndian>()?;
        let line_y2 = cursor.read_f64::<LittleEndian>()?;
        let endpoints = (Vec2::new(line_x1, line_y1), Vec2::new(line_x2, line_y2));
        let left_ext = line_ext & 0x1 != 0;
        let right_ext = line_ext & 0x2 != 0;

        match line_type {
            LineType::Standard => {
                track_builder
                    .line_group()
                    .add_standard_line(line_id, endpoints, line_inv, left_ext, right_ext);
            }
            LineType::Acceleration => {
                track_builder
                    .line_group()
                    .add_acceleration_line(line_id, endpoints, line_inv, left_ext, right_ext)
                    .multiplier(line_multiplier);
            }
            LineType::Scenery => {
                track_builder
                    .line_group()
                    .add_scenery_line(line_id, endpoints)
                    .width(line_scenery_width);
            }
        }
    }

    for line in track_builder.line_group().get_scenery_lines() {
        max_id += 1;
        line.id(max_id);
    }

    if included_features.contains(FEATURE_FRICTIONLESS) {
        track_builder.metadata().zero_friction_riders(true);
    }

    if included_features.contains(FEATURE_REMOUNT) {
        track_builder.metadata().remount_riders(true);
        track_builder.metadata().lra_remount(true);
    }

    if included_features.contains(FEATURE_ZERO_START) {
        track_builder.metadata().zero_velocity_start_riders(true);
    }

    let current = cursor.stream_position()?;
    let end = cursor.seek(SeekFrom::End(0))?;
    cursor.seek(SeekFrom::Start(current))?;

    if current == end {
        return Ok(track_builder.build()?);
    }

    // Metadata section

    let mut meta_magic_number = [0u8; 4];
    cursor.read_exact(&mut meta_magic_number)?;

    if &meta_magic_number != b"META" {
        return Err(TrkReadError::InvalidData {
            name: "metadata magic number".to_string(),
            value: bytes_to_hex_string(&meta_magic_number),
        });
    }

    let num_entries = cursor.read_u16::<LittleEndian>()?;

    let mut start_zoom = 4.0;
    let mut start_gravity_x = 0.0;
    let mut start_gravity_y = 1.0;
    let mut gravity_well_size = 10.0;
    let mut start_line_color_red = 0;
    let mut start_line_color_green = 0;
    let mut start_line_color_blue = 0;
    let mut start_bg_color_red = 244;
    let mut start_bg_color_green = 245;
    let mut start_bg_color_blue = 249;

    for _ in 0..num_entries {
        let meta_string = parse_string::<LittleEndian>(&mut cursor, StringLength::U16)?;
        let key_value_pair: Vec<&str> = meta_string.split("=").filter(|s| !s.is_empty()).collect();

        if key_value_pair.len() != 2 {
            return Err(TrkReadError::InvalidData {
                name: "metadata key value pair".to_string(),
                value: key_value_pair.join(","),
            });
        }

        let key = key_value_pair[0];
        let value = key_value_pair[1];

        match key {
            FEATURE_START_ZOOM => {
                start_zoom = f64::from(value.parse::<f32>()?);
            }
            FEATURE_X_GRAVITY => {
                start_gravity_x = f64::from(value.parse::<f32>()?);
            }
            FEATURE_Y_GRAVITY => {
                start_gravity_y = f64::from(value.parse::<f32>()?);
            }
            FEATURE_GRAVITY_WELL_SIZE => {
                gravity_well_size = value.parse::<f64>()?;
            }
            FEATURE_BACKGROUND_COLOR_R => {
                start_bg_color_red = u8::try_from(value.parse::<i32>()?)?;
            }
            FEATURE_BACKGROUND_COLOR_G => {
                start_bg_color_green = u8::try_from(value.parse::<i32>()?)?;
            }
            FEATURE_BACKGROUND_COLOR_B => {
                start_bg_color_blue = u8::try_from(value.parse::<i32>()?)?;
            }
            FEATURE_LINE_COLOR_R => {
                start_line_color_red = u8::try_from(value.parse::<i32>()?)?;
            }
            FEATURE_LINE_COLOR_G => {
                start_line_color_green = u8::try_from(value.parse::<i32>()?)?;
            }
            FEATURE_LINE_COLOR_B => {
                start_line_color_blue = u8::try_from(value.parse::<i32>()?)?;
            }
            FEATURE_TRIGGERS => {
                for (i, trigger) in value.split('&').filter(|s| !s.is_empty()).enumerate() {
                    let values: Vec<&str> = trigger.split(':').filter(|s| !s.is_empty()).collect();

                    if values.is_empty() {
                        return Err(TrkReadError::InvalidData {
                            name: "size of trigger data".to_string(),
                            value: "0".to_string(),
                        });
                    }

                    match values[0] {
                        "0" => {
                            // Zoom
                            let target_zoom = f64::from(values[1].parse::<f32>()?);
                            let start_frame = u32::try_from(values[2].parse::<i32>()?)?;
                            let end_frame = u32::try_from(values[3].parse::<i32>()?)?;
                            let zoom_event = CameraZoomEvent::new(target_zoom);
                            let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                            track_builder
                                .camera_zoom_group()
                                .add_trigger()
                                .trigger(frame_bounds)
                                .event(zoom_event);
                        }
                        "1" => {
                            // Background Color
                            let red = u8::try_from(values[1].parse::<i32>()?)?;
                            let green = u8::try_from(values[2].parse::<i32>()?)?;
                            let blue = u8::try_from(values[3].parse::<i32>()?)?;
                            let start_frame = u32::try_from(values[4].parse::<i32>()?)?;
                            let end_frame = u32::try_from(values[5].parse::<i32>()?)?;
                            let bg_color_event =
                                BackgroundColorEvent::new(RGBColor::new(red, green, blue));
                            let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                            track_builder
                                .background_color_group()
                                .add_trigger()
                                .trigger(frame_bounds)
                                .event(bg_color_event);
                        }
                        "2" => {
                            // Line Color
                            let red = u8::try_from(values[1].parse::<i32>()?)?;
                            let green = u8::try_from(values[2].parse::<i32>()?)?;
                            let blue = u8::try_from(values[3].parse::<i32>()?)?;
                            let start_frame = u32::try_from(values[4].parse::<i32>()?)?;
                            let end_frame = u32::try_from(values[5].parse::<i32>()?)?;
                            let line_color_event =
                                LineColorEvent::new(RGBColor::new(red, green, blue));
                            let frame_bounds = FrameBoundsTrigger::new(start_frame, end_frame);
                            track_builder
                                .line_color_group()
                                .add_trigger()
                                .trigger(frame_bounds)
                                .event(line_color_event);
                        }
                        other => {
                            return Err(TrkReadError::InvalidData {
                                name: format!("triggers {} type", i),
                                value: other.to_string(),
                            });
                        }
                    }
                }
            }
            other => {
                return Err(TrkReadError::InvalidData {
                    name: "metadata key".to_string(),
                    value: other.to_string(),
                });
            }
        }
    }

    track_builder.metadata().start_zoom(start_zoom);
    track_builder
        .metadata()
        .start_gravity(Vec2::new(start_gravity_x, start_gravity_y));
    track_builder
        .metadata()
        .gravity_well_size(gravity_well_size);
    track_builder
        .metadata()
        .start_background_color(RGBColor::new(
            start_bg_color_red,
            start_bg_color_green,
            start_bg_color_blue,
        ));
    track_builder.metadata().start_line_color(RGBColor::new(
        start_line_color_red,
        start_line_color_green,
        start_line_color_blue,
    ));

    Ok(track_builder.build()?)
}
