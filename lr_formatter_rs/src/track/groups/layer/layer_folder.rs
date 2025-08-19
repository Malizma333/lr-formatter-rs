use derive_builder::Builder;
use getset::CloneGetters;

#[derive(CloneGetters, Builder)]
#[getset(get_clone = "pub")]
pub struct LayerFolder {
    id: u32,
    index: usize,
    #[builder(setter(strip_option), default)]
    name: Option<String>,
    #[builder(setter(strip_option), default)]
    visible: Option<bool>,
    #[builder(setter(strip_option), default)]
    editable: Option<bool>,
    #[builder(setter(strip_option), default)]
    size: Option<u32>,
}
