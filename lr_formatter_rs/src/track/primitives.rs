mod events;
mod rgb_color;
mod triggers;
mod vec2;

pub use events::{BackgroundColorEvent, CameraZoomEvent, Event, LineColorEvent};
pub use rgb_color::RGBColor;
pub use triggers::{FrameBoundsTrigger, FrameReachedTrigger, LineHitTrigger, Trigger};
pub use vec2::Vec2;
