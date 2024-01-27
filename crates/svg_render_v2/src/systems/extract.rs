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
    types::{Node, NodeType, Paint},
};

use crate::{
    mixin_change::ToMixinChange,
    resources::changed_entities::{ChangedEntitiesRes, ChangedEntity, ChangedEntityType},
};

// Special handling for ChildrenMixin as the ChildrenMixin is no Component itself in the ECS
// as the child parent relation is managed by Bevy's children implementation
pub fn extract_children(
    mut changed_entities: ResMut<ChangedEntitiesRes>,
    query: Extract<Query<(Entity, &Node, &Children), (With<Node>, Changed<Children>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, node, children)| {
        changed_entities
            .changed_entities
            .entry(entity)
            .or_insert_with(|| {
                // Try to get the parent entity id
                let mut parent_id: Option<Entity> = None;
                if let Ok(parent) = parent_query.get(entity) {
                    parent_id = Some(parent.get());
                }

                return ChangedEntity {
                    entity,
                    entity_type: match node.node_type {
                        NodeType::Frame => ChangedEntityType::FrameNode,
                        _ => ChangedEntityType::Unkown,
                    },
                    changes: vec![ChildrenMixin(children.to_vec()).to_mixin_change()],
                    parent_id,
                };
            });
    });
}

pub fn extract_node_mixin_generic<C: Component + ToMixinChange>(
    mut changed_entities: ResMut<ChangedEntitiesRes>,
    query: Extract<Query<(Entity, &Node, &C), (With<Node>, Changed<C>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, node, mixin)| {
        changed_entities
            .changed_entities
            .entry(entity)
            .or_insert_with(|| {
                // Try to get the parent entity id
                let mut parent_id: Option<Entity> = None;
                if let Ok(parent) = parent_query.get(entity) {
                    parent_id = Some(parent.get());
                }

                return ChangedEntity {
                    entity,
                    entity_type: match node.node_type {
                        NodeType::Frame => ChangedEntityType::FrameNode,
                        // TODO
                        _ => ChangedEntityType::Unkown,
                    },
                    changes: vec![mixin.to_mixin_change()],
                    parent_id,
                };
            });
    });
}

pub fn extract_paint_mixin_generic<C: Component + ToMixinChange>(
    mut changed_entities: ResMut<ChangedEntitiesRes>,
    query: Extract<Query<(Entity, &Paint, &C), (With<Paint>, Changed<C>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, paint, mixin)| {
        changed_entities
            .changed_entities
            .entry(entity)
            .or_insert_with(|| {
                // Try to get the parent entity id
                let mut parent_id: Option<Entity> = None;
                if let Ok(parent) = parent_query.get(entity) {
                    parent_id = Some(parent.get());
                }

                return ChangedEntity {
                    entity,
                    entity_type: match paint.paint_type {
                        // TODO
                        _ => ChangedEntityType::Unkown,
                    },
                    changes: vec![mixin.to_mixin_change()],
                    parent_id,
                };
            });
    });
}
