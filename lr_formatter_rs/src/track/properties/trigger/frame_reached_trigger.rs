use crate::track::trigger::{Event, EventBuilder};
use derive_builder::Builder;
use getset::CopyGetters;

#[derive(CopyGetters, Debug, Builder)]
#[getset(get_copy = "pub")]
pub struct FrameReachedTrigger {
    frame: u32,
}

impl Event for FrameReachedTrigger {}
impl EventBuilder for FrameReachedTrigger {}
