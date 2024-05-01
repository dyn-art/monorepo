use bevy_ecs::{component::Component, entity::Entity};
use bevy_transform::components::Transform;
use dyn_attributed_string::AttributedString;
use dyn_comp_asset::asset_id::ImageId;
use dyn_utils::properties::{corner_radii::CornerRadii, opacity::Opacity, size::Size};
use smallvec::SmallVec;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct HierarchyLevel(pub u8);

/// Represents an entity's dimensions with width and height.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct SizeMixin(pub Size);

/// Defines corner radii for rectangular entities, specifying each corner's radius.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct CornerRadiiMixin(pub CornerRadii);

/// Specifies an entity's blend mode for color blending with underlying colors.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct BlendModeMixin(pub BlendMode);

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum BlendMode {
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

/// Controls an entity's visibility state.
#[derive(Component, Debug, Copy, Clone)]
pub struct VisibilityMixin(pub bool);

impl Default for VisibilityMixin {
    fn default() -> Self {
        Self(true)
    }
}

/// Controls the opacity of an entity, ranging from 0.0 (fully transparent) to 1.0 (fully opaque).
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct OpacityMixin(pub Opacity);

/// Represents a Bezier path for drawing shape.
#[derive(Component, Debug, Clone)]
pub struct PathMixin {
    pub path: tiny_skia_path::Path,
    pub winding_rule: WindingRule,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum WindingRule {
    #[default]
    Nonzero,
    Evenodd,
}

/// Configures stroke properties for drawing paths.
#[derive(Component, Debug, Clone)]
pub struct StrokePathMixin {
    pub path: tiny_skia_path::Path,
    pub winding_rule: WindingRule,
}

#[derive(Component, Debug, Default, Clone)]
pub struct StyleChildrenMixin(pub SmallVec<[Entity; 2]>);

#[derive(Component, Debug, Clone)]
pub struct StyleParentMixin(pub Entity);

#[derive(Component, Debug, Clone)]
pub struct PaintChildMixin(pub Option<Entity>);

#[derive(Component, Debug, Default, Clone)]
pub struct PaintParentMixin(pub SmallVec<[Entity; 2]>);

#[derive(Component, Debug, Default, Clone)]
pub struct ImageAssetMixin(pub Option<ImageId>);

#[derive(Component, Debug, Clone)]
pub struct AttributedStringMixin(pub AttributedString);

#[derive(Component, Debug, Copy, Clone)]
pub struct LayoutNodeId(pub taffy::NodeId);

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct LayoutParentMixin(pub LayoutParent);

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct LayoutParent {
    // TODO
}

impl LayoutParent {
    pub fn to_style(&self) -> taffy::Style {
        taffy::Style::default()
    }
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct LayoutElementMixin(pub LayoutElement);

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum LayoutElement {
    Absolute(AbsoluteLayoutElement),
    Static(StaticLayoutElement),
}

impl Default for LayoutElement {
    fn default() -> Self {
        Self::Absolute(AbsoluteLayoutElement::default())
    }
}

impl LayoutElement {
    pub fn to_style(
        &self,
        entity: Entity, // TODO: REMOVE
        transform: &Transform,
        size: &Size,
        parent_size: Option<&Size>,
    ) -> taffy::Style {
        match self {
            LayoutElement::Absolute(element) => {
                element.to_style(entity, transform, size, parent_size)
            }
            LayoutElement::Static(element) => element.to_style(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct AbsoluteLayoutElement {
    pub constraints: Constraints,
}

impl AbsoluteLayoutElement {
    pub fn to_style(
        &self,
        entity: Entity, // TODO: REMOVE
        transform: &Transform,
        size: &Size,
        parent_size: Option<&Size>,
    ) -> taffy::Style {
        let mut style = taffy::Style::default();

        log::info!(
            "[AbsoluteLayoutElement::to_style] {:?}: {:?}, {:?} | Parent: {:?}",
            entity,
            transform,
            size,
            parent_size
        ); // TODO: REMOVE

        // Set the position type to absolute
        style.position = taffy::Position::Absolute;

        // Default insets
        let mut top = taffy::LengthPercentageAuto::Auto;
        let mut bottom = taffy::LengthPercentageAuto::Auto;
        let mut left = taffy::LengthPercentageAuto::Auto;
        let mut right = taffy::LengthPercentageAuto::Auto;

        // Adjust horizontal insets based on the horizontal constraint
        match self.constraints.horizontal {
            Constraint::Start => {
                left = taffy::LengthPercentageAuto::Length(transform.translation.x);
                right = taffy::LengthPercentageAuto::Auto;
            }
            Constraint::Center => {
                // TODO
            }
            Constraint::End => {
                left = taffy::LengthPercentageAuto::Auto;
                right = taffy::LengthPercentageAuto::Length(
                    parent_size.unwrap().width() - transform.translation.x,
                );
            }
            Constraint::Stretch | Constraint::Scale => {
                // TODO
            }
        }

        // Adjust vertical insets based on the vertical constraint
        match self.constraints.vertical {
            Constraint::Start => {
                top = taffy::LengthPercentageAuto::Length(transform.translation.y);
                bottom = taffy::LengthPercentageAuto::Auto;
            }
            Constraint::Center => {
                // TODO
            }
            Constraint::End => {
                top = taffy::LengthPercentageAuto::Auto;
                bottom = taffy::LengthPercentageAuto::Length(
                    parent_size.unwrap().height() - transform.translation.y,
                );
            }
            Constraint::Stretch | Constraint::Scale => {
                // TODO
            }
        }

        style.inset = taffy::Rect {
            top,
            bottom,
            left,
            right,
        };

        // Set the basic size, unless overridden by Scale
        if self.constraints.horizontal != Constraint::Scale
            && self.constraints.vertical != Constraint::Scale
        {
            style.size = taffy::Size {
                width: taffy::Dimension::Length(size.width()),
                height: taffy::Dimension::Length(size.height()),
            };
        }

        log::info!("[AbsoluteLayoutElement::to_style] Style: {:?}", style); // TODO: REMOVE

        return style;
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Constraints {
    pub horizontal: Constraint,
    pub vertical: Constraint,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum Constraint {
    #[default]
    Start,
    Center,
    End,
    Stretch,
    Scale,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct StaticLayoutElement {
    // padding, ..
}

impl StaticLayoutElement {
    pub fn to_style(&self) -> taffy::Style {
        taffy::Style::default()
    }
}
