use specs::prelude::*;

use crate::components::position::Position;
use crate::components::name::Name;

pub fn make_roach(world: &mut World, pos: Position, name: Name) -> Entity {
    world.create_entity()
        .with(pos)
        .with(name)
        .build()
}

