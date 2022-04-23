use crate::components::name::Name;
use crate::components::position::Position;
use specs::prelude::*;


pub struct RoachAi;

impl<'a> System<'a> for RoachAi {
    type SystemData = (
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, (names, mut positions): Self::SystemData) {
        for (name, pos) in (&names, &mut positions).join() {
            if name.0 == "Roach".to_string() {
                pos.x += 1;
                if pos.x > 79 {pos.x = 0};
            }
        }
    }
}