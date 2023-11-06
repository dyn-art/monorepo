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
