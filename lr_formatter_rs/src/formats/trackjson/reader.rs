use crate::{
    formats::{
        TrackReadError,
        trackjson::{FaultyU32, LRAJsonArrayLine},
    },
    track::{
        GridVersion, LineType, RGBColor, Track, TrackBuilder, TrackFeature, Vec2,
        layer::layer_group::LayerFeature, line::line_group::LineFeature,
        rider::rider_group::RiderFeature,
    },
};

use super::JsonTrack;

pub fn read(data: Vec<u8>) -> Result<Track, TrackReadError> {
    let track_builder = &mut TrackBuilder::new();
    let json_string = String::from_utf8(data.to_vec())?;
    let json_track: JsonTrack =
        serde_json::from_str(&json_string).map_err(|err| TrackReadError::Other {
            message: format!("Failed to deserialize json track: {}", err),
        })?;

    let grid_version = match json_track.version.as_str() {
        "6.0" => GridVersion::V6_0,
        "6.1" => GridVersion::V6_1,
        "6.2" => GridVersion::V6_2,
        other => {
            return Err(TrackReadError::InvalidData {
                name: "grid version".to_string(),
                value: other.to_string(),
            });
        }
    };

    track_builder.metadata().grid_version(grid_version);

    if let Some(line_list) = json_track.lines {
        for line in line_list {
            let line_type = match line.line_type {
                0 => LineType::Standard,
                1 => LineType::Acceleration,
                2 => LineType::Scenery,
                other => {
                    return Err(TrackReadError::InvalidData {
                        name: "line type".to_string(),
                        value: other.to_string(),
                    });
                }
            };

            let endpoints = (
                Vec2 {
                    x: line.x1,
                    y: line.y1,
                },
                Vec2 {
                    x: line.x2,
                    y: line.y2,
                },
            );

            let (left_extension, right_extension) = if line_type == LineType::Scenery {
                (false, false)
            } else if let Some(ext) = line.extended {
                (ext & 1 != 0, ext & 2 != 0)
            } else if let (Some(left_ext), Some(right_ext)) = (line.left_ext, line.right_ext) {
                (left_ext, right_ext)
            } else {
                (false, false)
            };

            let flipped = line.flipped.unwrap_or(false);

            match line_type {
                LineType::Standard => {
                    track_builder.line_group().add_standard_line(
                        line.id,
                        endpoints,
                        flipped,
                        left_extension,
                        right_extension,
                    )?;
                }
                LineType::Acceleration => {
                    if !line.multiplier.is_none_or(|x| x == 1.0) {
                        track_builder
                            .line_group()
                            .enable_feature(LineFeature::AccelerationMultiplier);
                    }
                    track_builder
                        .line_group()
                        .add_acceleration_line(
                            line.id,
                            endpoints,
                            flipped,
                            left_extension,
                            right_extension,
                        )?
                        .multiplier(line.multiplier);
                }
                LineType::Scenery => {
                    if !line.width.is_none_or(|x| x == 1.0) {
                        track_builder
                            .line_group()
                            .enable_feature(LineFeature::SceneryWidth);
                    }
                    track_builder
                        .line_group()
                        .add_scenery_line(line.id, endpoints)?
                        .width(line.width);
                }
            }
        }
    }

    // Legacy line array
    if let Some(line_list) = json_track.line_array {
        for line in line_list {
            match line {
                LRAJsonArrayLine::Standard(id, x1, y1, x2, y2, extended, flipped) => {
                    let endpoints = (Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 });
                    let left_extension = extended & 0x1 != 0;
                    let right_extension = extended & 0x2 != 0;
                    track_builder.line_group().add_standard_line(
                        id,
                        endpoints,
                        flipped,
                        left_extension,
                        right_extension,
                    )?;
                }
                LRAJsonArrayLine::Acceleration(
                    id,
                    x1,
                    y1,
                    x2,
                    y2,
                    extended,
                    flipped,
                    _,
                    _,
                    multiplier,
                ) => {
                    let endpoints = (Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 });
                    let left_extension = extended & 0x1 != 0;
                    let right_extension = extended & 0x2 != 0;
                    track_builder
                        .line_group()
                        .add_acceleration_line(
                            id,
                            endpoints,
                            flipped,
                            left_extension,
                            right_extension,
                        )?
                        .multiplier(Some(multiplier as f64));
                }
                LRAJsonArrayLine::Scenery(id, x1, y1, x2, y2) => {
                    let endpoints = (Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 });
                    track_builder.line_group().add_scenery_line(id, endpoints)?;
                }
            }
        }
    }

    if let Some(layers) = json_track.layers {
        track_builder.enable_feature(TrackFeature::Layers);
        track_builder
            .layer_group()?
            .enable_feature(LayerFeature::Name)
            .enable_feature(LayerFeature::Visible);
        for (index, layer) in layers.iter().enumerate() {
            let mut layer_editable = true;
            let mut layer_is_folder = false;
            let mut layer_folder_id = None;
            let mut layer_folder_size = 0;

            if let Some(editable) = layer.editable {
                track_builder
                    .layer_group()?
                    .enable_feature(LayerFeature::Editable);
                layer_editable = editable;
            }

            if let Some(folder_id) = &layer.folder_id {
                track_builder
                    .layer_group()?
                    .enable_feature(LayerFeature::Folders);
                if let FaultyU32::Valid(valid_folder_id) = folder_id {
                    layer_folder_id = Some(*valid_folder_id);
                }
            }

            if let Some(size) = layer.size {
                track_builder
                    .layer_group()?
                    .enable_feature(LayerFeature::Folders);
                layer_is_folder = false;
                layer_folder_size = size;
            }

            if layer_is_folder {
                track_builder
                    .layer_group()?
                    .add_layer(layer.id, index)?
                    .index(index)
                    .name(Some(layer.name.to_string()))
                    .visible(Some(layer.visible))
                    .editable(Some(layer_editable))
                    .folder_id(Some(layer_folder_id));
            } else {
                track_builder
                    .layer_group()?
                    .add_layer_folder(layer.id, index)?
                    .index(index)
                    .name(Some(layer.name.to_string()))
                    .visible(Some(layer.visible))
                    .editable(Some(layer_editable))
                    .size(Some(layer_folder_size));
            }
        }
    }

    if let Some(riders) = json_track.riders {
        track_builder.enable_feature(TrackFeature::Riders);
        track_builder
            .rider_group()?
            .enable_feature(RiderFeature::Remount)
            .enable_feature(RiderFeature::StartAngle);
        for rider in riders.iter() {
            let start_position = Vec2 {
                x: rider.start_pos.x,
                y: rider.start_pos.y,
            };
            let start_velocity = Vec2 {
                x: rider.start_vel.x,
                y: rider.start_vel.y,
            };

            let mut start_angle = 0.0;
            let mut can_remount = false;

            if let Some(angle) = rider.angle {
                track_builder
                    .rider_group()?
                    .enable_feature(RiderFeature::StartAngle);
                start_angle = angle;
            }

            if let Some(remount) = rider.remountable {
                track_builder
                    .rider_group()?
                    .enable_feature(RiderFeature::Remount);
                can_remount = remount;
            }

            track_builder
                .rider_group()?
                .add_rider()
                .start_position(start_position)
                .start_velocity(Some(start_velocity))
                .start_angle(Some(start_angle))
                .can_remount(Some(can_remount));
        }
    }

    track_builder.metadata().start_position(Vec2 {
        x: json_track.start_pos.x,
        y: json_track.start_pos.y,
    });

    track_builder.metadata().title(json_track.label);

    if let Some(creator) = json_track.creator {
        track_builder.metadata().artist(creator);
    }

    if let Some(description) = json_track.description {
        track_builder.metadata().description(description);
    }

    if let Some(duration) = json_track.duration {
        track_builder.metadata().duration(duration);
    }

    if let Some(script) = json_track.script {
        track_builder.metadata().script(script);
    }

    if let Some(zero_start) = json_track.zero_start {
        track_builder.metadata().zero_start(zero_start);
    }

    if let Some(gravity_well_size) = json_track.gravity_well_size {
        track_builder
            .metadata()
            .gravity_well_size(gravity_well_size);
    }

    let mut start_gravity = Vec2 { x: 0.0, y: 1.0 };

    if let Some(x_gravity) = json_track.x_gravity {
        start_gravity.x = x_gravity as f64;
    }

    if let Some(y_gravity) = json_track.y_gravity {
        start_gravity.y = y_gravity as f64;
    }

    track_builder.metadata().start_gravity(start_gravity);

    if let Some(start_zoom) = json_track.start_zoom {
        track_builder.metadata().start_zoom(start_zoom as f64);
    }

    let mut start_line_color = RGBColor {
        red: 0,
        green: 0,
        blue: 0,
    };

    if let Some(initial_line_red) = json_track.line_color_red {
        start_line_color.red = initial_line_red as u8;
    }

    if let Some(initial_line_green) = json_track.line_color_green {
        start_line_color.green = initial_line_green as u8;
    }

    if let Some(initial_line_blue) = json_track.line_color_blue {
        start_line_color.blue = initial_line_blue as u8;
    }

    track_builder.metadata().start_line_color(start_line_color);

    let mut start_background_color = RGBColor {
        red: 244,
        green: 245,
        blue: 249,
    };

    if let Some(initial_background_red) = json_track.background_color_red {
        start_background_color.red = initial_background_red as u8;
    }

    if let Some(initial_background_green) = json_track.background_color_green {
        start_background_color.green = initial_background_green as u8;
    }

    if let Some(initial_background_blue) = json_track.background_color_blue {
        start_background_color.blue = initial_background_blue as u8;
    }

    track_builder
        .metadata()
        .start_background_color(start_background_color);

    // TODO: These fields need parsing into the internal format still
    // line_based_triggers, time_based_triggers
    Ok(track_builder.build()?)
}
