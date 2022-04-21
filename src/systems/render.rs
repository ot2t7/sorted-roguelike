use specs::{ReadStorage, Read, System, Join, World};
use crate::components::renderable::Renderable;
use rltk::{RltkBuilder, BError, BTerm};

// This is the resource which defines the RLTK window for the entire ECS system
#[derive(Default)]
pub struct Window(BTerm);

pub struct Renderer;

impl<'a> System<'a> for Renderer {
    type SystemData = (
        Read<'a, Window>,
        ReadStorage<'a, Renderable>
    );

    fn run(&mut self, (window, renderables): Self::SystemData) {
        for (window, render_info) in (&window, &renderables).join() {
            println!("There's a {}!", render_info.glyph);
        }
    }
}

pub fn init_renderer(world: &mut World) -> BError {
    let context = RltkBuilder::simple80x50()
        .with_title(" ")
        .build()?;

    world.insert(Window(context));

    return Ok(());
}