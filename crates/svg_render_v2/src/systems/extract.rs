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
    types::{
        GradientPaint, GradientPaintVariant, ImagePaint, ImagePaintScaleMode, Node, NodeType,
        Paint, PaintType,
    },
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
        let changed_entity = changed_entities
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
                        NodeType::Rectangle => ChangedEntityType::ShapeNode,
                        NodeType::Text => ChangedEntityType::ShapeNode,
                        _ => ChangedEntityType::Unkown,
                    },
                    changes: Vec::new(),
                    parent_id,
                };
            });

        changed_entity
            .changes
            .push(ChildrenMixin(children.to_vec()).to_mixin_change());
    });
}

pub fn extract_node_mixin_generic<C: Component + ToMixinChange>(
    mut changed_entities: ResMut<ChangedEntitiesRes>,
    query: Extract<Query<(Entity, &Node, &C), (With<Node>, Changed<C>)>>,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(|(entity, node, mixin)| {
        let changed_entity = changed_entities
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
                        NodeType::Rectangle => ChangedEntityType::ShapeNode,
                        NodeType::Text => ChangedEntityType::ShapeNode,
                        _ => ChangedEntityType::Unkown,
                    },
                    changes: Vec::new(),
                    parent_id,
                };
            });

        changed_entity.changes.push(mixin.to_mixin_change());
    });
}

pub fn extract_paint_mixin_generic<C: Component + ToMixinChange>(
    mut changed_entities: ResMut<ChangedEntitiesRes>,
    query: Extract<
        Query<
            (
                Entity,
                &Paint,
                Option<&ImagePaint>,
                Option<&GradientPaint>,
                &C,
            ),
            (With<Paint>, Changed<C>),
        >,
    >,
    parent_query: Extract<Query<&Parent>>,
) {
    query.for_each(
        |(entity, paint, maybe_image_paint, maybe_gradient_paint, mixin)| {
            let changed_entity = changed_entities
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
                            PaintType::Solid => ChangedEntityType::SolidPaint,
                            PaintType::Image => {
                                if let Some(image_paint) = maybe_image_paint {
                                    match image_paint.scale_mode {
                                        ImagePaintScaleMode::Fill { .. } => {
                                            ChangedEntityType::ImageFillPaint
                                        }

                                        ImagePaintScaleMode::Fit { .. } => {
                                            ChangedEntityType::ImageFitPaint
                                        }

                                        ImagePaintScaleMode::Crop { .. } => {
                                            ChangedEntityType::ImageCropPaint
                                        }

                                        ImagePaintScaleMode::Tile { .. } => {
                                            ChangedEntityType::ImageTilePaint
                                        }
                                    }
                                } else {
                                    ChangedEntityType::Unkown
                                }
                            }
                            PaintType::Gradient => {
                                if let Some(gradient_paint) = maybe_gradient_paint {
                                    match gradient_paint.variant {
                                        GradientPaintVariant::Linear { .. } => {
                                            ChangedEntityType::LinearGradientPaint
                                        }
                                        GradientPaintVariant::Radial { .. } => {
                                            ChangedEntityType::RadialGradientPaint
                                        }
                                    }
                                } else {
                                    ChangedEntityType::Unkown
                                }
                            }
                            _ => ChangedEntityType::Unkown,
                        },
                        changes: Vec::new(),
                        parent_id,
                    };
                });

            changed_entity.changes.push(mixin.to_mixin_change());
        },
    );
}
