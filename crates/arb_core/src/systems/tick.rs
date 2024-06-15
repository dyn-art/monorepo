use crate::resources::tick::TickRes;
use bevy_ecs::system::{ResMut, SystemChangeTick};

pub fn collect_first_tick(mut tick_res: ResMut<TickRes>, system_change_tick: SystemChangeTick) {
    tick_res.first_in_cycle = system_change_tick.this_run();
}
