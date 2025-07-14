use derive_builder::Builder;
use getset::{CloneGetters, Getters};

use crate::track::{GridVersion, RGBColor, Vec2};

#[derive(Getters, CloneGetters, Debug, Builder)]
pub struct Metadata {
    // Shared Properties
    #[getset(get_clone = "pub")]
    grid_version: GridVersion,
    #[builder(setter(strip_option), default)]
    #[getset(get_clone = "pub")]
    start_position: Option<Vec2>,

    // Linerider.com Properties
    #[builder(setter(strip_option, into), default)]
    #[getset(get = "pub")]
    title: Option<String>,
    #[builder(setter(strip_option, into), default)]
    #[getset(get = "pub")]
    artist: Option<String>,
    #[builder(setter(strip_option, into), default)]
    #[getset(get = "pub")]
    description: Option<String>,
    #[builder(setter(strip_option), default)]
    #[getset(get_clone = "pub")]
    duration: Option<u32>,
    #[builder(setter(strip_option, into), default)]
    #[getset(get = "pub")]
    script: Option<String>,

    // LRA+ Properties
    #[builder(setter(strip_option), default)]
    #[getset(get_clone = "pub")]
    gravity_well_size: Option<f64>,
    #[builder(setter(strip_option, into), default)]
    #[getset(get = "pub")]
    audio_filename: Option<String>,
    #[builder(setter(strip_option), default)]
    #[getset(get_clone = "pub")]
    audio_offset_until_start: Option<f64>,
    #[builder(setter(strip_option), default)]
    #[getset(get_clone = "pub")]
    start_gravity: Option<Vec2>,
    #[builder(setter(strip_option), default)]
    #[getset(get_clone = "pub")]
    start_zoom: Option<f64>,
    #[builder(setter(strip_option), default)]
    #[getset(get_clone = "pub")]
    start_line_color: Option<RGBColor>,
    #[builder(setter(strip_option), default)]
    #[getset(get_clone = "pub")]
    start_background_color: Option<RGBColor>,
    #[builder(default)]
    #[getset(get_clone = "pub")]
    lra_remount: bool,
    #[builder(default)]
    #[getset(get_clone = "pub")]
    legacy_fakie: bool, // TODO more descriptive name
    #[builder(default)]
    #[getset(get_clone = "pub")]
    zero_friction_riders: bool,
    #[builder(default)]
    #[getset(get_clone = "pub")]
    zero_velocity_start_riders: bool,
    #[builder(default)]
    #[getset(get_clone = "pub")]
    remount_riders: bool,

    // Flash Properties
    #[builder(setter(strip_option), default)]
    #[getset(get_clone = "pub")]
    start_line: Option<u32>,
}
