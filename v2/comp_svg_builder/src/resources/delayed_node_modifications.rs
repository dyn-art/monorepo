use bevy_ecs::{entity::Entity, system::Resource};

/// A resource to store node modifications that cannot be applied immediately due to ECS restrictions.
/// It addresses the issue of query conflicts that arise from attempting to access `SvgNodeVariant` components mutably multiple times within the same system.
#[derive(Resource, Default, Debug)]
pub struct DelayedNodeModificationsRes {
    pub children_modifications: Vec<SvgNodeChildrenModification>,
}

#[derive(Debug, Clone)]
pub struct SvgNodeChildrenModification {
    pub parent_entity: Entity,
    pub added_entities: Vec<Entity>,
    pub removed_entities: Vec<Entity>,
}
