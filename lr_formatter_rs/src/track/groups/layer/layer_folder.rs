use derive_builder::Builder;
use getset::CloneGetters;

#[derive(CloneGetters, Builder)]
#[getset(get_clone = "pub")]
pub struct LayerFolder {
    id: u32,
    index: usize,
    name: Option<String>,
    visible: Option<bool>,
    editable: Option<bool>,
    size: Option<u32>,
}
