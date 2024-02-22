use std::sync::mpsc::{channel, Receiver};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use dyn_comp::prelude::*;
use dyn_comp_interaction::CompInteractionPlugin;
use dyn_comp_svg_builder::{events::SvgBuilderOutputEvent, CompSvgBuilderPlugin};
use events::{SvgCompInputEvent, SvgCompOutputEvent};
use wasm_bindgen::prelude::*;

use crate::resources::output_event_sender::OutputEventSenderRes;

mod bindgen;
pub mod events;
mod resources;

#[wasm_bindgen]
pub struct SvgCompHandle {
    app: App,
    svg_builder_output_event_receiver: Receiver<SvgBuilderOutputEvent>,
    output_event_receiver: Receiver<SvgCompOutputEvent>,
}

#[wasm_bindgen]
impl SvgCompHandle {
    pub fn create(js_dtif: JsValue, interactive: bool) -> Result<SvgCompHandle, JsValue> {
        let dtif: DtifComp = serde_wasm_bindgen::from_value(js_dtif)?;
        let mut app = App::new();

        let (svg_builder_output_event_sender, svg_builder_output_event_receiver) =
            channel::<SvgBuilderOutputEvent>();
        let (output_event_sender, output_event_receiver) = channel::<SvgCompOutputEvent>();

        // Register plugins
        app.add_plugins((
            CompPlugin { dtif },
            CompSvgBuilderPlugin {
                output_event_sender: svg_builder_output_event_sender,
            },
        ));
        if interactive {
            app.add_plugins(CompInteractionPlugin);
        }

        // Register resources
        app.world
            .insert_resource(OutputEventSenderRes::new(output_event_sender));

        return Ok(Self {
            app,
            svg_builder_output_event_receiver,
            output_event_receiver,
        });
    }

    pub fn update(&mut self, js_input_events: JsValue) -> Result<JsValue, JsValue> {
        let maybe_input_events: Result<Vec<SvgCompInputEvent>, _> =
            serde_wasm_bindgen::from_value(js_input_events);

        // Emit input events in ECS world
        if let Ok(input_events) = maybe_input_events {
            for input_event in input_events {
                match input_event {
                    SvgCompInputEvent::Comp(event) => {
                        event.send_into_ecs(&mut self.app.world);
                    }
                    SvgCompInputEvent::Interaction(event) => {
                        event.send_into_ecs(&mut self.app.world);
                    }
                }
            }
        }

        // Advance the execution of the schedule by one cycle
        self.app.update();

        // Collect output events that were emitted during the last update cycle
        let mut output_events = Vec::new();
        while let Ok(event) = self.output_event_receiver.try_recv() {
            output_events.push(event);
        }
        while let Ok(event) = self.svg_builder_output_event_receiver.try_recv() {
            match event {
                SvgBuilderOutputEvent::ElementChanges(event) => {
                    output_events.push(SvgCompOutputEvent::ElementChanges(event))
                }
            }
        }

        return Ok(serde_wasm_bindgen::to_value(&output_events)?);
    }
}
