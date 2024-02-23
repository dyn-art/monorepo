use bevy_ecs::world::World;
use dyn_comp_types::events::InputEvent;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum InteractionInputEvent {}

impl InputEvent for InteractionInputEvent {
    fn send_into_ecs(self, world: &mut World) {
        todo!()
    }
}
