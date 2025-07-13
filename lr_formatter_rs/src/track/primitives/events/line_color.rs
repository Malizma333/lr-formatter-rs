use getset::CloneGetters;

use crate::track::{RGBColor, primitives::events::Event};

#[derive(CloneGetters, Debug, Clone, Copy)]
#[getset(get_clone = "pub")]
pub struct LineColorEvent {
    color: RGBColor,
}

impl LineColorEvent {
    pub fn new(color: RGBColor) -> Self {
        Self { color }
    }
}

impl Event for LineColorEvent {}
