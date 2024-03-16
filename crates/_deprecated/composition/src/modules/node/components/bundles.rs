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
        EllipseNode, FrameNode, GradientPaint, GroupNode, ImagePaint, Node, NodeType, Paint,
        PaintType, PolygonNode, RectangleNode, SolidPaint, StarNode, TextNode, VectorNode,
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
    Polygon(PolygonNodeBundle),
    Ellipse(EllipseNodeBundle),
    Star(StarNodeBundle),
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

// =============================================================================
// Polygon Node Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct PolygonNodeBundle {
    #[serde(default = "default_polygon_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub polygon: PolygonNode,

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
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_polygon_node_bundle() -> Node {
    Node {
        node_type: NodeType::Polygon,
    }
}

// =============================================================================
// Ellipse Node Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct EllipseNodeBundle {
    #[serde(default = "default_ellipse_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub ellipse: EllipseNode,

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
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_ellipse_node_bundle() -> Node {
    Node {
        node_type: NodeType::Ellipse,
    }
}

// =============================================================================
// Star Node Bundle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct StarNodeBundle {
    #[serde(default = "default_star_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub star: StarNode,

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
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_star_node_bundle() -> Node {
    Node {
        node_type: NodeType::Star,
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
