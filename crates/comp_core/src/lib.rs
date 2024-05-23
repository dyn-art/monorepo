pub mod resources;
mod systems;

use bevy_app::{App, First, Last, Plugin, Update};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use bevy_transform::TransformPlugin;
use dyn_comp_asset::CompAssetPlugin;
use dyn_comp_bundles::{
    events::{CoreInputEvent, InputEvent},
    properties::{CompVersion, Viewport},
};
use dyn_utils::properties::size::Size;
use resources::{
    composition::CompositionRes, layout::LayoutRes, referencer::ReferencerRes, tick::TickRes,
};
use systems::{
    events::{
        create_asset_input_system, create_node_input_system, create_paint_input_system,
        delete_entity_input_system, despawn_removed_entities_system, focus_root_nodes_input_system,
        move_entity_input_system, update_composition_size_input_system,
        update_composition_viewport_input_system, update_drop_shadow_style_input_system,
        update_ellipse_node_input_system, update_entity_blend_mode_input_system,
        update_entity_children_input_system, update_entity_corner_radii_input_system,
        update_entity_opacity_input_system, update_entity_rotation_input_system,
        update_entity_size_input_system, update_entity_transform_input_system,
        update_entity_visibility_input_system, update_fill_style_input_system,
        update_frame_node_input_system, update_gradient_paint_input_system,
        update_image_paint_input_system, update_polygon_node_input_system,
        update_solid_paint_input_system, update_star_node_input_system,
        update_storke_style_input_system, update_text_node_input_system,
    },
    hierarchy::{add_root_component_system, remove_root_component_system, update_hierarchy_levels},
    layout::{
        absolute_layout::{apply_pre_absolute_layout_properties, update_absolute_layout},
        static_layout::{
            discover_new_static_layout_parent_nodes, mark_nodes_with_static_layout_change_as_stale,
            remove_stale_layout_nodes, update_static_layout,
            update_static_layout_parent_nodes_children,
        },
    },
    outline::{
        ellipse::outline_ellipse, polygon::outline_polygon, rectangle::outline_rectangle,
        star::outline_star, text::outline_text,
    },
    stroke::stroke_path_system,
    text::{compute_text_from_scratch, compute_text_on_size_change},
    tick::collect_first_tick,
    vector::resize_vector_node,
};

pub struct CompCorePlugin {
    pub version: Option<CompVersion>,
    pub size: Size,
    pub viewport: Option<Viewport>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CompCoreSystemSet {
    /// After this label, the system has processed input events.
    PreCreateInputEvents,
    CreateInputEvents,
    UpdateInputEvents,

    PreCompute,
    Compute,

    /// After this label, the system has applied layout calculations to the composition's nodes.
    PreLayout,
    StaticLayout,
    AbsoluteLayout,

    // After this label, the system has prepared the nodes for visual outlining.
    Prepare,

    /// After this label, the system has outlined the composition nodes.
    Outline,

    /// After this label, the system has made modifications based on the outlined composition nodes.
    PostOutline,

    Hierarchy,
    PostHierarchy,
}

impl Plugin for CompCorePlugin {
    fn build(&self, app: &mut App) {
        // Register plugins
        app.add_plugins(CompAssetPlugin);
        app.add_plugins(TransformPlugin);

        // Register events
        CoreInputEvent::register_events(app);

        // Register resources
        app.init_resource::<LayoutRes>();
        app.init_resource::<TickRes>();
        app.init_resource::<ReferencerRes>();
        app.insert_resource(CompositionRes {
            version: self.version.unwrap_or_default(),
            viewport: self.viewport.unwrap_or_default(),
            size: self.size,
        });

        // Register system sets
        app.configure_sets(
            Update,
            (
                CompCoreSystemSet::PreCreateInputEvents,
                CompCoreSystemSet::CreateInputEvents,
                CompCoreSystemSet::UpdateInputEvents,
                CompCoreSystemSet::PreCompute,
                CompCoreSystemSet::Compute,
                CompCoreSystemSet::PreLayout,
                CompCoreSystemSet::StaticLayout,
                CompCoreSystemSet::AbsoluteLayout,
                CompCoreSystemSet::Prepare,
                CompCoreSystemSet::Outline,
                CompCoreSystemSet::PostOutline,
                CompCoreSystemSet::Hierarchy,
                CompCoreSystemSet::PostHierarchy,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(First, collect_first_tick);
        app.add_systems(
            Update,
            (
                create_asset_input_system.in_set(CompCoreSystemSet::PreCreateInputEvents),
                create_paint_input_system
                    .in_set(CompCoreSystemSet::PreCreateInputEvents)
                    .after(create_asset_input_system),
                create_node_input_system.in_set(CompCoreSystemSet::CreateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                // Composition
                update_composition_size_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_composition_viewport_input_system
                    .in_set(CompCoreSystemSet::UpdateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                // Node
                update_frame_node_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_ellipse_node_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_star_node_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_polygon_node_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_text_node_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                // Style
                update_fill_style_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_storke_style_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_drop_shadow_style_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                // Paint
                update_solid_paint_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_image_paint_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_gradient_paint_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                // Entity
                delete_entity_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_entity_transform_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_entity_size_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                move_entity_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_entity_rotation_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_entity_visibility_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_entity_corner_radii_input_system
                    .in_set(CompCoreSystemSet::UpdateInputEvents),
                update_entity_blend_mode_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_entity_opacity_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
                update_entity_children_input_system.in_set(CompCoreSystemSet::UpdateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                compute_text_from_scratch.in_set(CompCoreSystemSet::Compute),
                compute_text_on_size_change.in_set(CompCoreSystemSet::Compute),
            ),
        );
        app.add_systems(
            Update,
            (
                apply_pre_absolute_layout_properties.in_set(CompCoreSystemSet::PreLayout),
                discover_new_static_layout_parent_nodes.in_set(CompCoreSystemSet::PreLayout),
                update_static_layout_parent_nodes_children.in_set(CompCoreSystemSet::PreLayout),
                mark_nodes_with_static_layout_change_as_stale.in_set(CompCoreSystemSet::PreLayout),
                remove_stale_layout_nodes.in_set(CompCoreSystemSet::PreLayout),
                update_static_layout.in_set(CompCoreSystemSet::StaticLayout),
                update_absolute_layout.in_set(CompCoreSystemSet::AbsoluteLayout),
            ),
        );
        app.add_systems(
            Update,
            (
                resize_vector_node.in_set(CompCoreSystemSet::Outline),
                outline_rectangle.in_set(CompCoreSystemSet::Outline),
                outline_ellipse.in_set(CompCoreSystemSet::Outline),
                outline_star.in_set(CompCoreSystemSet::Outline),
                outline_polygon.in_set(CompCoreSystemSet::Outline),
                outline_text.in_set(CompCoreSystemSet::Outline),
                stroke_path_system.in_set(CompCoreSystemSet::PostOutline),
            ),
        );
        app.add_systems(
            Update,
            (
                add_root_component_system.in_set(CompCoreSystemSet::Hierarchy),
                remove_root_component_system.in_set(CompCoreSystemSet::Hierarchy),
                update_hierarchy_levels.in_set(CompCoreSystemSet::Hierarchy),
                focus_root_nodes_input_system.in_set(CompCoreSystemSet::PostHierarchy),
            ),
        );
        app.add_systems(Last, despawn_removed_entities_system);
    }
}
