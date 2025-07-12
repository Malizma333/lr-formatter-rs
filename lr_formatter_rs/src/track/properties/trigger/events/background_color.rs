use crate::track::RGBColor;
use derive_builder::Builder;
use getset::CloneGetters;

#[derive(CloneGetters, Debug, Builder)]
#[getset(get_clone = "pub")]
pub struct BackgroundColorEvent {
    color: RGBColor,
}
