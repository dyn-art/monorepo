use bevy_ecs::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
pub fn cfgTest() -> () {
    let mut world = World::default();

    world.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));

    let mut schedule = Schedule::default();
    schedule.add_systems(print_position);
    schedule.run(&mut world);
}

fn print_position(query: Query<(Entity, &Position)>) {
    for (entity, position) in &query {
        log(format!(
            "Entity {:?} is at position: x {}, y {}",
            entity, position.x, position.y
        )
        .as_str());
    }
}
