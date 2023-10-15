use crate::js_bindings;
use bevy_app::{App, AppLabel, Plugin, SubApp};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

// =============================================================================
// Schedules
// =============================================================================

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RenderSchedule;

impl RenderSchedule {
    // Initialize the base schedule for rendering
    pub fn base_schedule() -> Schedule {
        let mut schedule = Schedule::new();
        // TODO:
        return schedule;
    }
}

#[derive(ScheduleLabel, PartialEq, Eq, Debug, Clone, Hash)]
pub struct ExtractSchedule;

// =============================================================================
// Resources
// =============================================================================

#[derive(Resource, Default)]
pub struct MainWorld(World);

#[derive(Resource, Default)]
struct ScratchMainWorld(World);

// =============================================================================
// App Labels
// =============================================================================

#[derive(AppLabel, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct SVGRenderApp;

// =============================================================================
// Plugin
// =============================================================================

pub struct SVGRenderPlugin;

impl Plugin for SVGRenderPlugin {
    fn build(&self, app: &mut App) {
        initialize_svg_render_app(app);
    }
}

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
// Helper
// =============================================================================

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
fn initialize_svg_render_app(app: &mut App) {
    app.init_resource::<ScratchMainWorld>();

    let mut render_app = App::empty();
    render_app.main_schedule_label = Box::new(RenderSchedule);

    let mut extract_schedule = Schedule::new();
    extract_schedule.set_apply_final_deferred(false);

    render_app
        .add_schedule(ExtractSchedule, extract_schedule)
        .add_schedule(RenderSchedule, RenderSchedule::base_schedule())
        .add_systems(ExtractSchedule, extract_system)
        .add_systems(RenderSchedule, render_system);

    app.insert_sub_app(
        SVGRenderApp,
        SubApp::new(render_app, move |main_world, render_app| {
            extract(main_world, render_app);
        }),
    );
}

// Take inspiration from: https://www.youtube.com/watch?v=dYO8bzF8aSc&t=20s
// https://github.com/Nilirad/bevy_prototype_lyon
// https://www.youtube.com/watch?v=5oKEPZ6LbNE&t=777s

// 1. Extract relevant data and create a "render" entity
// 2. Prepare the render entity for the renderer -> create path, color, etc.
// 3. Queue drawable entities as phase items
// 4. Render - Send message to frontend
