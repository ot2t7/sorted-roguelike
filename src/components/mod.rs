use specs::prelude::*;

pub mod name;
pub mod position;
pub mod renderable;

pub fn init_components(world: &mut World) {
    world.register::<name::Name>();
    world.register::<position::Position>();
    world.register::<renderable::Renderable>();
}