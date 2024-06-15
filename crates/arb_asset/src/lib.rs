pub mod asset;
pub mod asset_id;
pub mod resources;

use bevy_app::{App, Plugin};
use resources::AssetsRes;

pub struct ArbAssetPlugin;

impl Plugin for ArbAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsRes>();
    }
}
