use crate::{dtif_injector::DtifInjector, ToEcsBundleImpl};
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

impl ToEcsBundleImpl for FillStyle {
    type Bundle = FillStyleBundle;

    fn to_ecs_bundle(&self, dtif_injector: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
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

impl ToEcsBundleImpl for StrokeStyle {
    type Bundle = StrokeStyleBundle;

    fn to_ecs_bundle(&self, dtif_injector: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
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
