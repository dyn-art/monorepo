//! https://github.com/bevyengine/bevy/blob/release-0.11.3/crates/bevy_render/src/lib.rs
//!
//! This module provides a stripped-down version of the Bevy renderer's core functionality.
//! Specifically, it initializes a Render sub-app and sets up the basic render cycle.
//! Note that this does not include Bevy's full rendering logic and capabilities,
//! but rather aims to establish the necessary schedules and systems for a render cycle.

pub mod extract_param;

use crate::bindgen::js_bindings;
use bevy_app::{App, AppLabel, Plugin, SubApp};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use std::ops::{Deref, DerefMut};

// =============================================================================
// Schedules
// =============================================================================

/// The labels of the default App rendering sets.
///
/// The sets run in the order listed. A copy of [`apply_deferred`] is inserted
/// between each system set, assigned to the corresponding `*Flush` sets.
/// These flush sets apply deferred commands—commands that are queued up for later execution,
/// like entity modifications.
///
/// These `*Flush` sets ensure that any deferred commands queued up during a particular phase
/// are applied before moving on to the next phase, maintaining world state consistency.
/// While useful for ordering, you almost never want to add your custom systems to these flush sets.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum RenderSet {
    /// The copy of [`apply_deferred`] that runs at the beginning of this schedule.
    /// This is used for applying the commands from the [`ExtractSchedule`]
    ExtractCommands,
    /// Prepare render resources from the extracted data for the frontend.
    Prepare,
    /// The copy of [`apply_deferred`] that runs immediately after [`Prepare`](RenderSet::Prepare).
    PrepareFlush,
    /// todo
    Queue,
    /// The copy of [`apply_deferred`] that runs immediately after [`Queue`](RenderSet::Queue).
    QueueFlush,
    /// todo
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
pub struct RenderSchedule;

impl RenderSchedule {
    /// Sets up the base structure of the rendering [`Schedule`].
    ///
    /// The sets defined in this enum are configured to run in order,
    /// and a copy of [`apply_deferred`] is inserted into each `*Flush` set.
    pub fn base_schedule() -> Schedule {
        use RenderSet::*;

        let mut schedule = Schedule::new();

        // Create "stage-like" structure using buffer flushes + ordering
        schedule.add_systems((
            apply_deferred.in_set(PrepareFlush),
            apply_deferred.in_set(QueueFlush),
            apply_deferred.in_set(RenderFlush),
            apply_deferred.in_set(CleanupFlush),
        ));

        schedule.configure_sets(
            (
                ExtractCommands,
                Prepare,
                PrepareFlush,
                Queue,
                QueueFlush,
                Render,
                RenderFlush,
                Cleanup,
                CleanupFlush,
            )
                .chain(),
        );

        return schedule;
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

// =============================================================================
// Resources
// =============================================================================

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

#[derive(Resource, Default)]
struct ScratchMainWorld(World);

// =============================================================================
// App Labels
// =============================================================================

/// A label for the rendering sub-app.
#[derive(AppLabel, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RenderApp;

// =============================================================================
// Systems
// =============================================================================

fn render_system() {
    js_bindings::log("Inside render_system");
}

fn extract_system() {
    js_bindings::log("Inside extract_system");
}

// =============================================================================
// Plugin
// =============================================================================

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    /// Initializes the renderer, sets up the [`RenderSet`](RenderSet) and creates the rendering sub-app.
    fn build(&self, app: &mut App) {
        initialize_render_app(app);
    }
}

// =============================================================================
// Helper
// =============================================================================

/// Executes the [`ExtractSchedule`] step of the renderer.
/// This updates the render world with the extracted ECS data of the current frame.
fn extract(main_world: &mut World, render_app: &mut App) {
    let scratch_world = main_world.remove_resource::<ScratchMainWorld>().unwrap();
    let inserted_world = std::mem::replace(main_world, scratch_world.0);
    render_app.world.insert_resource(MainWorld(inserted_world));

    render_app.world.run_schedule(ExtractSchedule);

    let inserted_world = render_app.world.remove_resource::<MainWorld>().unwrap();
    let scratch_world = std::mem::replace(main_world, inserted_world.0);
    main_world.insert_resource(ScratchMainWorld(scratch_world));
}

// Initialize SVG rendering app
fn initialize_render_app(app: &mut App) {
    app.init_resource::<ScratchMainWorld>();

    let mut render_app = App::empty();
    render_app.main_schedule_label = Box::new(RenderSchedule);

    let mut extract_schedule = Schedule::new();
    extract_schedule.set_apply_final_deferred(false);

    render_app
        .add_schedule(ExtractSchedule, extract_schedule)
        .add_schedule(RenderSchedule, RenderSchedule::base_schedule())
        .add_systems(
            RenderSchedule,
            apply_extract_commands.in_set(RenderSet::ExtractCommands),
        )
        .add_systems(ExtractSchedule, extract_system)
        .add_systems(RenderSchedule, render_system);

    app.insert_sub_app(
        RenderApp,
        SubApp::new(render_app, move |main_world, render_app| {
            extract(main_world, render_app);
        }),
    );
}

/// Applies the commands from the extract schedule. This happens during
/// the render schedule rather than during extraction to allow the commands to run in parallel with the
/// main app when pipelined rendering is enabled.
fn apply_extract_commands(render_world: &mut World) {
    render_world.resource_scope(|render_world, mut schedules: Mut<Schedules>| {
        schedules
            .get_mut(&ExtractSchedule)
            .unwrap()
            .apply_deferred(render_world);
    });
}
