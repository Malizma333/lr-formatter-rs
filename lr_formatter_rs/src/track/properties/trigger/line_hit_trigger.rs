use crate::track::trigger::{Event, EventBuilder};
use derive_builder::Builder;
use getset::CopyGetters;

#[derive(CopyGetters, Debug, Builder)]
#[getset(get_copy = "pub")]
pub struct LineHitTrigger {
    id: u32,
}

impl Event for LineHitTrigger {}
impl EventBuilder for LineHitTriggerBuilder {}
