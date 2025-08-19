mod background_color;
mod camera_zoom;
mod line_color;

pub use background_color::BackgroundColorEvent;
pub use camera_zoom::CameraZoomEvent;
pub use line_color::LineColorEvent;

pub trait Event {}
