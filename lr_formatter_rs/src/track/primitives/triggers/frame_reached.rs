use getset::CloneGetters;

use crate::track::primitives::triggers::Trigger;

#[derive(CloneGetters, Debug, Clone, Copy)]
#[getset(get_clone = "pub")]
pub struct FrameReachedTrigger {
    frame: u32,
}

impl FrameReachedTrigger {
    pub fn new(frame: u32) -> Self {
        Self { frame }
    }
}

impl Trigger for FrameReachedTrigger {}
