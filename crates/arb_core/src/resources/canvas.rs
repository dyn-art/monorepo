use bevy_ecs::system::Resource;
use dyn_arb_bundles::properties::{ArbVersion, Viewport};
use dyn_utils::properties::size::Size;

#[derive(Resource, Debug)]
pub struct ArtboardRes {
    pub version: ArbVersion,
    pub viewport: Viewport,
    pub size: Size,
}
