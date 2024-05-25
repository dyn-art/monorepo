use crate::resources::tick::TickRes;
use bevy_ecs::{
    change_detection::DetectChanges,
    entity::Entity,
    query::{Changed, Or, With},
    system::{Commands, ParamSet, Query, Res},
    world::{Mut, Ref},
};
use bevy_hierarchy::{Children, Parent};
use bevy_transform::components::Transform;
use dyn_comp_bundles::components::mixins::{
    AbsoluteLayoutElementMixin, Constraint, Constraints, PreAbsoluteLayoutProperties, SizeMixin,
};
use dyn_utils::{properties::size::Size, units::abs::Abs};

// Note: Detached absolute layout calculation from the layout tree
// because I couldn't figure out how to make all constraints (e.g. center, scale) work with taffy

pub fn apply_pre_absolute_layout_properties(
    mut commands: Commands,
    tick_res: Res<TickRes>,
    query: Query<
        (Entity, Ref<Transform>, Ref<SizeMixin>, Option<&Parent>),
        (
            With<AbsoluteLayoutElementMixin>,
            Or<(Changed<Transform>, Changed<SizeMixin>)>,
        ),
    >,
    size_mixin_query: Query<&SizeMixin>,
) {
    for (entity, transform, size_mixin, maybe_parent) in query.iter() {
        // Check if Transform or Size has been altered during the current update cycle or the previous one.
        // Modifications within the current cycle (e.g., Translation, Resizing) indicate active user or system interactions
        // that require immediate attention to ensure accurate layout representation.
        // Changes from the previous cycle are typically residual updates from the layout system itself,
        // and should not trigger further updates in this system to avoid redundancy and potential feedback loops.
        //
        // https://discord.com/channels/691052431525675048/1228316069207216130
        if transform.last_changed().get() > tick_res.first_in_cycle.get()
            || size_mixin.last_changed().get() > tick_res.first_in_cycle.get()
            // If its the first update cycle
            || tick_res.first_in_cycle.get() < 100
        {
            let maybe_parent_size = maybe_parent
                .and_then(|parent| size_mixin_query.get(parent.get()).ok())
                .map(|size_mixin| size_mixin.0);

            commands.entity(entity).insert(PreAbsoluteLayoutProperties {
                parent_size: maybe_parent_size,
                size: size_mixin.0,
                translation: transform.translation,
            });
        }
    }
}

pub fn update_absolute_layout(
    mut query_set: ParamSet<(
        Query<(&Children, &SizeMixin), Changed<SizeMixin>>,
        Query<(Entity, &Parent), (With<AbsoluteLayoutElementMixin>, Changed<SizeMixin>)>,
        Query<&SizeMixin, With<Children>>,
        Query<(
            &mut SizeMixin,
            &AbsoluteLayoutElementMixin,
            &mut Transform,
            &PreAbsoluteLayoutProperties,
        )>,
    )>,
) {
    let mut to_update_children: Vec<(Entity, Size)> = Vec::new();

    // Identify to update children from parent
    for (children, SizeMixin(parent_size)) in query_set.p0().iter() {
        for child in children.iter() {
            to_update_children.push((*child, *parent_size));
        }
    }

    // Identify to update children from child
    let to_update_entity_with_parent: Vec<(Entity, Entity)> = query_set
        .p1()
        .iter()
        .map(|(entity, parent)| (entity, parent.get()))
        .collect();
    for (entity, parent) in to_update_entity_with_parent {
        if let Ok(SizeMixin(parent_size)) = query_set.p2().get(parent) {
            to_update_children.push((entity, *parent_size));
        }
    }

    // Compute absolute layout and apply it to the children
    for (child, parent_size) in to_update_children.iter() {
        if let Ok((
            mut size_mixin,
            AbsoluteLayoutElementMixin(absolute_layout_element),
            mut transform,
            layout_metric,
        )) = query_set.p3().get_mut(*child)
        {
            apply_horizontal_constraint(
                &absolute_layout_element.constraints,
                &mut transform,
                &mut size_mixin,
                &parent_size,
                layout_metric,
            );
            apply_vertical_constraint(
                &absolute_layout_element.constraints,
                &mut transform,
                &mut size_mixin,
                &parent_size,
                layout_metric,
            );
        }
    }
}

fn apply_horizontal_constraint(
    constraints: &Constraints,
    child_transform: &mut Mut<Transform>,
    child_size_mixin: &mut Mut<SizeMixin>,
    parent_size: &Size,
    pre_layout_properties: &PreAbsoluteLayoutProperties,
) {
    let PreAbsoluteLayoutProperties {
        parent_size: pre_parent_size,
        size: pre_size,
        translation: pre_translation,
    } = pre_layout_properties;

    match constraints.horizontal {
        Constraint::Start => {
            child_transform.translation.x = pre_translation.x;
        }
        Constraint::Center => {
            let center_offset_x =
                (pre_parent_size.unwrap().width() - pre_size.width()) / 2.0 - pre_translation.x;
            let current_center_x = (parent_size.width() - child_size_mixin.0.width()) / 2.0;
            child_transform.translation.x = current_center_x - center_offset_x;
        }
        Constraint::End => {
            child_transform.translation.x =
                pre_translation.x + parent_size.width() - pre_parent_size.unwrap().width();
        }
        Constraint::Stretch => {
            let left = pre_translation.x;
            let right = pre_translation.x + parent_size.width() - pre_parent_size.unwrap().width();
            child_transform.translation.x = left;
            child_size_mixin.0.width = pre_size.width + Abs::pt(right - left);
        }
        Constraint::Scale => {
            // TODO
            log::warn!("Horizontal 'Scale' constraint currently not supported!");
        }
    }
}

fn apply_vertical_constraint(
    constraints: &Constraints,
    child_transform: &mut Mut<Transform>,
    child_size_mixin: &mut Mut<SizeMixin>,
    parent_size: &Size,
    pre_layout_properties: &PreAbsoluteLayoutProperties,
) {
    let PreAbsoluteLayoutProperties {
        parent_size: pre_parent_size,
        size: pre_size,
        translation: pre_translation,
    } = pre_layout_properties;

    match constraints.vertical {
        Constraint::Start => {
            child_transform.translation.y = pre_translation.y;
        }
        Constraint::Center => {
            let center_offset_y =
                (pre_parent_size.unwrap().height() - pre_size.height()) / 2.0 - pre_translation.y;
            let current_center_y = (parent_size.height() - child_size_mixin.0.height()) / 2.0;
            child_transform.translation.y = current_center_y - center_offset_y;
        }
        Constraint::End => {
            child_transform.translation.y =
                parent_size.height() - pre_parent_size.unwrap().height() + pre_translation.y;
        }
        Constraint::Stretch => {
            let top = pre_translation.y;
            let bottom =
                pre_translation.y + parent_size.height() - pre_parent_size.unwrap().height();
            child_transform.translation.y = top;
            child_size_mixin.0.height = pre_size.height + Abs::pt(bottom - top);
        }
        Constraint::Scale => {
            // TODO
            log::warn!("Vertical 'Scale' constraint currently not supported!");
        }
    }
}
