use super::resizing::resize_bounds;
use crate::{
    events::CursorMovedOnArbInputEvent,
    resources::arb_interaction::{HandleSide, ShapeVariant, XYWH},
    utils::transform_point_to_viewport,
};
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query},
};
use bevy_hierarchy::{BuildChildren, Parent};
use bevy_transform::{
    components::{GlobalTransform, Transform},
    TransformBundle,
};
use dyn_arb_bundles::{
    components::{
        marker::Root,
        mixins::{
            AbsoluteLayoutElementMixin, BlendModeMixin, CornerRadiiMixin, OpacityMixin,
            PaintChildMixin, PaintParentMixin, SizeMixin, StyleChildrenMixin, StyleParentMixin,
            VisibilityMixin,
        },
        nodes::{
            ArbNode, ArbNodeVariant, EllipseArbNode, PolygonArbNode, RectangleArbNode,
            StarArbNode,
        },
        paints::{ArbPaint, ArbPaintVariant, SolidArbPaint},
        styles::{ArbStyle, ArbStyleVariant, FillArbStyle},
    },
    utils::{get_parent_global_transfrom, global_to_local_point3},
    EllipseArbNodeBundle, FillStyleBundle, PolygonArbNodeBundle, RectangleArbNodeBundle,
    SolidPaintBundle, StarArbNodeBundle,
};
use dyn_arb_core::resources::canvas::ArtboardRes;
use dyn_utils::properties::{color::Color, size::Size};
use glam::Vec2;
use smallvec::smallvec;

pub fn handle_inserting(
    commands: &mut Commands,
    arb_res: &ArtboardRes,
    query: &mut Query<(&mut Transform, &mut SizeMixin, Option<&Parent>)>,
    root_node_query: &Query<Entity, (With<ArbNode>, With<Root>)>,
    global_transform_query: &Query<&GlobalTransform>,
    event: &CursorMovedOnArbInputEvent,
    maybe_entity: &mut Option<Entity>,
    shape_variant: ShapeVariant,
    origin: &Vec2,
) {
    let CursorMovedOnArbInputEvent {
        position: cursor_position,
        ..
    } = event;
    let global_cursor_position = transform_point_to_viewport(arb_res, cursor_position, true);
    let global_origin = transform_point_to_viewport(arb_res, origin, true);

    if let Some((mut transform, mut size_mixin, maybe_parent)) =
        maybe_entity.and_then(|entity| query.get_mut(entity).ok())
    {
        let SizeMixin(size) = size_mixin.as_mut();
        let maybe_parent_global_transform =
            get_parent_global_transfrom(maybe_parent, global_transform_query);
        let local_cursor_position = global_to_local_point3(
            global_cursor_position.extend(0.0),
            maybe_parent_global_transform,
        )
        .truncate();
        let local_initial_bounds = XYWH {
            position: global_to_local_point3(
                global_origin.extend(0.0),
                maybe_parent_global_transform,
            )
            .truncate(),
            size: Size::zero(),
        };

        let new_bounds = resize_bounds(
            &local_initial_bounds,
            HandleSide::Right as u8 + HandleSide::Bottom as u8,
            &local_cursor_position,
            0.0,
        );

        transform.translation.x = new_bounds.position.x;
        transform.translation.y = new_bounds.position.y;
        *size = new_bounds.size;
    } else {
        // TODO: More advanced logic to determine parent of new node
        let maybe_parent = root_node_query.iter().next();

        let maybe_parent_global_transform =
            maybe_parent.and_then(|parent| global_transform_query.get(parent).ok());
        let local_cursor_position = global_to_local_point3(
            global_cursor_position.extend(0.0),
            maybe_parent_global_transform,
        )
        .truncate();
        let local_initial_bounds = XYWH {
            position: global_to_local_point3(
                global_origin.extend(0.0),
                maybe_parent_global_transform,
            )
            .truncate(),
            size: Size::zero(),
        };

        let new_bounds = resize_bounds(
            &local_initial_bounds,
            HandleSide::Right as u8 + HandleSide::Bottom as u8,
            &local_cursor_position,
            0.0,
        );

        // TODO: Streamline spawning nodes?

        // Spawn node
        let node_entity = match shape_variant {
            ShapeVariant::Rectangle => commands
                .spawn((
                    RectangleArbNodeBundle {
                        node: ArbNode {
                            variant: ArbNodeVariant::Rectangle,
                        },
                        rectangle: RectangleArbNode::default(),
                        transform: TransformBundle::from_transform(Transform::from_translation(
                            new_bounds.position.extend(0.0),
                        )),
                        size: SizeMixin(new_bounds.size),
                        corner_radii: CornerRadiiMixin::default(),
                        visibility: VisibilityMixin::default(),
                        blend_mode: BlendModeMixin::default(),
                        opacity: OpacityMixin::default(),
                    },
                    AbsoluteLayoutElementMixin::default(),
                ))
                .id(),
            ShapeVariant::Ellipse => commands
                .spawn((
                    EllipseArbNodeBundle {
                        node: ArbNode {
                            variant: ArbNodeVariant::Ellipse,
                        },
                        ellipse: EllipseArbNode::default(),
                        transform: TransformBundle::from_transform(Transform::from_translation(
                            new_bounds.position.extend(0.0),
                        )),
                        size: SizeMixin(new_bounds.size),
                        visibility: VisibilityMixin::default(),
                        blend_mode: BlendModeMixin::default(),
                        opacity: OpacityMixin::default(),
                    },
                    AbsoluteLayoutElementMixin::default(),
                ))
                .id(),
            ShapeVariant::Star => commands
                .spawn((
                    StarArbNodeBundle {
                        node: ArbNode {
                            variant: ArbNodeVariant::Star,
                        },
                        star: StarArbNode::default(),
                        transform: TransformBundle::from_transform(Transform::from_translation(
                            new_bounds.position.extend(0.0),
                        )),
                        size: SizeMixin(new_bounds.size),
                        visibility: VisibilityMixin::default(),
                        blend_mode: BlendModeMixin::default(),
                        opacity: OpacityMixin::default(),
                    },
                    AbsoluteLayoutElementMixin::default(),
                ))
                .id(),
            ShapeVariant::Polygon => commands
                .spawn((
                    PolygonArbNodeBundle {
                        node: ArbNode {
                            variant: ArbNodeVariant::Polygon,
                        },
                        polygon: PolygonArbNode::default(),
                        transform: TransformBundle::from_transform(Transform::from_translation(
                            new_bounds.position.extend(0.0),
                        )),
                        size: SizeMixin(new_bounds.size),
                        visibility: VisibilityMixin::default(),
                        blend_mode: BlendModeMixin::default(),
                        opacity: OpacityMixin::default(),
                    },
                    AbsoluteLayoutElementMixin::default(),
                ))
                .id(),
        };

        // Spawn paint
        let paint_entity = commands
            .spawn(SolidPaintBundle {
                paint: ArbPaint {
                    variant: ArbPaintVariant::Solid,
                },
                solid: SolidArbPaint {
                    color: Color::new_rgb(184, 185, 188),
                },
            })
            .id();

        // Spawn fill style
        let mut fill_style_entity_commands = commands.spawn(FillStyleBundle {
            style: ArbStyle {
                variant: ArbStyleVariant::Fill,
            },
            fill: FillArbStyle,
            visibility: VisibilityMixin::default(),
            blend_mode: BlendModeMixin::default(),
            opacity: OpacityMixin::default(),
        });
        fill_style_entity_commands
            .insert((StyleParentMixin(node_entity), PaintChildMixin(paint_entity)));
        let fill_style_entity = fill_style_entity_commands.id();

        // Reference style entity in paint
        let mut paint_entity_commands = commands.entity(paint_entity);
        paint_entity_commands.insert(PaintParentMixin(smallvec![fill_style_entity]));

        // Reference style entity in node
        let mut node_entity_commands = commands.entity(node_entity);
        node_entity_commands.insert(StyleChildrenMixin(smallvec![fill_style_entity]));

        if let Some(parent) = maybe_parent {
            commands.entity(parent).insert_children(0, &[node_entity]);
        } else {
            node_entity_commands.insert(Root);
        }

        *maybe_entity = Some(node_entity);
    }
}
