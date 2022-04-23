use specs::prelude::*;
use rltk::prelude::*;

use crate::systems::render;

pub struct Game {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>
}

impl GameState for Game {
    fn tick(&mut self, ctx : &mut Rltk) {
        render::render_frame(ctx, &self.world);
        self.run_systems();
    }
}

impl Game {
    fn run_systems(&mut self) {
        self.dispatcher.dispatch(&self.world);
    }
}