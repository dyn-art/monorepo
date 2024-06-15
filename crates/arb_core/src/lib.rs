mod debug;
pub mod resources;
mod systems;

use bevy_app::{App, First, Last, Plugin, Update};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use bevy_transform::TransformPlugin;
use dyn_arb_asset::ArbAssetPlugin;
use dyn_arb_bundles::{
    events::{CoreInputEvent, InputEvent},
    properties::{ArbVersion, Viewport},
};
use dyn_utils::properties::size::Size;
use resources::{canvas::ArtboardRes, layout::LayoutRes, referencer::ReferencerRes, tick::TickRes};
use systems::{
    cleanup::despawn_removed_entities_system,
    events::{
        create_asset_input_system, create_node_input_system, create_paint_input_system,
        delete_entity_input_system, focus_root_nodes_input_system, move_entity_input_system,
        update_canvas_size_input_system, update_canvas_viewport_input_system,
        update_drop_shadow_style_input_system, update_ellipse_node_input_system,
        update_entity_blend_mode_input_system, update_entity_children_input_system,
        update_entity_corner_radii_input_system, update_entity_opacity_input_system,
        update_entity_rotation_input_system, update_entity_size_input_system,
        update_entity_transform_input_system, update_entity_visibility_input_system,
        update_fill_style_input_system, update_frame_node_input_system,
        update_gradient_paint_input_system, update_image_paint_input_system,
        update_polygon_node_input_system, update_solid_paint_input_system,
        update_star_node_input_system, update_storke_style_input_system,
        update_text_node_input_system,
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

pub struct ArbCorePlugin {
    pub version: Option<ArbVersion>,
    pub size: Size,
    pub viewport: Option<Viewport>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ArbCoreSystemSet {
    /// After this label, the system has processed input events.
    PreCreateInputEvents,
    CreateInputEvents,
    UpdateInputEvents,

    PreCompute,
    Compute,

    /// After this label, the system has applied layout calculations to the canvas's nodes.
    PreLayout,
    StaticLayout,
    AbsoluteLayout,

    // After this label, the system has prepared the nodes for visual outlining.
    Prepare,

    /// After this label, the system has outlined the canvas nodes.
    Outline,

    /// After this label, the system has made modifications based on the outlined canvas nodes.
    PostOutline,

    Hierarchy,
    PostHierarchy,
}

impl Plugin for ArbCorePlugin {
    fn build(&self, app: &mut App) {
        // Register plugins
        app.add_plugins(ArbAssetPlugin);
        app.add_plugins(TransformPlugin);

        // Register events
        CoreInputEvent::register_events(app);

        // Register resources
        app.init_resource::<LayoutRes>();
        app.init_resource::<TickRes>();
        app.init_resource::<ReferencerRes>();
        app.insert_resource(ArtboardRes {
            version: self.version.unwrap_or_default(),
            viewport: self.viewport.unwrap_or_default(),
            size: self.size,
        });
        #[cfg(feature = "lua_scripts")]
        app.init_resource::<resources::lua::LuaRes>();

        // Register system sets
        app.configure_sets(
            Update,
            (
                ArbCoreSystemSet::PreCreateInputEvents,
                ArbCoreSystemSet::CreateInputEvents,
                ArbCoreSystemSet::UpdateInputEvents,
                ArbCoreSystemSet::PreCompute,
                ArbCoreSystemSet::Compute,
                ArbCoreSystemSet::PreLayout,
                ArbCoreSystemSet::StaticLayout,
                ArbCoreSystemSet::AbsoluteLayout,
                ArbCoreSystemSet::Prepare,
                ArbCoreSystemSet::Outline,
                ArbCoreSystemSet::PostOutline,
                ArbCoreSystemSet::Hierarchy,
                ArbCoreSystemSet::PostHierarchy,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(
            First,
            (
                collect_first_tick,
                #[cfg(feature = "lua_scripts")]
                systems::events::register_lua_script_input_system,
                #[cfg(feature = "lua_scripts")]
                systems::events::execute_lua_script_input_system
                    .after(systems::events::register_lua_script_input_system),
            ),
        );
        app.add_systems(
            Update,
            (
                create_asset_input_system.in_set(ArbCoreSystemSet::PreCreateInputEvents),
                create_paint_input_system
                    .in_set(ArbCoreSystemSet::PreCreateInputEvents)
                    .after(create_asset_input_system),
                create_node_input_system.in_set(ArbCoreSystemSet::CreateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                // Artboard
                update_canvas_size_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_canvas_viewport_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                // Node
                update_frame_node_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_ellipse_node_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_star_node_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_polygon_node_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_text_node_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                // Style
                update_fill_style_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_storke_style_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_drop_shadow_style_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                // Paint
                update_solid_paint_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_image_paint_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_gradient_paint_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                // Entity
                delete_entity_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_entity_transform_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_entity_size_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                move_entity_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_entity_rotation_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_entity_visibility_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_entity_corner_radii_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_entity_blend_mode_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_entity_opacity_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
                update_entity_children_input_system.in_set(ArbCoreSystemSet::UpdateInputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                compute_text_from_scratch.in_set(ArbCoreSystemSet::Compute),
                compute_text_on_size_change.in_set(ArbCoreSystemSet::Compute),
            ),
        );
        app.add_systems(
            Update,
            (
                apply_pre_absolute_layout_properties.in_set(ArbCoreSystemSet::PreLayout),
                discover_new_static_layout_parent_nodes.in_set(ArbCoreSystemSet::PreLayout),
                update_static_layout_parent_nodes_children.in_set(ArbCoreSystemSet::PreLayout),
                mark_nodes_with_static_layout_change_as_stale.in_set(ArbCoreSystemSet::PreLayout),
                remove_stale_layout_nodes.in_set(ArbCoreSystemSet::PreLayout),
                update_static_layout.in_set(ArbCoreSystemSet::StaticLayout),
                update_absolute_layout.in_set(ArbCoreSystemSet::AbsoluteLayout),
            ),
        );
        app.add_systems(
            Update,
            (
                resize_vector_node.in_set(ArbCoreSystemSet::Outline),
                outline_rectangle.in_set(ArbCoreSystemSet::Outline),
                outline_ellipse.in_set(ArbCoreSystemSet::Outline),
                outline_star.in_set(ArbCoreSystemSet::Outline),
                outline_polygon.in_set(ArbCoreSystemSet::Outline),
                outline_text.in_set(ArbCoreSystemSet::Outline),
                stroke_path_system.in_set(ArbCoreSystemSet::PostOutline),
            ),
        );
        app.add_systems(
            Update,
            (
                add_root_component_system.in_set(ArbCoreSystemSet::Hierarchy),
                remove_root_component_system.in_set(ArbCoreSystemSet::Hierarchy),
                update_hierarchy_levels.in_set(ArbCoreSystemSet::Hierarchy),
                focus_root_nodes_input_system.in_set(ArbCoreSystemSet::PostHierarchy),
            ),
        );
        app.add_systems(Last, despawn_removed_entities_system);
    }
}
