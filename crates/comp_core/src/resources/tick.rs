use bevy_ecs::{component::Tick, system::Resource};

#[derive(Resource, Debug)]
pub struct TickRes {
    pub first_in_cycle: Tick,
}
