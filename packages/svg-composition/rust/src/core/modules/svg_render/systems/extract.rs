use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::Parent;
use dyn_bevy_render_skeleton::extract_param::Extract;
use dyn_composition::core::modules::node::components::{mixins::Paint, types::Node};

use crate::core::{
    mixin_change::ToMixinChange,
    modules::svg_render::resources::changed_components::{ChangedComponents, ChangedNode},
};

pub fn extract_mixin_generic<T: Component + ToMixinChange>(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &Node, &T), (With<Node>, Changed<T>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, node, mixin)| {
        let changed_component = changed.changed_nodes.entry(entity).or_insert_with(|| {
            // Try to get the parent entity id
            let mut parent_id: Option<Entity> = None;
            if let Ok(parent) = parent_query.get(entity) {
                parent_id = Some(parent.get());
            }

            return ChangedNode {
                node_type: node.node_type.clone(),
                changes: Vec::new(),
                parent_id,
            };
        });
        changed_component.changes.push(mixin.to_mixin_change());
    });
}

pub fn extract_paint(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &Paint), Changed<Paint>>>,
) {
    query.for_each(|(entity, paint)| {
        changed
            .changed_paints
            .entry(entity)
            .or_insert_with(|| paint.clone());
    })
}
