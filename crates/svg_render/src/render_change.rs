use serde::Serialize;
use specta::Type;

use super::resources::svg_composition::svg_element::events::{
    AttributeRemoved, AttributeUpdated, ElementAppended, ElementCreated, ElementDeleted,
    StyleRemoved, StyleUpdated,
};

/// Represents the different types of events that can be emitted by a SVGElement
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
