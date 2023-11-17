use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::Parent;
use dyn_bevy_render_skeleton::extract_param::Extract;
use dyn_composition::core::modules::node::components::types::Node;

use crate::core::modules::svg_render::{
    mixin_change::ToMixinChange,
    resources::changed_components::{ChangedComponent, ChangedComponents},
};

pub fn extract_mixin_generic<T: Component + ToMixinChange>(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &Node, &T), (With<Node>, Changed<T>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, node, mixin)| {
        let changed_component = changed.changes.entry(entity).or_insert_with(|| {
            // Try to get the parent entity id
            let mut parent_id: Option<Entity> = None;
            if let Ok(parent) = parent_query.get(entity) {
                parent_id = Some(parent.get());
            }

            return ChangedComponent {
                node_type: node.node_type.clone(),
                changes: vec![],
                parent_id,
            };
        });
        changed_component.changes.push(mixin.to_mixin_change());
    });
}
