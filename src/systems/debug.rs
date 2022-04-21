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
        let mut entities_counted: u32 = 0;

        for (name, position) in (&names, &positions).join() {
            println!("{:?} at {:?}.", name, position);
            entities_counted += 1;
        }

        if entities_counted == 0 {
            println!("No entities yet!");
        }
    }
}