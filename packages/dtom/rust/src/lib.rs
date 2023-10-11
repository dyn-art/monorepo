use bevy_ecs::prelude::*;
use wasm_bindgen::prelude::*;

// =============================================================================
// JavaScript bindings
// =============================================================================

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

// =============================================================================
// Components
// =============================================================================

#[derive(Component, Default, Debug)]
struct Position {
    x: f32,
    y: f32,
    flagged: bool, // Whether this component has changed in the last update cycle
}

#[derive(Component, Default, Debug)]
struct Dimensions {
    width: f32,
    height: f32,
    flagged: bool,
}

#[derive(Component, Default, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    flagged: bool,
}

#[derive(Component, Default, Debug)]
struct Rectangle;

#[derive(Bundle, Default, Debug)]
struct RecantgleBundle {
    rectangle: Rectangle,
    position: Position,
    dimensions: Dimensions,
    color: Color,
}

// =============================================================================
// Resources
// =============================================================================

#[derive(Resource, Default, Debug)]
pub struct SharedMouseEvents {
    pub events: Vec<MouseEventType>,
}

#[derive(Debug)]
pub enum MouseEventType {
    Move(MouseMoveEvent),
    Down,
    Up,
}

// =============================================================================
// Events
// =============================================================================

#[derive(Event, Debug)]
pub struct MouseMoveEvent {
    x: f32,
    y: f32,
}

#[derive(Event, Debug)]
struct MouseDownEvent;

#[derive(Event, Debug)]
struct MouseUpEvent;

// =============================================================================
// Systems
// =============================================================================

fn mouse_move_system(mut reader: EventReader<MouseMoveEvent>) {
    for event in reader.iter() {
        // Handle mouse move event
        log(format!("Mouse Move System {:?}", event).as_str());
    }
}

fn mouse_down_system(mut reader: EventReader<MouseDownEvent>) {
    for event in reader.iter() {
        // Handle mouse down event
        log(format!("Mouse Down System {:?}", event).as_str());
    }
}

fn mouse_up_system(mut reader: EventReader<MouseUpEvent>) {
    for event in reader.iter() {
        // Handle mouse up event
        log(format!("Mouse Up System {:?}", event).as_str());
    }
}

fn mouse_event_writer_system(
    mut mouse_move_writer: EventWriter<MouseMoveEvent>,
    mut mouse_down_writer: EventWriter<MouseDownEvent>,
    mut mouse_up_writer: EventWriter<MouseUpEvent>,
    mut shared: ResMut<SharedMouseEvents>,
) {
    while let Some(event) = shared.events.pop() {
        match event {
            MouseEventType::Move(event) => mouse_move_writer.send(event),
            MouseEventType::Down => mouse_down_writer.send(MouseDownEvent),
            MouseEventType::Up => mouse_up_writer.send(MouseUpEvent),
        }
    }
}

// =============================================================================
// Editor
// =============================================================================

#[wasm_bindgen]
pub struct Editor {
    world: World,
    schedule: Schedule,
}

#[wasm_bindgen]
impl Editor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut world = World::default();

        world.insert_resource(SharedMouseEvents { events: Vec::new() });

        let mut schedule = Schedule::default();

        schedule.add_systems(mouse_event_writer_system);
        schedule.add_systems(mouse_move_system);
        schedule.add_systems(mouse_down_system);
        schedule.add_systems(mouse_up_system);

        Editor { world, schedule }
    }

    // Will try Event driven approach first where the scheuler runs only once,
    // when an event from the JS site is received.
    // Otherwise this update method would run in a loop on the JS side.
    // pub fn update(&mut self) {
    //     self.schedule.run(&mut self.world);
    // }

    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        let mut shared = self.world.get_resource_mut::<SharedMouseEvents>().unwrap();
        shared
            .events
            .push(MouseEventType::Move(MouseMoveEvent { x, y }));
        log(format!("Handle Mouse Move: (x: {x}, y: {y})").as_str());
        self.schedule.run(&mut self.world);
    }

    pub fn handle_mouse_down(&mut self) {
        let mut shared = self.world.get_resource_mut::<SharedMouseEvents>().unwrap();
        shared.events.push(MouseEventType::Down);
        log("Handle Mouse Down");
        self.schedule.run(&mut self.world);
    }

    pub fn handle_mouse_up(&mut self) {
        let mut shared = self.world.get_resource_mut::<SharedMouseEvents>().unwrap();
        shared.events.push(MouseEventType::Up);
        log("Handle Mouse Up");
        self.schedule.run(&mut self.world);
    }

    pub fn add_rectangle(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        r: u8,
        g: u8,
        b: u8,
    ) -> () {
        let entity: Entity = self
            .world
            .spawn((RecantgleBundle {
                rectangle: Rectangle,
                position: Position {
                    x,
                    y,
                    flagged: false,
                },
                dimensions: Dimensions {
                    width,
                    height,
                    flagged: false,
                },
                color: Color {
                    r,
                    g,
                    b,
                    flagged: false,
                },
                ..Default::default()
            },))
            .id();

        log(format!("Added rectangle with id {:?}", entity).as_str());
    }
}
