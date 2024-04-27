use crate::{
    conversion::string_to_tiny_skia_path, dtif_handler::DtifHandler, styles::Style, SpawnBundleImpl,
};
use bevy_ecs::world::{EntityWorldMut, World};
use bevy_transform::{components::Transform, TransformBundle};
use dyn_attributed_string::{HorizontalTextAlignment, LineWrap, VerticalTextAlignment};
use dyn_comp_bundles::{
    components::{
        mixins::{
            BlendMode, BlendModeMixin, Constraints, ConstraintsMixin, CornerRadiiMixin,
            OpacityMixin, PathMixin, SizeMixin, VisibilityMixin, WindingRule,
        },
        nodes::{
            CompNode, CompNodeVariant, EllipseArcData, EllipseCompNode, FrameCompNode,
            PolygonCompNode, RectangleCompNode, StarCompNode, TextCompNode, VectorCompNode,
        },
    },
    properties::TextAttributeInterval,
    EllipseCompNodeBundle, FrameCompNodeBundle, PolygonCompNodeBundle, RectangleCompNodeBundle,
    StarCompNodeBundle, TextCompNodeBundle, VectorNodeBundle,
};
use dyn_utils::{
    properties::{corner_radii::CornerRadii, opacity::Opacity, size::Size},
    serde::{default_as_false, default_as_true},
    units::angle::Angle,
};
use glam::{Quat, Vec2, Vec3};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum Node {
    Frame(FrameNode),
    Rectangle(RectangleNode),
    Ellipse(EllipseNode),
    Star(StarNode),
    Polygon(PolygonNode),
    Text(TextNode),
    Vector(VectorNode),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FrameNode {
    #[serde(default = "default_as_false")]
    pub clip_content: bool,
    #[serde(default)]
    pub translation: Vec2,
    #[serde(default)]
    pub rotation_deg: Angle,
    pub size: Size,
    #[serde(default)]
    pub corner_radii: CornerRadii,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub constraints: Constraints,
    #[serde(default)]
    pub styles: Vec<Style>,
    #[serde(default)]
    pub children: Vec<String>,
}

impl FrameNode {
    fn to_ecs_bundle(&self) -> FrameCompNodeBundle {
        FrameCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Frame,
            },
            frame: FrameCompNode {
                clip_content: self.clip_content,
            },
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: Quat::from_rotation_z(self.rotation_deg.to_rad()),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            corner_radii: CornerRadiiMixin(self.corner_radii),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
            constraints: ConstraintsMixin(self.constraints),
        }
    }
}

impl SpawnBundleImpl for FrameNode {
    fn spawn<'a>(&self, _: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RectangleNode {
    #[serde(default)]
    pub translation: Vec2,
    #[serde(default)]
    pub rotation_deg: Angle,
    pub size: Size,
    #[serde(default)]
    pub corner_radii: CornerRadii,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub constraints: Constraints,
    #[serde(default)]
    pub styles: Vec<Style>,
}

impl RectangleNode {
    fn to_ecs_bundle(&self) -> RectangleCompNodeBundle {
        RectangleCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Rectangle,
            },
            rectangle: RectangleCompNode::default(),
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: Quat::from_rotation_z(self.rotation_deg.to_rad()),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            corner_radii: CornerRadiiMixin(self.corner_radii),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
            constraints: ConstraintsMixin(self.constraints),
        }
    }
}

impl SpawnBundleImpl for RectangleNode {
    fn spawn<'a>(&self, _: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct EllipseNode {
    #[serde(default)]
    pub starting_angle: f32,
    #[serde(default)]
    pub ending_angle: f32,
    #[serde(default)]
    pub inner_radius_ratio: f32,
    #[serde(default)]
    pub translation: Vec2,
    #[serde(default)]
    pub rotation_deg: Angle,
    pub size: Size,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub constraints: Constraints,
    #[serde(default)]
    pub styles: Vec<Style>,
}

impl EllipseNode {
    fn to_ecs_bundle(&self) -> EllipseCompNodeBundle {
        EllipseCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Ellipse,
            },
            ellipse: EllipseCompNode {
                arc_data: EllipseArcData {
                    starting_angle: self.starting_angle,
                    ending_angle: self.ending_angle,
                    inner_radius_ratio: self.inner_radius_ratio,
                },
            },
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: Quat::from_rotation_z(self.rotation_deg.to_rad()),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
            constraints: ConstraintsMixin(self.constraints),
        }
    }
}

impl SpawnBundleImpl for EllipseNode {
    fn spawn<'a>(&self, _: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct StarNode {
    #[serde(default)]
    pub inner_radius_ratio: f32,
    #[serde(default = "default_star_point_count")]
    pub point_count: u8,
    #[serde(default)]
    pub translation: Vec2,
    #[serde(default)]
    pub rotation_deg: Angle,
    pub size: Size,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub constraints: Constraints,
    #[serde(default)]
    pub styles: Vec<Style>,
}

impl StarNode {
    fn to_ecs_bundle(&self) -> StarCompNodeBundle {
        StarCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Star,
            },
            star: StarCompNode {
                inner_radius_ratio: self.inner_radius_ratio,
                point_count: self.point_count,
            },
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: Quat::from_rotation_z(self.rotation_deg.to_rad()),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
            constraints: ConstraintsMixin(self.constraints),
        }
    }
}

impl SpawnBundleImpl for StarNode {
    fn spawn<'a>(&self, _: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle())
    }
}

#[inline]
fn default_star_point_count() -> u8 {
    5
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PolygonNode {
    #[serde(default = "default_polygon_point_count")]
    pub point_count: u8,
    #[serde(default)]
    pub translation: Vec2,
    #[serde(default)]
    pub rotation_deg: Angle,
    pub size: Size,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub constraints: Constraints,
    #[serde(default)]
    pub styles: Vec<Style>,
}

impl PolygonNode {
    fn to_ecs_bundle(&self) -> PolygonCompNodeBundle {
        PolygonCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Polygon,
            },
            polygon: PolygonCompNode {
                point_count: self.point_count,
            },
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: Quat::from_rotation_z(self.rotation_deg.to_rad()),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
            constraints: ConstraintsMixin(self.constraints),
        }
    }
}

impl SpawnBundleImpl for PolygonNode {
    fn spawn<'a>(&self, _: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle())
    }
}

#[inline]
fn default_polygon_point_count() -> u8 {
    3
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TextNode {
    pub text: String,
    pub attributes: Vec<TextAttributeInterval>,
    #[serde(default)]
    pub line_wrap: LineWrap,
    #[serde(default)]
    pub horizontal_text_alignment: HorizontalTextAlignment,
    #[serde(default)]
    pub vertical_text_alignment: VerticalTextAlignment,
    #[serde(default)]
    pub translation: Vec2,
    #[serde(default)]
    pub rotation_deg: Angle,
    pub size: Size,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub constraints: Constraints,
    #[serde(default)]
    pub styles: Vec<Style>,
}

impl TextNode {
    fn to_ecs_bundle(&self) -> TextCompNodeBundle {
        TextCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Text,
            },
            text: TextCompNode {
                text: self.text.clone(),
                attributes: self.attributes.iter().cloned().collect(),
                line_wrap: self.line_wrap,
                horizontal_text_alignment: self.horizontal_text_alignment,
                vertical_text_alignment: self.vertical_text_alignment,
            },
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: Quat::from_rotation_z(self.rotation_deg.to_rad()),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
            constraints: ConstraintsMixin(self.constraints),
        }
    }
}

impl SpawnBundleImpl for TextNode {
    fn spawn<'a>(&self, _: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct VectorNode {
    path: String,
    #[serde(default)]
    winding_rule: WindingRule,
    #[serde(default)]
    pub translation: Vec2,
    #[serde(default)]
    pub rotation_deg: Angle,
    pub size: Size,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub constraints: Constraints,
    #[serde(default)]
    pub styles: Vec<Style>,
}

impl VectorNode {
    fn to_ecs_bundle(&self) -> VectorNodeBundle {
        VectorNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Vector,
            },
            path: PathMixin {
                path: string_to_tiny_skia_path(&self.path).unwrap(),
                winding_rule: self.winding_rule,
            },
            vector: VectorCompNode,
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: Quat::from_rotation_z(self.rotation_deg.to_rad()),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
            constraints: ConstraintsMixin(self.constraints),
        }
    }
}

impl SpawnBundleImpl for VectorNode {
    fn spawn<'a>(&self, _: &DtifHandler, world: &'a mut World) -> EntityWorldMut<'a> {
        world.spawn(self.to_ecs_bundle())
    }
}
