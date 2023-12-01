//! Events emitted by the Composition
//!
//! This module is responsible for defining the interface and mechanism through which
//! events are emitted from a `Composition`. Events can include state changes, errors,
//! or any other significant occurrences within the Composition that external entities
//! may be interested in responding to.

// Event emission is not implemented in the core Composition itself.
// Instead, the responsibility for emitting events lies with the specific implementations
// that extend the 'Composition'. This design decision allows each implementation to
// determine its own set of relevant events based on its unique behavior and requirements.
//
// To emit output events, set up a system using the Bevy_ECS
// and listen on desired component changes.
// Implement an event queue or similar resource to manage these events.
// For practical implementation examples, refer to the `dtom` package.
