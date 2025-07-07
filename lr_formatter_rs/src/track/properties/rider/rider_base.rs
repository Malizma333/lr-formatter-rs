use crate::track::Vec2;
use derive_builder::Builder;
use getset::CloneGetters;

#[derive(CloneGetters, Debug, Builder)]
#[getset(get_clone = "pub")]
pub struct Rider {
    start_position: Vec2,
    start_velocity: Option<Vec2>,
    start_angle: Option<f64>,
    can_remount: Option<bool>,
}
