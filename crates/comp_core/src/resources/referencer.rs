use bevy_ecs::{entity::Entity, system::Resource};
use dyn_comp_asset::asset_id::AssetId;
use dyn_comp_bundles::reference_id::ReferenceId;
use std::collections::HashMap;

#[derive(Resource, Default, Debug)]
pub struct ReferencerRes {
    reference_id_to_entity: HashMap<ReferenceId, Entity>,
    reference_id_to_asset_id: HashMap<ReferenceId, AssetId>,
}

impl ReferencerRes {
    pub fn new() -> Self {
        Self {
            reference_id_to_entity: HashMap::new(),
            reference_id_to_asset_id: HashMap::new(),
        }
    }

    pub fn get_reference_id_to_entity_map(&self) -> &HashMap<ReferenceId, Entity> {
        &self.reference_id_to_entity
    }

    pub fn get_reference_id_to_asset_id_map(&self) -> &HashMap<ReferenceId, AssetId> {
        &self.reference_id_to_asset_id
    }

    pub fn reference_entity(&mut self, reference_id: ReferenceId, entity: Entity) {
        self.reference_id_to_entity.insert(reference_id, entity);
    }

    // Adds a new reference_id to asset_id mapping
    pub fn reference_asset_id(&mut self, reference_id: ReferenceId, asset_id: AssetId) {
        self.reference_id_to_asset_id.insert(reference_id, asset_id);
    }

    pub fn get_entity(&self, reference_id: &ReferenceId) -> Option<&Entity> {
        self.reference_id_to_entity.get(reference_id)
    }

    pub fn get_asset_id(&self, reference_id: &ReferenceId) -> Option<&AssetId> {
        self.reference_id_to_asset_id.get(reference_id)
    }

    pub fn remove_entity(&mut self, reference_id: &ReferenceId) {
        self.reference_id_to_entity.remove(reference_id);
    }

    pub fn remove_asset_id(&mut self, reference_id: &ReferenceId) {
        self.reference_id_to_asset_id.remove(reference_id);
    }
}
