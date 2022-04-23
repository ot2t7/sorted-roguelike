use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}