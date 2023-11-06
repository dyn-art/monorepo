use std::sync::mpsc::Sender;

use bevy_app::{App, Plugin};
use bevy_ecs::schedule::IntoSystemConfigs;
use dyn_bevy_render_skeleton::{ExtractSchedule, Render, RenderApp, RenderSet};
use dyn_composition::core::modules::node::components::mixins::{
    BlendMixin, ChildrenMixin, CompositionMixin, LayoutMixin, ParentMixin, PathMixin,
    RectangleCornerMixin,
};
use serde::Serialize;
use specta::Type;

use self::{
    resources::ChangedComponents,
    systems::{extract_mixin_generic, prepare_render_changes, queue_render_changes},
};

use super::output_event::{resources::OutputEventQueue, OutputEvent};

mod resources;
mod systems;

#[derive(Serialize, Clone, Debug, Type)]
pub enum RenderChange {
    RectangleCorner(RectangleCornerMixin),
    Children(ChildrenMixin),
    Layout(LayoutMixin),
    Composition(CompositionMixin),
    Blend(BlendMixin),
    Path(PathMixin),
    ParentMixin(ParentMixin),
}

pub trait ToRenderChange {
    fn to_render_change(&self) -> RenderChange;
}

impl ToRenderChange for ChildrenMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::Children(self.clone())
    }
}

impl ToRenderChange for LayoutMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::Layout(self.clone())
    }
}

impl ToRenderChange for CompositionMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::Composition(self.clone())
    }
}

impl ToRenderChange for BlendMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::Blend(self.clone())
    }
}

impl ToRenderChange for PathMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::Path(self.clone())
    }
}

impl ToRenderChange for RectangleCornerMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::RectangleCorner(self.clone())
    }
}

impl ToRenderChange for ParentMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::ParentMixin(self.clone())
    }
}

pub struct BindgenRenderPlugin {
    pub output_event_sender: Sender<OutputEvent>,
}

impl Plugin for BindgenRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        // Register resources
        render_app.init_resource::<ChangedComponents>();
        render_app.insert_resource(OutputEventQueue::new(self.output_event_sender.clone()));

        // Register systems
        render_app
            .add_systems(
                ExtractSchedule,
                (
                    extract_mixin_generic::<RectangleCornerMixin>,
                    extract_mixin_generic::<ChildrenMixin>,
                    extract_mixin_generic::<LayoutMixin>,
                    extract_mixin_generic::<CompositionMixin>,
                    extract_mixin_generic::<BlendMixin>,
                    extract_mixin_generic::<PathMixin>,
                    extract_mixin_generic::<ParentMixin>,
                ),
            )
            .add_systems(
                Render,
                (
                    prepare_render_changes.in_set(RenderSet::Prepare),
                    queue_render_changes.in_set(RenderSet::Queue),
                ),
            );
    }
}
