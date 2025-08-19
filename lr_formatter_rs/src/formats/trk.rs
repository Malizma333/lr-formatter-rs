//! Format used by [Line Rider: Advanced](https://github.com/jealouscloud/linerider-advanced) and its forks

mod error;
mod reader;

pub use error::TrkReadError;
pub use reader::read;

// These string literals are implementation-specific, do not modify
const FEATURE_RED_MULTIPLIER: &str = "REDMULTIPLIER";
const FEATURE_SCENERY_WIDTH: &str = "SCENERYWIDTH";
const FEATURE_SONG_INFO: &str = "SONGINFO";
const FEATURE_IGNORABLE_TRIGGER: &str = "IGNORABLE_TRIGGER";
const FEATURE_6_1: &str = "6.1";
const FEATURE_ZERO_START: &str = "ZEROSTART";
const FEATURE_REMOUNT: &str = "REMOUNT";
const FEATURE_FRICTIONLESS: &str = "FRICTIONLESS";
const FEATURE_START_ZOOM: &str = "STARTZOOM";
const FEATURE_X_GRAVITY: &str = "XGRAVITY";
const FEATURE_Y_GRAVITY: &str = "YGRAVITY";
const FEATURE_GRAVITY_WELL_SIZE: &str = "GRAVITYWELLSIZE";
const FEATURE_BACKGROUND_COLOR_R: &str = "BGCOLORR";
const FEATURE_BACKGROUND_COLOR_G: &str = "BGCOLORG";
const FEATURE_BACKGROUND_COLOR_B: &str = "BGCOLORB";
const FEATURE_LINE_COLOR_R: &str = "LINECOLORR";
const FEATURE_LINE_COLOR_G: &str = "LINECOLORG";
const FEATURE_LINE_COLOR_B: &str = "LINECOLORB";
const FEATURE_TRIGGERS: &str = "TRIGGERS";
