use getset::CloneGetters;

use crate::track::primitives::triggers::Trigger;

#[derive(CloneGetters, Debug, Clone, Copy)]
#[getset(get_clone = "pub")]
pub struct FrameBoundsTrigger {
    start: u32,
    end: u32,
}

impl FrameBoundsTrigger {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

impl Trigger for FrameBoundsTrigger {}
