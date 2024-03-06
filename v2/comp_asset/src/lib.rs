pub mod asset;
pub mod asset_id;
pub mod resources;

use bevy_app::{App, Plugin};
use resources::AssetDatabaseRes;

pub struct CompAssetPlugin;

impl Plugin for CompAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetDatabaseRes>();
    }
}
