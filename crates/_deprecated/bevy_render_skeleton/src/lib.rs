//! https://github.com/bevyengine/bevy/blob/release-0.12.0/crates/bevy_render/src/lib.rs
//!
//! This module provides a stripped-down version of the Bevy renderer's core functionality.
//! Specifically, it initializes a Render sub-app and sets up the basic render cycle.
//! Note that this does not include Bevy's full rendering logic and capabilities,
//! but rather aims to establish the necessary schedules and systems for a render cycle.

pub mod extract_param;

use bevy_app::{App, AppLabel, Plugin, SubApp};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use std::ops::{Deref, DerefMut};

/// Contains the default Bevy rendering backend based on wgpu.
#[derive(Default)]
pub struct RenderPlugin;

/// The labels of the default App rendering sets.
///
/// The sets run in the order listed, with [`apply_deferred`] inserted between each set.
///
/// The `*Flush` sets are assigned to the copy of [`apply_deferred`]
/// that runs immediately after the matching system set.
/// These can be useful for ordering, but you almost never want to add your systems to these sets.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum RenderSet {
    /// The copy of [`apply_deferred`] that runs at the beginning of this schedule.
    /// This is used for applying the commands from the [`ExtractSchedule`]
    ExtractCommands,
    /// Prepare assets that have been created/modified/removed this frame.
    PrepareAssets,
    /// Create any additional views such as those used for shadow mapping.
    ManageViews,
    /// The copy of [`apply_deferred`] that runs immediately after [`ManageViews`](RenderSet::ManageViews).
    ManageViewsFlush,
    /// Queue drawable entities as phase items in [`RenderPhase`](crate::render_phase::RenderPhase)s
    /// ready for sorting
    Queue,
    /// A sub-set within [`Queue`](RenderSet::Queue) where mesh entity queue systems are executed. Ensures `prepare_assets::<Mesh>` is completed.
    QueueMeshes,
    // TODO: This could probably be moved in favor of a system ordering abstraction in `Render` or `Queue`
    /// Sort the [`RenderPhases`](render_phase::RenderPhase) here.
    PhaseSort,
    /// Prepare render resources from extracted data for the GPU based on their sorted order.
    /// Create [`BindGroups`](crate::render_resource::BindGroup) that depend on those data.
    Prepare,
    /// A sub-set within [`Prepare`](RenderSet::Prepare) for initializing buffers, textures and uniforms for use in bind groups.
    PrepareResources,
    /// The copy of [`apply_deferred`] that runs between [`PrepareResources`](RenderSet::PrepareResources) and ['PrepareBindGroups'](RenderSet::PrepareBindGroups).
    PrepareResourcesFlush,
    /// A sub-set within [`Prepare`](RenderSet::Prepare) for constructing bind groups, or other data that relies on render resources prepared in [`PrepareResources`](RenderSet::PrepareResources).
    PrepareBindGroups,
    /// The copy of [`apply_deferred`] that runs immediately after [`Prepare`](RenderSet::Prepare).
    PrepareFlush,
    /// Actual rendering happens here.
    /// In most cases, only the render backend should insert resources here.
    Render,
    /// The copy of [`apply_deferred`] that runs immediately after [`Render`](RenderSet::Render).
    RenderFlush,
    /// Cleanup render resources here.
    Cleanup,
    /// The copy of [`apply_deferred`] that runs immediately after [`Cleanup`](RenderSet::Cleanup).
    CleanupFlush,
}

/// The main render schedule.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Render;

impl Render {
    /// Sets up the base structure of the rendering [`Schedule`].
    ///
    /// The sets defined in this enum are configured to run in order,
    /// and a copy of [`apply_deferred`] is inserted into each [`*Flush` set](RenderSet).
    pub fn base_schedule() -> Schedule {
        use RenderSet::*;

        let mut schedule = Schedule::new(Self);

        // Create "stage-like" structure using buffer flushes + ordering
        schedule.add_systems((
            apply_deferred.in_set(ManageViewsFlush),
            apply_deferred.in_set(PrepareResourcesFlush),
            apply_deferred.in_set(RenderFlush),
            apply_deferred.in_set(PrepareFlush),
            apply_deferred.in_set(CleanupFlush),
        ));

        schedule.configure_sets(
            (
                ExtractCommands,
                ManageViews,
                ManageViewsFlush,
                Queue,
                PhaseSort,
                Prepare,
                PrepareFlush,
                Render,
                RenderFlush,
                Cleanup,
                CleanupFlush,
            )
                .chain(),
        );

        schedule.configure_sets((ExtractCommands, PrepareAssets, Prepare).chain());
        schedule.configure_sets(
            (PrepareResources, PrepareResourcesFlush, PrepareBindGroups)
                .chain()
                .in_set(Prepare),
        );

        schedule
    }
}

/// Schedule which extract data from the main world and inserts it into the render world.
///
/// This step should be kept as short as possible to increase the "pipelining potential" for
/// running the next frame while rendering the current frame.
///
/// This schedule is run on the main world, but its buffers are not applied
/// via [`Schedule::apply_deferred`](bevy_ecs::schedule::Schedule) until it is returned to the render world.
#[derive(ScheduleLabel, PartialEq, Eq, Debug, Clone, Hash)]
pub struct ExtractSchedule;

/// The simulation [`World`] of the application, stored as a resource.
/// This resource is only available during [`ExtractSchedule`] and not
/// during command application of that schedule.
/// See [`Extract`] for more details.
#[derive(Resource, Default)]
pub struct MainWorld(World);

impl Deref for MainWorld {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MainWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A Label for the rendering sub-app.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
pub struct RenderApp;

impl Plugin for RenderPlugin {
    /// Initializes the renderer, sets up the [`RenderSet`] and creates the rendering sub-app.
    fn build(&self, app: &mut App) {
        initialize_render_app(app);
    }
}

/// A "scratch" world used to avoid allocating new worlds every frame when
/// swapping out the [`MainWorld`] for [`ExtractSchedule`].
#[derive(Resource, Default)]
struct ScratchMainWorld(World);

/// Executes the [`ExtractSchedule`] step of the renderer.
/// This updates the render world with the extracted ECS data of the current frame.
fn extract(main_world: &mut World, render_app: &mut App) {
    // temporarily add the app world to the render world as a resource
    let scratch_world = main_world.remove_resource::<ScratchMainWorld>().unwrap();
    let inserted_world = std::mem::replace(main_world, scratch_world.0);
    render_app.world.insert_resource(MainWorld(inserted_world));

    render_app.world.run_schedule(ExtractSchedule);

    // move the app world back, as if nothing happened.
    let inserted_world = render_app.world.remove_resource::<MainWorld>().unwrap();
    let scratch_world = std::mem::replace(main_world, inserted_world.0);
    main_world.insert_resource(ScratchMainWorld(scratch_world));
}

/// SAFETY: this function must be called from the main thread.
fn initialize_render_app(app: &mut App) {
    app.init_resource::<ScratchMainWorld>();

    let mut render_app = App::empty();
    render_app.main_schedule_label = Render.intern();

    let mut extract_schedule = Schedule::new(ExtractSchedule);
    extract_schedule.set_apply_final_deferred(false);

    render_app
        .add_schedule(extract_schedule)
        .add_schedule(Render::base_schedule())
        .add_systems(
            Render,
            (
                // This set applies the commands from the extract schedule while the render schedule
                // is running in parallel with the main app.
                apply_extract_commands.in_set(RenderSet::ExtractCommands),
                World::clear_entities.in_set(RenderSet::Cleanup),
            ),
        );

        let (sender, receiver) = bevy_time::create_time_channels();
        app.insert_resource(receiver);
        render_app.insert_resource(sender);

        app.insert_sub_app(RenderApp, SubApp::new(render_app, move |main_world, render_app| {
            #[cfg(feature = "tracing")]
            let _render_span = bevy_utils::tracing::info_span!("extract main app to render subapp").entered();
            {
                #[cfg(feature = "tracing")]
                let _stage_span =
                    bevy_utils::tracing::info_span!("reserve_and_flush")
                        .entered();
    
                // reserve all existing main world entities for use in render_app
                // they can only be spawned using `get_or_spawn()`
                let total_count = main_world.entities().total_count();
    
                assert_eq!(
                    render_app.world.entities().len(),
                    0,
                    "An entity was spawned after the entity list was cleared last frame and before the extract schedule began. This is not supported",
                );
    
                // This is safe given the clear_entities call in the past frame and the assert above
                unsafe {
                    render_app
                        .world
                        .entities_mut()
                        .flush_and_reserve_invalid_assuming_no_entities(total_count);
                }
            }
    
            // run extract schedule
            extract(main_world, render_app);
        }));
}

/// Applies the commands from the extract schedule. This happens during
/// the render schedule rather than during extraction to allow the commands to run in parallel with the
/// main app when pipelined rendering is enabled.
fn apply_extract_commands(render_world: &mut World) {
    render_world.resource_scope(|render_world, mut schedules: Mut<Schedules>| {
        schedules
            .get_mut(ExtractSchedule)
            .unwrap()
            .apply_deferred(render_world);
    });
}

