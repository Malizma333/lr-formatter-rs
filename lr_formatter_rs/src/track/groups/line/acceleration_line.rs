use crate::track::Vec2;
use derive_builder::Builder;
use getset::CloneGetters;

#[derive(CloneGetters, Builder)]
#[getset(get_clone = "pub")]
pub struct AccelerationLine {
    id: u32,
    #[getset(skip)]
    endpoints: (Vec2, Vec2),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    multiplier: Option<f64>,
}

impl AccelerationLine {
    pub fn x1(&self) -> f64 {
        self.endpoints.0.x()
    }

    pub fn y1(&self) -> f64 {
        self.endpoints.0.y()
    }

    pub fn x2(&self) -> f64 {
        self.endpoints.1.x()
    }

    pub fn y2(&self) -> f64 {
        self.endpoints.1.y()
    }
}
