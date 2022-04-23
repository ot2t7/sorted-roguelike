// This file does not use the ECS System convention for handling the rendering logic.
// This is because RLTK does not allow systems to change the state of the BTerm window.
// So, we're going to be running inside of their tick function instead.

use specs::prelude::*;
use rltk::prelude::*;
use std::result::Result;

use crate::components::renderable::Renderable;
use crate::components::position::Position;

pub fn render_frame(ctx : &mut Rltk, world: &World) {
    ctx.cls();
    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();

    for (pos, render) in (&positions, &renderables).join() {
        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
}

pub fn init_renderer() -> Result<BTerm, Box<dyn std::error::Error + Send + Sync>> {
    let context = RltkBuilder::simple80x50()
        .with_title(" ")
        .build()?;

    return Ok(context);
}