use specs::{Component, VecStorage};
use rltk::RGB;

pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}