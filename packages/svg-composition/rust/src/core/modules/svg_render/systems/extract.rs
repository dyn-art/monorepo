use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::{Children, Parent};
use dyn_bevy_render_skeleton::extract_param::Extract;
use dyn_composition::core::modules::node::components::{
    mixins::{ChildrenMixin, DimensionMixin, Paint},
    types::Node,
};

use crate::core::{
    mixin_change::ToMixinChange,
    modules::svg_render::resources::changed_components::{
        ChangedComponentsRes, ChangedNode, ChangedPaint,
    },
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

pub fn extract_mixin_generic<C: Component + ToMixinChange>(
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

pub fn extract_paint(
    mut changed: ResMut<ChangedComponentsRes>,
    changed_paint_query: Extract<Query<(Entity, &Paint), Changed<Paint>>>,
    parent_query: Extract<Query<&Parent>>,
    children_query: Extract<Query<&Children>>,
    changed_dimension_query: Extract<Query<(Entity, &DimensionMixin), Changed<DimensionMixin>>>,
    paint_query: Extract<Query<(Entity, &Paint)>>,
) {
    // Query changed paints
    changed_paint_query.for_each(|(entity, paint)| {
        changed.changed_paints.entry(entity).or_insert_with(|| {
            let mut parent_id: Option<Entity> = None;
            let mut parent_dimension: Option<DimensionMixin> = None;

            // Try to get the parent entity id and its dimension mixin
            if let Ok(parent) = parent_query.get(entity) {
                let parent_entity = parent.get();
                parent_id = Some(parent_entity);

                if let Ok(dimension_mixin) =
                    changed_dimension_query.get_component::<DimensionMixin>(parent_entity)
                {
                    parent_dimension = Some(dimension_mixin.clone());
                }
            }

            return ChangedPaint {
                paint: paint.clone(),
                parent_id,
                parent_dimension,
            };
        });
    });

    // Query changed parent dimensions
    // TODO: Improve
    changed_dimension_query.for_each(|(parent_entity, dimension_mixin)| {
        if let Ok(children) = children_query.get_component::<Children>(parent_entity) {
            for child in children.iter() {
                if let Ok(paint) = paint_query.get_component::<Paint>(*child) {
                    changed
                        .changed_paints
                        .entry(*child)
                        .or_insert_with(|| ChangedPaint {
                            paint: paint.clone(),
                            parent_id: Some(parent_entity),
                            parent_dimension: Some(dimension_mixin.clone()),
                        });
                }
            }
        }
    });
}
