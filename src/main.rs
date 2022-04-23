use specs::prelude::*;

mod components;
mod entities;
mod systems;
mod state;

use entities::roach::make_roach;
use components::name::Name;
use components::position::Position;

use systems::render;

fn main() {
    let mut game = state::Game {
        world: World::new(),
        dispatcher: systems::init_systems()
    };

    let ctx = render::init_renderer().expect("Could not start the renderer");
    components::init_components(&mut game.world);

    make_roach(
        &mut game.world, 
        Position {
            x: 3,
            y: 4
        }, 
        Name("Roach".to_string())
    );

    rltk::main_loop(ctx, game).expect("A game tick failed processing.");
}