use crate::{
    common::{DtifFill, DtifStroke},
    ToEcsBundleImpl,
};
use bevy_transform::components::Transform;
use dyn_comp_types::{
    bundles::{FrameCompNodeBundle, GroupCompNodeBundle, RectangleCompNodeBundle},
    common::{BlendMode, CornerRadii, Opacity, Size, Visibility},
    mixins::{BlendModeMixin, CornerRadiiMixin, OpacityMixin, SizeMixin, VisibilityMixin},
    nodes::{CompNode, FrameCompNode, GroupCompNode, RectangleCompNode},
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
    pub clip_content: bool,
    pub translation: Vec2,
    pub angle_in_radians: f32,
    pub size: Size,
    pub corner_radii: CornerRadii,
    pub visibility: Visibility,
    pub fill: Vec<DtifFill>,
    pub blend_mode: BlendMode,
    pub opacity: Opacity,
    pub stroke: Vec<DtifStroke>,
    pub children: Vec<String>,
}

impl ToEcsBundleImpl for FrameNode {
    type Bundle = FrameCompNodeBundle;

    fn to_ecs_bundle(&self) -> Self::Bundle {
        FrameCompNodeBundle {
            node: CompNode::default(),
            frame: FrameCompNode {
                clip_content: self.clip_content,
            },
            transform: Transform {
                translation: Vec3::new(self.translation.x, self.translation.y, 0.0),
                rotation: Quat::from_rotation_z(self.angle_in_radians),
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
    pub angle_in_radians: f32,
    pub size: Size,
    pub visibility: Visibility,
    pub blend_mode: BlendMode,
    pub opacity: Opacity,
    pub children: Vec<String>,
}

impl ToEcsBundleImpl for GroupNode {
    type Bundle = GroupCompNodeBundle;

    fn to_ecs_bundle(&self) -> Self::Bundle {
        GroupCompNodeBundle {
            node: CompNode::default(),
            group: GroupCompNode,
            transform: Transform {
                translation: Vec3::new(self.translation.x, self.translation.y, 0.0),
                rotation: Quat::from_rotation_z(self.angle_in_radians),
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
    pub angle_in_radians: f32,
    pub size: Size,
    pub corner_radii: CornerRadii,
    pub visibility: Visibility,
    pub fill: Vec<DtifFill>,
    pub blend_mode: BlendMode,
    pub opacity: Opacity,
    pub stroke: Vec<DtifStroke>,
}

impl ToEcsBundleImpl for RectangleNode {
    type Bundle = RectangleCompNodeBundle;

    fn to_ecs_bundle(&self) -> Self::Bundle {
        RectangleCompNodeBundle {
            node: CompNode::default(),
            rectangle: RectangleCompNode::default(),
            transform: Transform {
                translation: Vec3::new(self.translation.x, self.translation.y, 0.0),
                rotation: Quat::from_rotation_z(self.angle_in_radians),
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
