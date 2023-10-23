use bevy_ecs::{event::EventWriter, system::ResMut};

use crate::{
    bindgen::{
        event_queue::{
            from_js_event_queue::{FromJsEvent, FromJsEventQueue},
            to_js_event_queue::ToJsEventQueue,
        },
        js_bindings,
    },
    core::canvas::events::{CursorEnteredCanvas, CursorExitedCanvas, CursorMovedOnCanvas},
};

pub fn forward_events_to_js(mut event_queue: ResMut<ToJsEventQueue>) {
    event_queue.forward_events_to_js();
}

pub fn poll_events_from_js(
    mut event_queue: ResMut<FromJsEventQueue>,
    mut cursor_moved_events: EventWriter<CursorMovedOnCanvas>,
    mut cursor_entered_events: EventWriter<CursorEnteredCanvas>,
    mut cursor_exited_events: EventWriter<CursorExitedCanvas>,
) {
    // Poll events from JS
    let events = event_queue.poll_events_from_js();

    // Map JS events to Bevy events
    events.iter().for_each(|event| match event {
        FromJsEvent::PointerDownEventOnEntity { entity } => {
            // TODO
            js_bindings::log(&format!("PointerDownEvent: {:?}", entity));
        }
        FromJsEvent::PointerMovedOnCanvas { position } => {
            cursor_moved_events.send(CursorMovedOnCanvas {
                position: *position,
            });
            js_bindings::log(&format!("PointerMoveEvent: {:?}", position));
        }
        FromJsEvent::PointerEnteredCanvas => {
            cursor_entered_events.send(CursorEnteredCanvas);
            js_bindings::log("PointerEnteredCanvas");
        }
        FromJsEvent::PointerExitedCanvas => {
            cursor_exited_events.send(CursorExitedCanvas);
            js_bindings::log("PointerExitedCanvas");
        }
    });
}
