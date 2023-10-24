use bevy_ecs::{event::EventWriter, system::ResMut};

use crate::{
    bindgen::{
        event_queue::{
            from_js_event_queue::{FromJsEvent, FromJsEventQueue},
            to_js_event_queue::ToJsEventQueue,
        },
        js_bindings,
    },
    core::composition::events::{
        CursorEnteredComposition, CursorExitedComposition, CursorMovedOnComposition,
    },
};

pub fn forward_events_to_js(mut event_queue: ResMut<ToJsEventQueue>) {
    event_queue.forward_events_to_js();
}

pub fn poll_events_from_js(
    mut event_queue: ResMut<FromJsEventQueue>,
    mut cursor_moved_events: EventWriter<CursorMovedOnComposition>,
    mut cursor_entered_events: EventWriter<CursorEnteredComposition>,
    mut cursor_exited_events: EventWriter<CursorExitedComposition>,
) {
    // Poll events from JS
    let events = event_queue.poll_events_from_js();

    // Map JS events to Bevy events
    events.iter().for_each(|event| match event {
        FromJsEvent::PointerDownEventOnEntity { entity } => {
            // TODO
            js_bindings::log(&format!("PointerDownEvent: {:?}", entity));
        }
        FromJsEvent::PointerMovedOnComposition { position } => {
            cursor_moved_events.send(CursorMovedOnComposition {
                position: *position,
            });
            js_bindings::log(&format!("PointerMoveEvent: {:?}", position));
        }
        FromJsEvent::PointerEnteredComposition => {
            cursor_entered_events.send(CursorEnteredComposition);
            js_bindings::log("PointerEnteredComposition");
        }
        FromJsEvent::PointerExitedComposition => {
            cursor_exited_events.send(CursorExitedComposition);
            js_bindings::log("PointerExitedComposition");
        }
    });
}
