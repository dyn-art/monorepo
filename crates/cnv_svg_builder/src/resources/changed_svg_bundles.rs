#![cfg(feature = "output_svg_element_changes")]

use crate::svg::{svg_bundle::SvgBundle, svg_element::element_changes::SvgElementChanges};
use bevy_ecs::{entity::Entity, system::Resource};

#[derive(Resource, Debug, Default)]
pub struct ChangedSvgBundlesRes {
    changes: Vec<ChangedSvgBundle>,
    deferred_changes: Vec<SvgElementChanges>,
}

impl ChangedSvgBundlesRes {
    pub fn drain_changes(&mut self) -> Vec<ChangedSvgBundle> {
        self.changes.drain(..).collect()
    }

    pub fn push_change(&mut self, change: ChangedSvgBundle) {
        self.changes.push(change)
    }

    pub fn drain_deferred_changes(&mut self) -> Vec<SvgElementChanges> {
        self.deferred_changes.drain(..).collect()
    }

    pub fn push_deferred_change(&mut self, change: SvgElementChanges) {
        self.deferred_changes.push(change)
    }

    pub fn drain_removed_bundle_changes(&mut self, svg_bundle: &mut dyn SvgBundle) {
        let (deferred_changes, changes) = svg_bundle.drain_changes();
        self.deferred_changes.extend(changes);
        self.deferred_changes.extend(deferred_changes);
    }
}

#[derive(Debug, Clone)]
pub struct ChangedSvgBundle {
    pub entity: Entity,
    pub elements_changes: Vec<SvgElementChanges>,
    pub child_index: usize,
    pub hierarchy_level: u8,
}
