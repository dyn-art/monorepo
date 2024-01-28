use std::sync::mpsc::Sender;

use bevy_ecs::system::Resource;

use crate::events::output_event::SVGRenderOutputEvent;

use self::svg_context::SVGContext;

pub mod bundles;
pub mod svg_bundle;
pub mod svg_context;
pub mod svg_element;

#[derive(Resource, Debug)]
pub struct SVGCompositionRes {
    pub context: SVGContext,
}

impl SVGCompositionRes {
    #[cfg(feature = "output-event")]
    pub fn new(output_event_sender: Option<Sender<SVGRenderOutputEvent>>) -> Self {
        SVGCompositionRes {
            context: SVGContext::new(output_event_sender),
        }
    }

    #[cfg(not(feature = "output-event"))]
    pub fn new() -> Self {
        SVGCompositionRes {
            context: SVGContext::new(),
        }
    }
}
