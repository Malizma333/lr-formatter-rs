pub mod background_color;
pub mod camera_zoom;
pub mod frame_reached_trigger;
pub mod line_color;
pub mod line_hit_trigger;
pub mod trigger_group;
pub mod triggered_event;

trait Event: Default {}
trait EventBuilder: Default {}
trait Trigger: Default {}
trait TriggerBuilder: Default {}
