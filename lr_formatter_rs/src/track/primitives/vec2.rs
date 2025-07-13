use getset::CloneGetters;

#[derive(Debug, Clone, Copy, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
