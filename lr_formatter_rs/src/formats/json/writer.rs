use crate::{
    formats::json::{
        FaultyU32, JsonLayer, JsonLine, JsonReadError, JsonRider, JsonTrack, LAYER_TYPE_FOLDER,
        LAYER_TYPE_LAYER, V2,
    },
    track::{GridVersion, Track},
};

pub fn write(track: &Track) -> Result<Vec<u8>, JsonReadError> {
    let version = match track.metadata().grid_version() {
        GridVersion::V6_0 => String::from("6.0"),
        GridVersion::V6_1 => String::from("6.1"),
        GridVersion::V6_2 => String::from("6.2"),
    };

    let mut lines = Vec::<JsonLine>::new();
    let mut layers = Vec::<JsonLayer>::new();
    let mut riders = Vec::<JsonRider>::new();

    for line in track.line_group().standard_lines() {
        lines.push(JsonLine {
            id: line.id(),
            line_type: 0,
            x1: line.x1(),
            y1: line.y1(),
            x2: line.x2(),
            y2: line.y2(),
            flipped: Some(line.flipped()),
            left_ext: Some(line.left_extension()),
            right_ext: Some(line.right_extension()),
            extended: None,
            multiplier: None,
            width: None,
        });
    }

    for line in track.line_group().acceleration_lines() {
        lines.push(JsonLine {
            id: line.id(),
            line_type: 1,
            x1: line.x1(),
            y1: line.y1(),
            x2: line.x2(),
            y2: line.y2(),
            flipped: Some(line.flipped()),
            left_ext: Some(line.left_extension()),
            right_ext: Some(line.right_extension()),
            extended: None,
            multiplier: line.multiplier(),
            width: None,
        });
    }

    for line in track.line_group().scenery_lines() {
        lines.push(JsonLine {
            id: line.id(),
            line_type: 2,
            x1: line.x1(),
            y1: line.y1(),
            x2: line.x2(),
            y2: line.y2(),
            flipped: None,
            left_ext: None,
            right_ext: None,
            extended: None,
            multiplier: None,
            width: line.width(),
        });
    }

    if let Some(layer_group) = track.layer_group() {
        let mut tupled_layers = vec![];
        for layer in layer_group.layers() {
            let json_folder_id = if let Some(valid_id) = layer.folder_id().unwrap_or(None) {
                Some(FaultyU32::Valid(valid_id))
            } else {
                Some(FaultyU32::Invalid(-1))
            };

            tupled_layers.push((
                layer.index(),
                JsonLayer {
                    id: layer.id(),
                    layer_type: Some(LAYER_TYPE_LAYER),
                    name: layer.name().unwrap_or("".to_string()),
                    visible: layer.visible().unwrap_or(true),
                    editable: layer.editable(),
                    folder_id: json_folder_id,
                    size: None,
                },
            ));
        }
        if let Some(layer_folders) = layer_group.layer_folders() {
            for layer_folder in layer_folders {
                tupled_layers.push((
                    layer_folder.index(),
                    JsonLayer {
                        id: layer_folder.id(),
                        layer_type: Some(LAYER_TYPE_FOLDER),
                        name: layer_folder.name().unwrap_or("".to_string()),
                        visible: layer_folder.visible().unwrap_or(true),
                        editable: layer_folder.editable(),
                        folder_id: None,
                        size: layer_folder.size(),
                    },
                ));
            }
        }

        tupled_layers.sort_by_key(|layer| layer.0);

        for layer in tupled_layers {
            layers.push(layer.1);
        }
    } else {
        // Default layer
        layers.push(JsonLayer {
            id: 0,
            layer_type: Some(LAYER_TYPE_LAYER),
            name: "Base Layer".to_string(),
            visible: true,
            editable: Some(true),
            folder_id: None,
            size: None,
        });
    }

    if let Some(rider_group) = track.rider_group() {
        for rider in rider_group.riders() {
            let start_position = V2 {
                x: rider.start_position().x(),
                y: rider.start_position().y(),
            };

            let start_velocity = if let Some(start_vel) = rider.start_velocity() {
                V2 {
                    x: start_vel.x(),
                    y: start_vel.y(),
                }
            } else {
                V2 { x: 0.4, y: 0.0 }
            };

            riders.push(JsonRider {
                start_pos: start_position,
                start_vel: start_velocity,
                angle: rider.start_angle(),
                remountable: rider.can_remount(),
            });
        }
    } else {
        riders.push(JsonRider {
            start_pos: V2 { x: 0.0, y: 0.0 },
            start_vel: V2 { x: 0.4, y: 0.0 },
            angle: Some(0.0),
            remountable: Some(true),
        });
    }

    let start_pos = if let Some(start_position) = track.metadata().start_position() {
        V2 {
            x: start_position.x(),
            y: start_position.y(),
        }
    } else {
        V2 { x: 0.0, y: 0.0 }
    };

    let label = track.metadata().title().clone().unwrap_or("".to_string());
    let creator = Some(track.metadata().artist().clone().unwrap_or("".to_string()));
    let description = Some(
        track
            .metadata()
            .description()
            .clone()
            .unwrap_or("".to_string()),
    );
    let script = Some(track.metadata().script().clone().unwrap_or("".to_string()));
    let duration = Some(track.metadata().duration().unwrap_or(1200));

    let track = JsonTrack {
        label,
        version,
        start_pos,
        lines: Some(lines),
        creator,
        description,
        duration,
        script,
        layers: Some(layers),
        riders: Some(riders),
        // Deprecated LRA Json format
        line_array: None,
        time_based_triggers: None,
        start_zoom: None,
        zero_start: None,
        line_based_triggers: None,
        line_color_blue: None,
        line_color_green: None,
        line_color_red: None,
        background_color_blue: None,
        background_color_green: None,
        background_color_red: None,
        gravity_well_size: None,
        x_gravity: None,
        y_gravity: None,
    };

    let track_string = serde_json::to_string(&track)?;

    Ok(track_string.into_bytes())
}
