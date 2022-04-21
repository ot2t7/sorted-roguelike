use specs::prelude::*;
use rltk::prelude::*;

use crate::components::position::Position;
use crate::components::name::Name;
use crate::components::renderable::Renderable;

pub fn make_roach(world: &mut World, pos: Position, name: Name) -> Entity {
    world.create_entity()
        .with(pos)
        .with(name)
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(PURPLE),
            bg: RGB::named(BLACK)
        })
        .build()
}

