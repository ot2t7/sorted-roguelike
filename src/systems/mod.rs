use specs::prelude::*;

// pub mod debug;
pub mod render;

pub fn init_systems() -> Dispatcher<'static, 'static> {
    let dispatcher = DispatcherBuilder::new()
        .build();

    return dispatcher;
}