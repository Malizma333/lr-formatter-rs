use getset::CloneGetters;

use crate::track::primitives::triggers::Trigger;

#[derive(CloneGetters, Debug, Clone, Copy)]
#[getset(get_clone = "pub")]
pub struct LineHitTrigger {
    id: u32,
    frame_length: u32,
}

impl LineHitTrigger {
    pub fn new(id: u32, frame_length: u32) -> Self {
        Self { id, frame_length }
    }
}

impl Trigger for LineHitTrigger {}
