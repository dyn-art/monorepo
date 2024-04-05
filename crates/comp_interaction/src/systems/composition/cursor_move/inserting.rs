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
            BlendModeMixin, CornerRadiiMixin, OpacityMixin, Root, SizeMixin, VisibilityMixin,
        },
        nodes::{CompNode, CompNodeVariant, RectangleCompNode},
    },
    RectangleCompNodeBundle,
};
use dyn_comp_core::resources::composition::CompositionRes;

pub fn handle_inserting(
    commands: &mut Commands,
    comp_res: &CompositionRes,
    query: &mut Query<(&mut Transform, &mut SizeMixin)>,
    event: &CursorMovedOnCompInputEvent,
    maybe_entity: &mut Option<Entity>,
    shape_variant: ShapeVariant,
    initial_bounds: &XYWH,
) {
    log::info!(
        "[handle_inserting] {:?} - {:?} - {:?} - {:?}",
        event,
        maybe_entity,
        shape_variant,
        initial_bounds
    ); // TODO: REMOVE

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
        let mut entity_commands = match shape_variant {
            ShapeVariant::Rectangle => commands.spawn(RectangleCompNodeBundle {
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
            }),
        };

        // TODO: Append entity to parent? or root or based on where the cursor is
        entity_commands.insert(Root);

        *maybe_entity = Some(entity_commands.id());
    }
}
