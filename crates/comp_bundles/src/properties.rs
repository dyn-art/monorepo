use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use dyn_attributed_string::{
    dyn_fonts_book::font::{
        info::FontFamily,
        variant::{FontStretch, FontStyle, FontWeight},
    },
    text_attrs::{TextAttrs, TextAttrsInterval},
};
use dyn_comp_asset::asset_id::{AssetId, ImageId};
use dyn_utils::{
    properties::size::Size,
    units::{abs::Abs, font_unit::FontUnit},
};
use glam::Vec2;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct ReferenceId(String);

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum ReferenceIdOrEntity {
    Entity { value: Entity },
    ReferenceId { value: ReferenceId },
}

impl ReferenceIdOrEntity {
    pub fn get_entity(
        &self,
        reference_id_to_entity: &HashMap<ReferenceId, Entity>,
    ) -> Option<Entity> {
        match self {
            ReferenceIdOrEntity::Entity { value: entity } => Some(*entity),
            ReferenceIdOrEntity::ReferenceId {
                value: reference_id,
            } => reference_id_to_entity.get(reference_id).copied(),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum ReferenceIdOrImageId {
    ImageId { value: ImageId },
    ReferenceId { value: ReferenceId },
}

impl ReferenceIdOrImageId {
    pub fn get_image_id(
        &self,
        reference_id_to_asset_id: &HashMap<ReferenceId, AssetId>,
    ) -> Option<ImageId> {
        match self {
            ReferenceIdOrImageId::ImageId { value: image_id } => Some(*image_id),
            ReferenceIdOrImageId::ReferenceId {
                value: reference_id,
            } => {
                if let Some(asset_id) = reference_id_to_asset_id.get(reference_id) {
                    match asset_id {
                        AssetId::Image(image_id) => Some(*image_id),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct Viewport {
    pub physical_position: Vec2,
    pub physical_size: Size,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct TextAttributeInterval {
    pub start: usize,
    pub end: usize,
    pub attributes: TextAttributes,
}

impl TextAttributeInterval {
    pub fn to_attrs_interval(&self) -> TextAttrsInterval {
        TextAttrsInterval {
            start: self.start,
            stop: self.end,
            val: TextAttrs {
                font_id: None,
                font_family: self.attributes.font_family.clone(),
                font_style: self.attributes.font_style,
                font_stretch: self.attributes.font_stretch,
                font_weight: self.attributes.font_weight,
                font_size: self.attributes.font_size,
                small_caps: self.attributes.small_caps,
                apply_kerning: self.attributes.apply_kerning,
                letter_spacing: self.attributes.letter_spacing,
                word_spacing: self.attributes.word_spacing,
                line_height: self.attributes.line_height,
            },
        }
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct TextAttributes {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub font_family: Option<FontFamily>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub font_style: Option<FontStyle>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub font_stretch: Option<FontStretch>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub font_weight: Option<FontWeight>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub font_size: Option<Abs>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub small_caps: Option<bool>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub apply_kerning: Option<bool>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub letter_spacing: Option<FontUnit>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub word_spacing: Option<FontUnit>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub line_height: Option<FontUnit>,
}

/// Used to control how child nodes are aligned.
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum AlignItems {
    /// Items are packed toward the start of the axis
    #[default]
    Start,
    /// Items are packed toward the end of the axis
    End,
    /// Items are packed towards the flex-relative start of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to End. In all other cases it is equivalent to Start.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to Start. In all other cases it is equivalent to End.
    FlexEnd,
    /// Items are packed along the center of the cross axis
    Center,
    /// Items are aligned such as their baselines align
    Baseline,
    /// Stretch to fill the container
    Stretch,
}

impl From<AlignItems> for taffy::AlignItems {
    fn from(value: AlignItems) -> Self {
        match value {
            AlignItems::Start => taffy::AlignItems::Start.into(),
            AlignItems::End => taffy::AlignItems::End.into(),
            AlignItems::FlexStart => taffy::AlignItems::FlexStart.into(),
            AlignItems::FlexEnd => taffy::AlignItems::FlexEnd.into(),
            AlignItems::Center => taffy::AlignItems::Center.into(),
            AlignItems::Baseline => taffy::AlignItems::Baseline.into(),
            AlignItems::Stretch => taffy::AlignItems::Stretch.into(),
        }
    }
}

/// Used to control how child nodes are aligned.
/// Does not apply to Flexbox, and will be ignored if specified on a flex container
/// For Grid it controls alignment in the inline axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items)
pub type JustifyItems = AlignItems;

/// Used to control how the specified nodes is aligned.
/// Overrides the parent Node's `AlignItems` property.
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-self)
pub type AlignSelf = AlignItems;

/// Used to control how the specified nodes is aligned.
/// Overrides the parent Node's `JustifyItems` property.
/// Does not apply to Flexbox, and will be ignored if specified on a flex child
/// For Grid it controls alignment in the inline axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self)
pub type JustifySelf = AlignItems;

/// Sets the distribution of space between and around content items
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-content)
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum AlignContent {
    /// Items are packed toward the start of the axis
    #[default]
    Start,
    /// Items are packed toward the end of the axis
    End,
    /// Items are packed towards the flex-relative start of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to End. In all other cases it is equivalent to Start.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to Start. In all other cases it is equivalent to End.
    FlexEnd,
    /// Items are centered around the middle of the axis
    Center,
    /// Items are stretched to fill the container
    Stretch,
    /// The first and last items are aligned flush with the edges of the container (no gap)
    /// The gap between items is distributed evenly.
    SpaceBetween,
    /// The gap between the first and last items is exactly THE SAME as the gap between items.
    /// The gaps are distributed evenly
    SpaceEvenly,
    /// The gap between the first and last items is exactly HALF the gap between items.
    /// The gaps are distributed evenly in proportion to these ratios.
    SpaceAround,
}

impl From<AlignContent> for taffy::AlignContent {
    fn from(value: AlignContent) -> Self {
        match value {
            AlignContent::Start => taffy::AlignContent::Start.into(),
            AlignContent::End => taffy::AlignContent::End.into(),
            AlignContent::FlexStart => taffy::AlignContent::FlexStart.into(),
            AlignContent::FlexEnd => taffy::AlignContent::FlexEnd.into(),
            AlignContent::Center => taffy::AlignContent::Center.into(),
            AlignContent::Stretch => taffy::AlignContent::Stretch.into(),
            AlignContent::SpaceBetween => taffy::AlignContent::SpaceBetween.into(),
            AlignContent::SpaceAround => taffy::AlignContent::SpaceAround.into(),
            AlignContent::SpaceEvenly => taffy::AlignContent::SpaceEvenly.into(),
        }
    }
}

/// Sets the distribution of space between and around content items
/// For Flexbox it controls alignment in the main axis
/// For Grid it controls alignment in the inline axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content)
pub type JustifyContent = AlignContent;

/// The direction of the flexbox layout main axis.
///
/// There are always two perpendicular layout axes: main (or primary) and cross (or secondary).
/// Adding items will cause them to be positioned adjacent to each other along the main axis.
/// By varying this value throughout your tree, you can create complex axis-aligned layouts.
///
/// Items are always aligned relative to the cross axis, and justified relative to the main axis.
///
/// The default behavior is [`FlexDirection::Row`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-direction-property)
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum FlexDirection {
    /// Defines +x as the main axis
    ///
    /// Items will be added from left to right in a row.
    #[default]
    Row,
    /// Defines +y as the main axis
    ///
    /// Items will be added from top to bottom in a column.
    Column,
    /// Defines -x as the main axis
    ///
    /// Items will be added from right to left in a row.
    RowReverse,
    /// Defines -y as the main axis
    ///
    /// Items will be added from bottom to top in a column.
    ColumnReverse,
}

impl From<FlexDirection> for taffy::FlexDirection {
    fn from(value: FlexDirection) -> Self {
        match value {
            FlexDirection::Row => taffy::FlexDirection::Row,
            FlexDirection::Column => taffy::FlexDirection::Column,
            FlexDirection::RowReverse => taffy::FlexDirection::RowReverse,
            FlexDirection::ColumnReverse => taffy::FlexDirection::ColumnReverse,
        }
    }
}
