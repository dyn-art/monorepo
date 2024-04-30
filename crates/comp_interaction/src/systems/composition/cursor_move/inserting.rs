use super::resizing::resize_bounds;
use crate::{
    events::CursorMovedOnCompInputEvent,
    resources::comp_interaction::{HandleSide, ShapeVariant, XYWH},
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
use dyn_comp_bundles::{
    components::{
        marker::Root,
        mixins::{
            BlendModeMixin, CornerRadiiMixin, LayoutElementMixin, OpacityMixin, PaintChildMixin,
            PaintParentMixin, SizeMixin, StyleChildrenMixin, StyleParentMixin, VisibilityMixin,
        },
        nodes::{
            CompNode, CompNodeVariant, EllipseCompNode, PolygonCompNode, RectangleCompNode,
            StarCompNode,
        },
        paints::{CompPaint, CompPaintVariant, SolidCompPaint},
        styles::{CompStyle, CompStyleVariant, FillCompStyle},
    },
    utils::{get_parent_global_transfrom, global_to_local_point3},
    EllipseCompNodeBundle, FillStyleBundle, PolygonCompNodeBundle, RectangleCompNodeBundle,
    SolidPaintBundle, StarCompNodeBundle,
};
use dyn_comp_core::resources::composition::CompositionRes;
use dyn_utils::properties::{color::Color, size::Size};
use glam::Vec2;
use smallvec::smallvec;

pub fn handle_inserting(
    commands: &mut Commands,
    comp_res: &CompositionRes,
    query: &mut Query<(&mut Transform, &mut SizeMixin, Option<&Parent>)>,
    root_node_query: &Query<Entity, (With<CompNode>, With<Root>)>,
    global_transform_query: &Query<&GlobalTransform>,
    event: &CursorMovedOnCompInputEvent,
    maybe_entity: &mut Option<Entity>,
    shape_variant: ShapeVariant,
    origin: &Vec2,
) {
    let CursorMovedOnCompInputEvent {
        position: cursor_position,
        ..
    } = event;
    let global_cursor_position = transform_point_to_viewport(comp_res, cursor_position, true);
    let global_origin = transform_point_to_viewport(comp_res, origin, true);

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
                .spawn(RectangleCompNodeBundle {
                    node: CompNode {
                        variant: CompNodeVariant::Rectangle,
                    },
                    rectangle: RectangleCompNode::default(),
                    transform: TransformBundle::from_transform(Transform::from_translation(
                        new_bounds.position.extend(0.0),
                    )),
                    size: SizeMixin(new_bounds.size),
                    corner_radii: CornerRadiiMixin::default(),
                    visibility: VisibilityMixin::default(),
                    blend_mode: BlendModeMixin::default(),
                    opacity: OpacityMixin::default(),
                    layout_element: LayoutElementMixin::default(),
                })
                .id(),
            ShapeVariant::Ellipse => commands
                .spawn(EllipseCompNodeBundle {
                    node: CompNode {
                        variant: CompNodeVariant::Ellipse,
                    },
                    ellipse: EllipseCompNode::default(),
                    transform: TransformBundle::from_transform(Transform::from_translation(
                        new_bounds.position.extend(0.0),
                    )),
                    size: SizeMixin(new_bounds.size),
                    visibility: VisibilityMixin::default(),
                    blend_mode: BlendModeMixin::default(),
                    opacity: OpacityMixin::default(),
                    layout_element: LayoutElementMixin::default(),
                })
                .id(),
            ShapeVariant::Star => commands
                .spawn(StarCompNodeBundle {
                    node: CompNode {
                        variant: CompNodeVariant::Star,
                    },
                    star: StarCompNode::default(),
                    transform: TransformBundle::from_transform(Transform::from_translation(
                        new_bounds.position.extend(0.0),
                    )),
                    size: SizeMixin(new_bounds.size),
                    visibility: VisibilityMixin::default(),
                    blend_mode: BlendModeMixin::default(),
                    opacity: OpacityMixin::default(),
                    layout_element: LayoutElementMixin::default(),
                })
                .id(),
            ShapeVariant::Polygon => commands
                .spawn(PolygonCompNodeBundle {
                    node: CompNode {
                        variant: CompNodeVariant::Polygon,
                    },
                    polygon: PolygonCompNode::default(),
                    transform: TransformBundle::from_transform(Transform::from_translation(
                        new_bounds.position.extend(0.0),
                    )),
                    size: SizeMixin(new_bounds.size),
                    visibility: VisibilityMixin::default(),
                    blend_mode: BlendModeMixin::default(),
                    opacity: OpacityMixin::default(),
                    layout_element: LayoutElementMixin::default(),
                })
                .id(),
        };

        // Spawn paint
        let paint_entity = commands
            .spawn(SolidPaintBundle {
                paint: CompPaint {
                    variant: CompPaintVariant::Solid,
                },
                solid: SolidCompPaint {
                    color: Color::new_rgb(184, 185, 188),
                },
            })
            .id();

        // Spawn fill style
        let mut fill_style_entity_commands = commands.spawn(FillStyleBundle {
            style: CompStyle {
                variant: CompStyleVariant::Fill,
            },
            fill: FillCompStyle,
            paint: PaintChildMixin(Some(paint_entity)),
            visibility: VisibilityMixin::default(),
            blend_mode: BlendModeMixin::default(),
            opacity: OpacityMixin::default(),
        });
        fill_style_entity_commands.insert(StyleParentMixin(node_entity));
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
