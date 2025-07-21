//! Format used by the updated web version of Line Rider, [linerider.com](https://www.linerider.com/)

mod error;
mod reader;
mod serde_boolean;
mod serde_line_array;
mod writer;

pub use error::{JsonReadError, JsonWriteError};
pub use reader::read;
pub use writer::write;

use serde::{Deserialize, Serialize};
use serde_boolean::option_bool_from_any;

// LRA line array types:
// [type: 0, id: int, x1: double, y1: double, x2: double, y2: double, extended: u8, flipped: bool]
// [type: 1, id: int, x1: double, y1: double, x2: double, y2: double, extended: u8, flipped: bool, _?: -1, _?: -1, multiplier?: int]
// [type: 2, id: int, x1: double, y1: double, x2: double, y2: double]
// Extended bitflags 0b000000ba
// a: 1 if starting/left extension
// b: 1 if ending/right extension
#[derive(Debug)]
enum LRAJsonArrayLine {
    Standard(u32, f64, f64, f64, f64, u8, bool),
    Acceleration(u32, f64, f64, f64, f64, u8, bool, (), (), u32),
    Scenery(u32, f64, f64, f64, f64),
}

#[derive(Serialize, Deserialize, Debug)]
struct LRAJsonLegacyZoomTrigger {
    #[serde(rename = "ID")]
    id: u32,
    zoom: bool,  // whether zoom trigger enabled
    target: f32, // target to zoom to
    frames: u32, // duration of zoom
}

// Faulty U32's are used here whenever properties are -999, which
// represents undefined/unused in the LRA json trigger format
#[derive(Serialize, Deserialize, Debug)]
struct LRAJsonTrigger {
    #[serde(rename = "triggerType")]
    trigger_type: u8,
    start: u32,
    end: u32,
    #[serde(rename = "zoomTarget")]
    zoom_target: f32,
    #[serde(rename = "backgroundRed")]
    background_red: Option<FaultyU32>,
    #[serde(rename = "backgroundGreen")]
    background_green: Option<FaultyU32>,
    #[serde(rename = "backgroundBlue")]
    background_blue: Option<FaultyU32>,
    #[serde(rename = "lineRed")]
    line_red: Option<FaultyU32>,
    #[serde(rename = "lineGreen")]
    line_green: Option<FaultyU32>,
    #[serde(rename = "lineBlue")]
    line_blue: Option<FaultyU32>,
}

// A u32 value that can take the range of a normal u32, or negative for invalid (for parsing some json fields)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum FaultyU32 {
    Valid(u32),
    Invalid(i32),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct V2 {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonLine {
    id: u32,
    #[serde(rename = "type")]
    line_type: u8,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "option_bool_from_any"
    )]
    flipped: Option<bool>,
    #[serde(
        default,
        rename = "leftExtended",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "option_bool_from_any"
    )]
    left_ext: Option<bool>,
    #[serde(
        default,
        rename = "rightExtended",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "option_bool_from_any"
    )]
    right_ext: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extended: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
}

const LAYER_TYPE_LAYER: u8 = 0;
const LAYER_TYPE_FOLDER: u8 = 1;

#[derive(Serialize, Deserialize, Debug)]
struct JsonLayer {
    id: u32,
    #[serde(rename = "type")]
    layer_type: Option<u8>,
    name: String,
    visible: bool,
    editable: Option<bool>,
    #[serde(rename = "folderId", skip_serializing_if = "Option::is_none")]
    folder_id: Option<FaultyU32>, // -1 if not a folder id
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRider {
    #[serde(rename = "startPosition")]
    start_pos: V2,
    #[serde(rename = "startVelocity")]
    start_vel: V2,
    #[serde(rename = "startAngle", skip_serializing_if = "Option::is_none")]
    angle: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "option_bool_from_any"
    )]
    remountable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonTrack {
    label: String,
    creator: Option<String>,
    description: Option<String>,
    duration: Option<u32>,
    version: String,
    lines: Option<Vec<JsonLine>>,
    layers: Option<Vec<JsonLayer>>,
    riders: Option<Vec<JsonRider>>,
    script: Option<String>,
    #[serde(rename = "startPosition")]
    start_pos: V2,
    #[serde(rename = "linesArray", skip_serializing_if = "Option::is_none")]
    line_array: Option<Vec<LRAJsonArrayLine>>,
    #[serde(rename = "startZoom", skip_serializing_if = "Option::is_none")]
    start_zoom: Option<f32>,
    #[serde(rename = "zeroStart", skip_serializing_if = "Option::is_none")]
    zero_start: Option<bool>,
    #[serde(rename = "triggers", skip_serializing_if = "Option::is_none")]
    line_based_triggers: Option<Vec<LRAJsonLegacyZoomTrigger>>,
    #[serde(rename = "gameTriggers", skip_serializing_if = "Option::is_none")]
    time_based_triggers: Option<Vec<LRAJsonTrigger>>,
    #[serde(rename = "xGravity", skip_serializing_if = "Option::is_none")]
    x_gravity: Option<f32>,
    #[serde(rename = "yGravity", skip_serializing_if = "Option::is_none")]
    y_gravity: Option<f32>,
    #[serde(rename = "gravityWellSize", skip_serializing_if = "Option::is_none")]
    gravity_well_size: Option<f64>,
    #[serde(rename = "bgR", skip_serializing_if = "Option::is_none")]
    background_color_red: Option<u32>,
    #[serde(rename = "bgG", skip_serializing_if = "Option::is_none")]
    background_color_green: Option<u32>,
    #[serde(rename = "bgB", skip_serializing_if = "Option::is_none")]
    background_color_blue: Option<u32>,
    #[serde(rename = "lineR", skip_serializing_if = "Option::is_none")]
    line_color_red: Option<u32>,
    #[serde(rename = "lineG", skip_serializing_if = "Option::is_none")]
    line_color_green: Option<u32>,
    #[serde(rename = "lineB", skip_serializing_if = "Option::is_none")]
    line_color_blue: Option<u32>,
}
