use crate::track::{
    RGBColor,
    trigger::{Event, EventBuilder},
};
use derive_builder::Builder;
use getset::CopyGetters;

#[derive(CopyGetters, Debug, Builder)]
#[getset(get_copy = "pub")]
pub struct BackgroundColorEvent {
    color: RGBColor,
}

impl Event for BackgroundColorEvent {}
impl EventBuilder for BackgroundColorEventBuilder {}
