use getset::CloneGetters;

use crate::track::primitives::events::Event;

#[derive(CloneGetters, Debug, Clone, Copy)]
#[getset(get_clone = "pub")]
pub struct CameraZoomEvent {
    zoom: f64,
}

impl CameraZoomEvent {
    pub fn new(zoom: f64) -> Self {
        Self { zoom }
    }
}

impl Event for CameraZoomEvent {}
