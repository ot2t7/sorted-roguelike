use specs::prelude::*;

// pub mod debug;
pub mod render;
pub mod roach_ai;

pub fn init_systems() -> Dispatcher<'static, 'static> {
    let dispatcher = DispatcherBuilder::new()
        .with(roach_ai::RoachAi, "roach_ai", &[])
        .build();

    return dispatcher;
}