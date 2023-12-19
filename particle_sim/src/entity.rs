use ultraviolet::Vec2;

pub(crate) struct Entity {
    pub pos: Vec2,
    pub col: u32,
    pub radius: f32,
    pub vel: f32,
    pub acc: f32,
}
