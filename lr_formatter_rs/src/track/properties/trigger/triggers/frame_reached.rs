use derive_builder::Builder;
use getset::CloneGetters;

#[derive(CloneGetters, Debug, Builder)]
#[getset(get_clone = "pub")]
pub struct FrameReachedTrigger {
    frame: u32,
}
