use crate::formats::lrb::UnsupportedModWarning;

#[derive(Debug)]
pub enum LrbReadWarning {
    UnsupportedMod(UnsupportedModWarning),
}
