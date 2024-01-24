use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::{Children, Parent};
use dyn_bevy_render_skeleton::extract_param::Extract;
use dyn_composition::core::modules::node::components::{
    mixins::ChildrenMixin,
    types::{Node, Paint},
};

use crate::{
    mixin_change::{ToNodeMixinChange, ToPaintMixinChange},
    resources::changed_components::{ChangedComponentsRes, ChangedNode, ChangedPaint},
};

// Special handling for ChildrenMixin as the ChildrenMixin is no Component itself in the ECS
// as the child parent relation is managed by Bevy's children implementation
pub fn extract_children(
    mut changed: ResMut<ChangedComponentsRes>,
    query: Extract<Query<(Entity, &Node, &Children), (With<Node>, Changed<Children>)>>,
    parent_query: Extract<Query<&Parent>>,
    node_query: Extract<Query<Entity, With<Node>>>,
) {
    query.for_each(|(entity, node, children)| {
        let changed_component = changed.changed_nodes.entry(entity).or_insert_with(|| {
            let mut parent_id: Option<Entity> = None;

            // Try to get the parent entity id
            if let Ok(parent) = parent_query.get(entity) {
                parent_id = Some(parent.get());
            }

            return ChangedNode {
                node_type: node.node_type.clone(),
                changes: Vec::new(),
                parent_id,
            };
        });

        // TODO: Improve
        // Note: Also paints are included here as they are managed with Bevy child parent relation too

        // Filter children to include only those that are Nodes
        let node_children: Vec<_> = children
            .iter()
            .filter_map(|child_entity| {
                if node_query.get(*child_entity).is_ok() {
                    Some(child_entity.clone())
                } else {
                    None
                }
            })
            .clone()
            .collect();

        changed_component
            .changes
            .push(ChildrenMixin(node_children).to_mixin_change());
    });
}

pub fn extract_node_mixin_generic<C: Component + ToNodeMixinChange>(
    mut changed: ResMut<ChangedComponentsRes>,
    query: Extract<Query<(Entity, &Node, &C), (With<Node>, Changed<C>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, node, mixin)| {
        let changed_component = changed.changed_nodes.entry(entity).or_insert_with(|| {
            let mut parent_id: Option<Entity> = None;

            // Try to get the parent entity id
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

pub fn extract_paint_mixin_generic<C: Component + ToPaintMixinChange>(
    mut changed: ResMut<ChangedComponentsRes>,
    query: Extract<Query<(Entity, &Paint, &C), (With<Paint>, Changed<C>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, paint, mixin)| {
        let changed_component = changed.changed_paints.entry(entity).or_insert_with(|| {
            let mut parent_id: Option<Entity> = None;

            // Try to get the parent entity id
            if let Ok(parent) = parent_query.get(entity) {
                parent_id = Some(parent.get());
            }

            return ChangedPaint {
                paint_type: paint.paint_type.clone(),
                changes: Vec::new(),
                parent_id,
            };
        });

        changed_component.changes.push(mixin.to_mixin_change());
    });
}
