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
    resources::changed_components::{
        ChangedComponentsRes, ChangedEntity, ChangedNode, ChangedPaint,
    },
};

// Special handling for ChildrenMixin as the ChildrenMixin is no Component itself in the ECS
// as the child parent relation is managed by Bevy's children implementation
pub fn extract_children(
    mut changed: ResMut<ChangedComponentsRes>,
    query: Extract<Query<(Entity, &Node, &Children), (With<Node>, Changed<Children>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, node, children)| {
        let changed_component = changed.changed_entities.entry(entity).or_insert_with(|| {
            let mut parent_id: Option<Entity> = None;

            // Try to get the parent entity id
            if let Ok(parent) = parent_query.get(entity) {
                parent_id = Some(parent.get());
            }

            return ChangedEntity::Node(ChangedNode {
                node_type: node.node_type.clone(),
                changes: Vec::new(),
                parent_id,
            });
        });

        match changed_component {
            ChangedEntity::Node(changed_node) => {
                changed_node
                    .changes
                    .push(ChildrenMixin(children.to_vec()).to_mixin_change());
            }
            _ => {}
        }
    });
}

pub fn extract_node_mixin_generic<C: Component + ToNodeMixinChange>(
    mut changed: ResMut<ChangedComponentsRes>,
    query: Extract<Query<(Entity, &Node, &C), (With<Node>, Changed<C>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, node, mixin)| {
        let changed_component = changed.changed_entities.entry(entity).or_insert_with(|| {
            let mut parent_id: Option<Entity> = None;

            // Try to get the parent entity id
            if let Ok(parent) = parent_query.get(entity) {
                parent_id = Some(parent.get());
            }

            return ChangedEntity::Node(ChangedNode {
                node_type: node.node_type.clone(),
                changes: Vec::new(),
                parent_id,
            });
        });

        match changed_component {
            ChangedEntity::Node(changed_node) => {
                changed_node.changes.push(mixin.to_mixin_change());
            }
            _ => {}
        }
    });
}

pub fn extract_paint_mixin_generic<C: Component + ToPaintMixinChange>(
    mut changed: ResMut<ChangedComponentsRes>,
    query: Extract<Query<(Entity, &Paint, &C), (With<Paint>, Changed<C>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, paint, mixin)| {
        let changed_component = changed.changed_entities.entry(entity).or_insert_with(|| {
            let mut parent_id: Option<Entity> = None;

            // Try to get the parent entity id
            if let Ok(parent) = parent_query.get(entity) {
                parent_id = Some(parent.get());
            }

            return ChangedEntity::Paint(ChangedPaint {
                paint_type: paint.paint_type.clone(),
                changes: Vec::new(),
                parent_id,
            });
        });

        match changed_component {
            ChangedEntity::Paint(changed_paint) => {
                changed_paint.changes.push(mixin.to_mixin_change());
            }
            _ => {}
        }
    });
}
