use super::resizing::resize_bounds;
use crate::{
    events::CursorMovedOnCnvInputEvent,
    resources::cnv_interaction::{HandleSide, ShapeVariant, XYWH},
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
use dyn_cnv_bundles::{
    components::{
        marker::Root,
        mixins::{
            AbsoluteLayoutElementMixin, BlendModeMixin, CornerRadiiMixin, OpacityMixin,
            PaintChildMixin, PaintParentMixin, SizeMixin, StyleChildrenMixin, StyleParentMixin,
            VisibilityMixin,
        },
        nodes::{
            CnvNode, CnvNodeVariant, EllipseCnvNode, PolygonCnvNode, RectangleCnvNode,
            StarCnvNode,
        },
        paints::{CnvPaint, CnvPaintVariant, SolidCnvPaint},
        styles::{CnvStyle, CnvStyleVariant, FillCnvStyle},
    },
    utils::{get_parent_global_transfrom, global_to_local_point3},
    EllipseCnvNodeBundle, FillStyleBundle, PolygonCnvNodeBundle, RectangleCnvNodeBundle,
    SolidPaintBundle, StarCnvNodeBundle,
};
use dyn_cnv_core::resources::canvas::CanvasRes;
use dyn_utils::properties::{color::Color, size::Size};
use glam::Vec2;
use smallvec::smallvec;

pub fn handle_inserting(
    commands: &mut Commands,
    cnv_res: &CanvasRes,
    query: &mut Query<(&mut Transform, &mut SizeMixin, Option<&Parent>)>,
    root_node_query: &Query<Entity, (With<CnvNode>, With<Root>)>,
    global_transform_query: &Query<&GlobalTransform>,
    event: &CursorMovedOnCnvInputEvent,
    maybe_entity: &mut Option<Entity>,
    shape_variant: ShapeVariant,
    origin: &Vec2,
) {
    let CursorMovedOnCnvInputEvent {
        position: cursor_position,
        ..
    } = event;
    let global_cursor_position = transform_point_to_viewport(cnv_res, cursor_position, true);
    let global_origin = transform_point_to_viewport(cnv_res, origin, true);

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
                    RectangleCnvNodeBundle {
                        node: CnvNode {
                            variant: CnvNodeVariant::Rectangle,
                        },
                        rectangle: RectangleCnvNode::default(),
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
                    EllipseCnvNodeBundle {
                        node: CnvNode {
                            variant: CnvNodeVariant::Ellipse,
                        },
                        ellipse: EllipseCnvNode::default(),
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
                    StarCnvNodeBundle {
                        node: CnvNode {
                            variant: CnvNodeVariant::Star,
                        },
                        star: StarCnvNode::default(),
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
                    PolygonCnvNodeBundle {
                        node: CnvNode {
                            variant: CnvNodeVariant::Polygon,
                        },
                        polygon: PolygonCnvNode::default(),
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
                paint: CnvPaint {
                    variant: CnvPaintVariant::Solid,
                },
                solid: SolidCnvPaint {
                    color: Color::new_rgb(184, 185, 188),
                },
            })
            .id();

        // Spawn fill style
        let mut fill_style_entity_commands = commands.spawn(FillStyleBundle {
            style: CnvStyle {
                variant: CnvStyleVariant::Fill,
            },
            fill: FillCnvStyle,
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
