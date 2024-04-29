pub mod layout_tree;

use self::layout_tree::LayoutTree;
use bevy_ecs::system::Resource;

#[derive(Resource, Default)]
pub struct LayoutRes {
    pub tree: LayoutTree,
}
