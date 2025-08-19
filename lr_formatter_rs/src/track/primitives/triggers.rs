mod frame_bounds;
mod frame_reached;
mod line_hit;

pub use frame_bounds::FrameBoundsTrigger;
pub use frame_reached::FrameReachedTrigger;
pub use line_hit::LineHitTrigger;

pub trait Trigger {}
