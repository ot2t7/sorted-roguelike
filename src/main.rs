use specs::prelude::*;
use rltk::{RltkBuilder, BError};

mod components;
mod entities;
mod systems;

use entities::roach::make_roach;
use components::name::Name;
use components::position::Position;
use components::renderable::Renderable;

use systems::render::Renderer;

fn main() -> BError {
    let mut world = World::new();

    world.register::<Name>();
    world.register::<Position>();
    world.register::<Renderable>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Renderer, "renderer", &[])
        .build();

    dispatcher.dispatch(&mut world);

    make_roach(
        &mut world, 
        Position {
            x: 0.0,
            y: 0.0
        }, 
        Name("Roach".to_string())
    );

    dispatcher.dispatch(&mut world);

    return Ok(());
}