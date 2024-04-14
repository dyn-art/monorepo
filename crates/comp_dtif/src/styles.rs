use crate::{dtif_injector::DtifInjector, SpawnBundleImpl};
use bevy_ecs::world::{EntityWorldMut, World};
use dyn_comp_bundles::{
    components::{
        mixins::{BlendMode, BlendModeMixin, OpacityMixin, PaintChildMixin, VisibilityMixin},
        styles::{CompStyle, CompStyleVariant, FillCompStyle, StrokeCompStyle},
    },
    FillStyleBundle, StrokeStyleBundle,
};
use dyn_utils::{properties::opacity::Opacity, serde::default_as_true};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum Style {
    Fill(FillStyle),
    Stroke(StrokeStyle),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FillStyle {
    pub paint_id: String,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
}

impl FillStyle {
    fn to_ecs_bundle(&self, dtif_injector: &DtifInjector) -> FillStyleBundle {
        FillStyleBundle {
            style: CompStyle {
                variant: CompStyleVariant::Fill,
            },
            fill: FillCompStyle,
            paint: PaintChildMixin(
                dtif_injector
                    .get_sid_to_entity()
                    .get(&self.paint_id)
                    .copied(),
            ),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }
}

impl SpawnBundleImpl for FillStyle {
    fn spawn<'a>(&self, dtif_injector: &DtifInjector, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle(dtif_injector))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct StrokeStyle {
    width: f32,
    pub paint_id: String,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
}

impl StrokeStyle {
    fn to_ecs_bundle(&self, dtif_injector: &DtifInjector) -> StrokeStyleBundle {
        StrokeStyleBundle {
            style: CompStyle {
                variant: CompStyleVariant::Stroke,
            },
            stroke: StrokeCompStyle {
                stroke: tiny_skia_path::Stroke {
                    width: self.width,
                    ..Default::default()
                },
            },
            paint: PaintChildMixin(
                dtif_injector
                    .get_sid_to_entity()
                    .get(&self.paint_id)
                    .copied(),
            ),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }
}

impl SpawnBundleImpl for StrokeStyle {
    fn spawn<'a>(&self, dtif_injector: &DtifInjector, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle(dtif_injector))
    }
}
