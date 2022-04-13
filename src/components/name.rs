use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Name(pub String);

impl Component for Name {
    type Storage = VecStorage<Self>;
}