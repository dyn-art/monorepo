use bevy_ecs::{component::Tick, system::Resource};

#[derive(Resource, Debug)]
pub struct TickRes {
    pub first_in_cycle: Tick,
}

impl Default for TickRes {
    fn default() -> Self {
        Self {
            first_in_cycle: Tick::new(0),
        }
    }
}
