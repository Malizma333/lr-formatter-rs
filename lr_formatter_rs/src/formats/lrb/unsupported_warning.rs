use getset::CloneGetters;

#[derive(Debug, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct UnsupportedModWarning {
    name: String,
    version: u16,
    affects_physics: bool,
    affects_camera: bool,
    affect_scenery: bool,
}

impl UnsupportedModWarning {
    pub(super) fn new(name: String, version: u16) -> Self {
        Self {
            name,
            version,
            affects_physics: false,
            affects_camera: false,
            affect_scenery: false,
        }
    }

    pub(super) fn physics(&mut self) -> &mut Self {
        self.affects_physics = true;
        self
    }

    pub(super) fn camera(&mut self) -> &mut Self {
        self.affects_camera = true;
        self
    }

    pub(super) fn scenery(&mut self) -> &mut Self {
        self.affect_scenery = true;
        self
    }
}
