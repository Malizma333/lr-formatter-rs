use crate::track::Vec2;

pub fn from_lra_zoom(zoom: f32) -> f64 {
    f64::log(f64::from(zoom), 2.0)
}

pub fn to_lra_zoom(zoom: f64) -> f64 {
    f64::powf(2.0, zoom)
}

pub fn from_lrweb_gravity(gravity: Vec2) -> Vec2 {
    Vec2::new(gravity.x() / 0.175, gravity.y() / 0.175)
}

pub fn to_lrweb_gravity(gravity: Vec2) -> Vec2 {
    Vec2::new(gravity.x() * 0.175, gravity.y() * 0.175)
}

pub fn from_lra_scenery_width(width: u8) -> f64 {
    f64::from(width) / 10.0
}
