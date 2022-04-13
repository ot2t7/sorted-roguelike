use specs::{ReadStorage, System, Join};
use crate::components::name::Name;
use crate::components::position::Position;

pub struct DebugPrinter;

impl<'a> System<'a> for DebugPrinter {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, (positions, names): Self::SystemData) {
        for (name, position) in (&names, &positions).join() {
            println!("{:?} at {:?}.", name, position);
        }
    }
}