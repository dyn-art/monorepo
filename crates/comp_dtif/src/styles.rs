use crate::{dtif_handler::DtifHandler, SpawnBundleImpl};
use bevy_ecs::world::{EntityWorldMut, World};
use dyn_comp_bundles::{
    components::{
        mixins::{BlendMode, BlendModeMixin, OpacityMixin, PaintChildMixin, VisibilityMixin},
        styles::{
            CompStyle, CompStyleVariant, DropShadowCompStyle, FillCompStyle, StrokeCompStyle,
        },
    },
    DropShadowStyleBundle, FillStyleBundle, StrokeStyleBundle,
};
use dyn_utils::{
    properties::{color::Color, opacity::Opacity},
    serde::default_as_true,
    units::abs::Abs,
};
use glam::Vec2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum Style {
    Fill(FillStyle),
    Stroke(StrokeStyle),
    DropShadow(DropShadowStyle),
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
    fn to_ecs_bundle(&self, dtif_injector: &DtifHandler) -> FillStyleBundle {
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
    fn spawn<'a>(&self, dtif_injector: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
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
    fn to_ecs_bundle(&self, dtif_injector: &DtifHandler) -> StrokeStyleBundle {
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
    fn spawn<'a>(&self, dtif_injector: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle(dtif_injector))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DropShadowStyle {
    #[serde(default)]
    pub color: Color,
    pub position: Vec2,
    #[serde(default)]
    pub spread: Abs,
    pub blur: Abs,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
}

impl DropShadowStyle {
    fn to_ecs_bundle(&self, _: &DtifHandler) -> DropShadowStyleBundle {
        DropShadowStyleBundle {
            style: CompStyle {
                variant: CompStyleVariant::DropShadow,
            },
            dorp_shadow: DropShadowCompStyle {
                color: self.color,
                position: self.position,
                spread: self.spread,
                blur: self.blur,
            },
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }
}

impl SpawnBundleImpl for DropShadowStyle {
    fn spawn<'a>(&self, dtif_injector: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle(dtif_injector))
    }
}
