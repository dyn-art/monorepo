use bevy_ecs::system::Resource;
use dyn_comp_bundles::properties::{CompVersion, Viewport};
use dyn_utils::properties::size::Size;

#[derive(Resource, Debug)]
pub struct CompositionRes {
    pub version: CompVersion,
    pub viewport: Viewport,
    pub size: Size,
}
