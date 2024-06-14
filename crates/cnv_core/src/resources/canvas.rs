use bevy_ecs::system::Resource;
use dyn_cnv_bundles::properties::{CnvVersion, Viewport};
use dyn_utils::properties::size::Size;

#[derive(Resource, Debug)]
pub struct CanvasRes {
    pub version: CnvVersion,
    pub viewport: Viewport,
    pub size: Size,
}
