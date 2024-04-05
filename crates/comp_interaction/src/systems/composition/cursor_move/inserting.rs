use super::resizing::resize_bounds;
use crate::{
    events::CursorMovedOnCompInputEvent,
    resources::comp_interaction::{HandleSide, ShapeVariant, XYWH},
    utils::transform_point_to_viewport,
};
use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query},
};
use bevy_transform::{components::Transform, TransformBundle};
use dyn_comp_bundles::{
    components::{
        mixins::{
            BlendModeMixin, CornerRadiiMixin, OpacityMixin, PaintChildMixin, PaintParentMixin,
            Root, SizeMixin, StyleChildrenMixin, StyleParentMixin, VisibilityMixin,
        },
        nodes::{CompNode, CompNodeVariant, RectangleCompNode},
        paints::{CompPaint, CompPaintVariant, SolidCompPaint},
        styles::{CompStyle, CompStyleVariant, FillCompStyle},
    },
    FillStyleBundle, RectangleCompNodeBundle, SolidPaintBundle,
};
use dyn_comp_core::resources::composition::CompositionRes;
use dyn_utils::properties::color::Color;
use smallvec::smallvec;

pub fn handle_inserting(
    commands: &mut Commands,
    comp_res: &CompositionRes,
    query: &mut Query<(&mut Transform, &mut SizeMixin)>,
    event: &CursorMovedOnCompInputEvent,
    maybe_entity: &mut Option<Entity>,
    shape_variant: ShapeVariant,
    initial_bounds: &XYWH,
) {
    let CursorMovedOnCompInputEvent {
        position: cursor_position,
        ..
    } = event;
    let cursor_position = transform_point_to_viewport(comp_res, cursor_position, true);

    let new_bounds = resize_bounds(
        initial_bounds,
        HandleSide::Right as u8 + HandleSide::Bottom as u8,
        &cursor_position,
        0.0,
    );

    if let Some((mut transform, mut size_mixin)) =
        maybe_entity.and_then(|entity| query.get_mut(entity).ok())
    {
        let SizeMixin(size) = size_mixin.as_mut();
        transform.translation.x = new_bounds.position.x;
        transform.translation.y = new_bounds.position.y;
        *size = new_bounds.size;
    } else {
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
                    color: Color::black(),
                },
            })
            .id();

        // Spawn style
        let mut style_entity_commands = commands.spawn(FillStyleBundle {
            style: CompStyle {
                variant: CompStyleVariant::Fill,
            },
            fill: FillCompStyle,
            paint: PaintChildMixin(Some(paint_entity)),
            visibility: VisibilityMixin::default(),
            blend_mode: BlendModeMixin::default(),
            opacity: OpacityMixin::default(),
        });
        style_entity_commands.insert(StyleParentMixin(node_entity));
        let style_entity = style_entity_commands.id();

        // Reference style entity in paint
        let mut paint_entity_commands = commands.entity(paint_entity);
        paint_entity_commands.insert(PaintParentMixin(smallvec![style_entity]));

        // Reference style entity in node
        let mut node_entity_commands = commands.entity(node_entity);
        node_entity_commands.insert(StyleChildrenMixin(smallvec![style_entity]));

        // TODO
        node_entity_commands.insert(Root); // TODO: Append to root instead of making it a root node

        *maybe_entity = Some(node_entity);
    }
}
