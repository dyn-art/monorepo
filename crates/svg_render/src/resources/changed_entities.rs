use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Resource};

use crate::mixin_change::MixinChange;

#[derive(Resource, Debug, Default)]
pub struct ChangedEntitiesRes {
    pub changed_entities: HashMap<Entity, ChangedEntity>,
}

#[derive(Debug, Clone)]
pub struct ChangedEntity {
    pub entity: Entity,
    pub entity_type: ChangedEntityType,
    pub parent_id: Option<Entity>,
    pub changes: Vec<MixinChange>,
}

#[derive(Debug, Clone, Copy)]
pub enum ChangedEntityType {
    ShapeNode,
    FrameNode,
    SolidPaint,
    ImagePaint(ChangedEntityImagePaintType),
    GradientPaint(ChangedEntityGradientPaintType),
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum ChangedEntityGradientPaintType {
    Linear,
    Radial,
}

#[derive(Debug, Clone, Copy)]
pub enum ChangedEntityImagePaintType {
    Fill,
    Fit,
    Crop,
    Tile,
}
