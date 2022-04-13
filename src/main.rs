use specs::prelude::*;

mod components;
mod entities;
mod systems;

use entities::roach::make_roach;
use components::name::Name;
use components::position::Position;

use systems::debug::DebugPrinter;

fn main() {
    let mut world = World::new();

    world.register::<Name>();
    world.register::<Position>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(DebugPrinter, "debug_printer", &[])
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
}