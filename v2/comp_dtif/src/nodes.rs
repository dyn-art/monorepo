use crate::{
    common::{DtifFill, DtifStroke},
    dtif_injector::DtifInjector,
    ToEcsBundleImpl,
};
use bevy_transform::components::Transform;
use dyn_comp_common::{
    bundles::{FrameCompNodeBundle, GroupCompNodeBundle, RectangleCompNodeBundle},
    common::{BlendMode, CornerRadii, Degree, Opacity, Size, Visibility},
    mixins::{BlendModeMixin, CornerRadiiMixin, OpacityMixin, SizeMixin, VisibilityMixin},
    nodes::{CompNode, CompNodeVariant, FrameCompNode, GroupCompNode, RectangleCompNode},
};
use glam::{Quat, Vec2, Vec3};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum Node {
    Frame(FrameNode),
    Group(GroupNode),
    Rectangle(RectangleNode),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FrameNode {
    #[serde(default = "default_as_false")]
    pub clip_content: bool,
    pub translation: Vec2,
    #[serde(default)]
    pub angle: Degree,
    pub size: Size,
    #[serde(default)]
    pub corner_radii: CornerRadii,
    #[serde(default)]
    pub visibility: Visibility,
    pub fill: Vec<DtifFill>,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub stroke: Vec<DtifStroke>,
    #[serde(default)]
    pub children: Vec<String>,
}

#[inline]
fn default_as_false() -> bool {
    false
}

impl ToEcsBundleImpl for FrameNode {
    type Bundle = FrameCompNodeBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        FrameCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Frame,
            },
            frame: FrameCompNode {
                clip_content: self.clip_content,
            },
            transform: Transform {
                translation: Vec3::new(self.translation.x, self.translation.y, 0.0),
                rotation: Quat::from_rotation_z(self.angle.to_radians()),
                scale: Vec3::default(),
            },
            size: SizeMixin(self.size),
            corner_radii: CornerRadiiMixin(self.corner_radii),
            visibility: VisibilityMixin(self.visibility),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupNode {
    pub translation: Vec2,
    #[serde(default)]
    pub angle: Degree,
    pub size: Size,
    #[serde(default)]
    pub visibility: Visibility,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub children: Vec<String>,
}

impl ToEcsBundleImpl for GroupNode {
    type Bundle = GroupCompNodeBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        GroupCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Group,
            },
            group: GroupCompNode,
            transform: Transform {
                translation: Vec3::new(self.translation.x, self.translation.y, 0.0),
                rotation: Quat::from_rotation_z(self.angle.to_radians()),
                scale: Vec3::default(),
            },
            visibility: VisibilityMixin(self.visibility),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RectangleNode {
    pub translation: Vec2,
    #[serde(default)]
    pub angle: Degree,
    pub size: Size,
    #[serde(default)]
    pub corner_radii: CornerRadii,
    #[serde(default)]
    pub visibility: Visibility,
    pub fill: Vec<DtifFill>,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub opacity: Opacity,
    #[serde(default)]
    pub stroke: Vec<DtifStroke>,
}

impl ToEcsBundleImpl for RectangleNode {
    type Bundle = RectangleCompNodeBundle;

    fn to_ecs_bundle(&self, _: &DtifInjector) -> Self::Bundle {
        RectangleCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Rectangle,
            },
            rectangle: RectangleCompNode::default(),
            transform: Transform {
                translation: Vec3::new(self.translation.x, self.translation.y, 0.0),
                rotation: Quat::from_rotation_z(self.angle.to_radians()),
                scale: Vec3::default(),
            },
            size: SizeMixin(self.size),
            corner_radii: CornerRadiiMixin(self.corner_radii),
            visibility: VisibilityMixin(self.visibility),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }
}
