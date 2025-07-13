use crate::track::Vec2;
use derive_builder::Builder;
use getset::CloneGetters;

#[derive(CloneGetters, Builder)]
#[getset(get_clone = "pub")]
pub struct Rider {
    start_position: Vec2,
    #[builder(setter(strip_option), default)]
    start_velocity: Option<Vec2>,
    #[builder(setter(strip_option), default)]
    start_angle: Option<f64>,
    #[builder(setter(strip_option), default)]
    can_remount: Option<bool>,
}
