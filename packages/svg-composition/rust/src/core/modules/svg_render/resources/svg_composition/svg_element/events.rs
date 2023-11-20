use serde::Serialize;
use serde_with::serde_as;
use specta::Type;

use super::{attributes::SVGAttribute, styles::SVGStyle};

/// Represents the different types of events that can be emitted by SVGElement
/// to synchronize its state with the frontend.
///
/// Note on Child Element Management:
/// - Child elements are managed implicitly through their own lifecycle events rather than
///   explicit child addition or removal events.
/// - When a child element is created (`ElementCreated`), it includes an optional `parent_id`
///   indicating its parent. This way, the frontend knows to append this new child element
///   to the specified parent element.
/// - When a child element is deleted (`ElementDeleted`), it is responsible for removing itself
///   from the DOM. The parent element implicitly recognizes this removal.
/// - This approach avoids the need for separate `ChildAdded` or `ChildRemoved` events, simplifying
///   the event model and reducing the number of events needed to manage the DOM structure.
#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum RenderChange {
    ElementCreated(ElementCreated),
    ElementDeleted(ElementDeleted),
    ElementAppended(ElementAppended),
    AttributeUpdated(AttributeUpdated),
    AttributeRemoved(AttributeRemoved),
    StyleUpdated(StyleUpdated),
    StyleRemoved(StyleRemoved),
}

/// Emitted when a new SVGElement is created
#[serde_as]
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementCreated {
    #[serde(rename = "tagName")]
    pub tag_name: &'static str,
    pub attributes: Vec<SVGAttribute>,
    pub styles: Vec<SVGStyle>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<u32>, // Optional parent ID, if it's a child element
}

/// Emitted when a SVGElement is deleted
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementDeleted {}

/// Emitted when a SVGElement (child) is append to another SVGElement (parent)
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementAppended {
    #[serde(rename = "parentId")]
    pub parent_id: u32,
}

/// Emitted when an attribute of an SVGElement is updated
#[derive(Debug, Serialize, Clone, Type)]
pub struct AttributeUpdated {
    #[serde(rename = "newValue")]
    pub new_value: SVGAttribute,
}

/// Emitted when an attribute of a SVGElement is removed
#[derive(Debug, Serialize, Clone, Type)]
pub struct AttributeRemoved {
    key: &'static str,
}

/// Emitted when a style property of a SVGElement is updated
#[derive(Debug, Serialize, Clone, Type)]
pub struct StyleUpdated {
    #[serde(rename = "newValue")]
    pub new_value: SVGStyle,
}

/// Emitted when a style property of a SVGElement is removed
#[derive(Debug, Serialize, Clone, Type)]
pub struct StyleRemoved {
    key: &'static str,
}
