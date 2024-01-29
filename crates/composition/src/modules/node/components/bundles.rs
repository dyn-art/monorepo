use bevy_ecs::bundle::Bundle;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    mixins::{
        BlendMixin, ChildrenMixin, DimensionMixin, FillMixin, GradientStopsMixin,
        ImageContentMixin, NodeCompositionMixin, NodeMetaMixin, PaintCompositionMixin, PathMixin,
        RectangleCornerMixin, RelativeTransformMixin,
    },
    types::{
        FrameNode, GradientPaint, GroupNode, ImagePaint, Node, NodeType, Paint, PaintType,
        RectangleNode, SolidPaint, TextNode, VectorNode,
    },
};

// =============================================================================
// Node Bundle
// =============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum NodeBundle {
    Frame(FrameNodeBundle),
    Group(GroupNodeBundle),
    Rectangle(RectangleNodeBundle),
    Text(TextNodeBundle),
    Vector(VectorNodeBundle),
}

// =============================================================================
// Frame Node Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct FrameNodeBundle {
    #[serde(default = "default_frame_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub meta: NodeMetaMixin,

    #[serde(flatten)]
    pub frame: FrameNode,

    #[serde(default)]
    pub rectangle_corner_mixin: RectangleCornerMixin,

    #[serde(rename = "children")]
    pub children_mixin: ChildrenMixin,

    #[serde(default)]
    pub composition_mixin: NodeCompositionMixin,

    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension_mixin: DimensionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,

    #[serde(rename = "fill", default)]
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_frame_node_bundle() -> Node {
    Node {
        node_type: NodeType::Frame,
    }
}

impl Default for FrameNodeBundle {
    fn default() -> Self {
        Self {
            node: default_frame_node_bundle(),
            frame: FrameNode::default(),
            meta: NodeMetaMixin::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension_mixin: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}

// =============================================================================
// Group Node Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupNodeBundle {
    #[serde(default = "default_group_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub group: GroupNode,

    #[serde(flatten)]
    pub meta: NodeMetaMixin,

    #[serde(rename = "children")]
    pub children_mixin: ChildrenMixin,

    #[serde(default)]
    pub composition_mixin: NodeCompositionMixin,

    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension_mixin: DimensionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,
}

#[inline]
fn default_group_node_bundle() -> Node {
    Node {
        node_type: NodeType::Group,
    }
}

impl Default for GroupNodeBundle {
    fn default() -> Self {
        Self {
            node: default_group_node_bundle(),
            group: GroupNode::default(),
            meta: NodeMetaMixin::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension_mixin: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

// =============================================================================
// Rectangle Node Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct RectangleNodeBundle {
    #[serde(default = "default_rectangle_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub recangle: RectangleNode,

    #[serde(flatten)]
    pub meta: NodeMetaMixin,

    #[serde(default)]
    pub rectangle_corner_mixin: RectangleCornerMixin,

    #[serde(default)]
    pub composition_mixin: NodeCompositionMixin,

    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension_mixin: DimensionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,

    #[serde(rename = "fill", default)]
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_rectangle_node_bundle() -> Node {
    Node {
        node_type: NodeType::Rectangle,
    }
}

impl Default for RectangleNodeBundle {
    fn default() -> Self {
        Self {
            node: default_rectangle_node_bundle(),
            recangle: RectangleNode::default(),
            meta: NodeMetaMixin::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension_mixin: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}

// =============================================================================
// Text Node Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct TextNodeBundle {
    #[serde(default = "default_text_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub text: TextNode,

    #[serde(flatten)]
    pub meta: NodeMetaMixin,

    #[serde(default)]
    pub composition_mixin: NodeCompositionMixin,

    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension_mixin: DimensionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,

    #[serde(rename = "fill", default)]
    pub fill_mixin: FillMixin, // TODO: Needs adjustment to work with text segments
}

#[inline]
fn default_text_node_bundle() -> Node {
    Node {
        node_type: NodeType::Text,
    }
}

impl Default for TextNodeBundle {
    fn default() -> Self {
        Self {
            node: default_text_node_bundle(),
            text: TextNode::default(),
            meta: NodeMetaMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension_mixin: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}

// =============================================================================
// Vector Node Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct VectorNodeBundle {
    #[serde(default = "default_vector_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub vector: VectorNode,

    #[serde(flatten)]
    pub meta: NodeMetaMixin,

    #[serde(flatten)]
    pub path_mixin: PathMixin,

    #[serde(default)]
    pub composition_mixin: NodeCompositionMixin,

    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension_mixin: DimensionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,

    #[serde(rename = "fill", default)]
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_vector_node_bundle() -> Node {
    Node {
        node_type: NodeType::Vector,
    }
}

impl Default for VectorNodeBundle {
    fn default() -> Self {
        Self {
            node: default_rectangle_node_bundle(),
            vector: VectorNode::default(),
            meta: NodeMetaMixin::default(),
            path_mixin: PathMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension_mixin: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}

// =============================================================================
// Paint Bundle
// =============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum PaintBundle {
    Solid(SolidPaintBundle),
    Image(ImagePaintBundle),
    Gradient(GradientPaintBundle),
}

// =============================================================================
// Solid Paint Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct SolidPaintBundle {
    #[serde(default = "default_solid_paint_bundle")]
    pub paint: Paint,

    #[serde(flatten)]
    pub solid: SolidPaint,

    #[serde(default)]
    pub composition_mixin: PaintCompositionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,
}

#[inline]
fn default_solid_paint_bundle() -> Paint {
    Paint {
        paint_type: PaintType::Solid,
    }
}

impl Default for SolidPaintBundle {
    fn default() -> Self {
        Self {
            paint: default_solid_paint_bundle(),
            solid: SolidPaint::default(),
            composition_mixin: PaintCompositionMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

// =============================================================================
// Image Paint Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImagePaintBundle {
    #[serde(default = "default_image_paint_bundle")]
    pub paint: Paint,

    #[serde(flatten)]
    pub image: ImagePaint,

    pub image_content: ImageContentMixin,

    #[serde(default)]
    pub composition_mixin: PaintCompositionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,
}

#[inline]
fn default_image_paint_bundle() -> Paint {
    Paint {
        paint_type: PaintType::Image,
    }
}

impl Default for ImagePaintBundle {
    fn default() -> Self {
        Self {
            paint: default_image_paint_bundle(),
            image: ImagePaint::default(),
            image_content: ImageContentMixin::default(),
            composition_mixin: PaintCompositionMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

// =============================================================================
// Gradient Paint Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GradientPaintBundle {
    #[serde(default = "default_gradient_paint_bundle")]
    pub paint: Paint,

    #[serde(flatten)]
    pub gradient: GradientPaint,

    #[serde(flatten)]
    pub gradient_stops_mixin: GradientStopsMixin,

    #[serde(default)]
    pub composition_mixin: PaintCompositionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,
}

#[inline]
fn default_gradient_paint_bundle() -> Paint {
    Paint {
        paint_type: PaintType::Gradient,
    }
}

impl Default for GradientPaintBundle {
    fn default() -> Self {
        Self {
            paint: default_gradient_paint_bundle(),
            gradient: GradientPaint::default(),
            gradient_stops_mixin: GradientStopsMixin::default(),
            composition_mixin: PaintCompositionMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}
