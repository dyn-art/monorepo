use crate::{
    conversion::string_to_tiny_skia_path, dtif_injector::DtifInjector, styles::Style,
    ToEcsBundleImpl,
};
use bevy_transform::{components::Transform, TransformBundle};
use dyn_attributed_string::{HorizontalTextAlignment, LineWrap, VerticalTextAlignment};
use dyn_comp_bundles::{
    components::{
        mixins::{
            BlendMode, BlendModeMixin, Constraints, ConstraintsMixin, CornerRadiiMixin,
            GroupConstraints, GroupConstraintsMixin, OpacityMixin, PathMixin, SizeMixin,
            VisibilityMixin,
        },
        nodes::{
            CompNode, CompNodeVariant, EllipseArcData, EllipseCompNode, FrameCompNode,
            GroupCompNode, PolygonCompNode, RectangleCompNode, StarCompNode, TextCompNode,
            VectorCompNode,
        },
    },
    properties::TextAttributeInterval,
    EllipseCompNodeBundle, FrameCompNodeBundle, GroupCompNodeBundle, PolygonCompNodeBundle,
    RectangleCompNodeBundle, StarCompNodeBundle, TextCompNodeBundle, VectorNodeBundle,
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
    Group(GroupNode),
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

impl ToEcsBundleImpl for FrameNode {
    type Bundle = FrameCompNodeBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupNode {
    #[serde(default)]
    pub translation: Option<Vec2>,
    #[serde(default)]
    pub rotation_deg: Option<Angle>,
    #[serde(default)]
    pub size: Option<Size>,
    #[serde(default = "default_as_true")]
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub constraints: Option<Constraints>,
    #[serde(default)]
    pub children: Vec<String>,
}

impl ToEcsBundleImpl for GroupNode {
    type Bundle = GroupCompNodeBundle;

    // TODO: Either I create custom Components for each Group component
    // like Transform where I've three states:
    // -> Derived from Children
    // -> Explicitly set
    // -> Not set (yet)
    //
    // or I add additional component like GroupTransformStatus
    // which then hold the above mentioned states
    // I feel like the approach with separate component is better,
    // because then I don't have to create own Transform component and stuff
    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
            node: CompNode {
                variant: CompNodeVariant::Group,
            },
            group: GroupCompNode,
            transform: TransformBundle::from_transform(Transform {
                translation: self
                    .translation
                    .map(|translation| translation.extend(0.0))
                    .unwrap_or_default(),
                rotation: self
                    .rotation_deg
                    .map(|rotation_deg| Quat::from_rotation_z(rotation_deg.to_rad()))
                    .unwrap_or_default(),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size.unwrap_or_default()),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
            constraints: GroupConstraintsMixin(match self.constraints {
                Some(constraints) => GroupConstraints::Constraints(constraints),
                None => GroupConstraints::Mixed,
            }),
        }
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

impl ToEcsBundleImpl for RectangleNode {
    type Bundle = RectangleCompNodeBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
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

impl ToEcsBundleImpl for EllipseNode {
    type Bundle = EllipseCompNodeBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
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

impl ToEcsBundleImpl for StarNode {
    type Bundle = StarCompNodeBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
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

impl ToEcsBundleImpl for PolygonNode {
    type Bundle = PolygonCompNodeBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
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

impl ToEcsBundleImpl for TextNode {
    type Bundle = TextCompNodeBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct VectorNode {
    path: String,
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

impl ToEcsBundleImpl for VectorNode {
    type Bundle = VectorNodeBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        Self::Bundle {
            node: CompNode {
                variant: CompNodeVariant::Vector,
            },
            path: PathMixin(string_to_tiny_skia_path(&self.path).unwrap()),
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
