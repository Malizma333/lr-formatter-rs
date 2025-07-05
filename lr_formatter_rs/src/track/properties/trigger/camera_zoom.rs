use crate::track::trigger::{Event, EventBuilder};
use derive_builder::Builder;
use getset::CopyGetters;

#[derive(CopyGetters, Debug, Builder)]
#[getset(get_copy = "pub")]
pub struct CameraZoomEvent {
    zoom: f64,
}

impl Event for CameraZoomEvent {}
impl EventBuilder for CameraZoomEventBuilder {}
