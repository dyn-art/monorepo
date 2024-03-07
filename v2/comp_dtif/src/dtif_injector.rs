use crate::{
    common::DtifFill,
    events::DtifInputEvent,
    nodes::{FrameNode, GroupNode, Node},
    paints::Paint,
    CompDtif, ToEcsBundleImpl,
};
use bevy_ecs::{
    entity::Entity,
    world::{EntityWorldMut, World},
};
use bevy_hierarchy::BuildWorldChildren;
use dyn_comp_asset::{asset_id::AssetId, resources::AssetDatabaseRes};
use dyn_comp_common::{
    common::Fill,
    events::InputEvent,
    mixins::{FillsMixin, PaintParentMixin, Root, StrokeFillsMixin, StrokeMixin},
};
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};

pub struct DtifInjector {
    /// Maps Ids of type String (sid) from the DTIF to actual spawned Bevy entities.
    sid_to_entity: HashMap<String, Entity>,
    /// Maps Ids of type String (sid) from the DTIF to actual asset id.
    sid_to_asset_id: HashMap<String, AssetId>,
}

impl DtifInjector {
    pub fn new() -> Self {
        Self {
            sid_to_entity: HashMap::default(),
            sid_to_asset_id: HashMap::default(),
        }
    }

    pub fn get_sid_to_entity(&self) -> &HashMap<String, Entity> {
        &self.sid_to_entity
    }

    pub fn get_sid_to_asset_id(&self) -> &HashMap<String, AssetId> {
        &self.sid_to_asset_id
    }

    pub fn load_assets(&mut self, dtif: &CompDtif, asset_db: &mut AssetDatabaseRes) {
        for (sid, asset) in &dtif.assets {
            if let Some(asset_id) = asset_db.insert_asset(asset.clone()) {
                self.sid_to_asset_id.insert(sid.clone(), asset_id);
            }
        }
    }

    pub fn inject_from_root(&mut self, dtif: &CompDtif, world: &mut World) -> Option<Entity> {
        // Process paints (before nodes as nodes can reference paint sid's)
        self.process_paints(dtif, world);

        // Process nodes starting from the root
        let maybe_root_node_entity = self.process_node(dtif.root_node_id.clone(), dtif, world);

        if let Some(root_node_entity) = maybe_root_node_entity {
            world.entity_mut(root_node_entity).insert(Root);
            self.inject_input_events(&dtif.events, world);
        }

        return maybe_root_node_entity;
    }

    fn process_node(
        &mut self,
        node_sid: String,
        dtif: &CompDtif,
        world: &mut World,
    ) -> Option<Entity> {
        dtif.nodes.get(&node_sid).map(|node| {
            let node_entity = self.spawn_node(node, world).id();
            self.sid_to_entity.insert(node_sid, node_entity);

            self.process_node_fills(node_entity, node, world);
            self.process_node_stroke(node_entity, node, world);
            self.process_node_children(node_entity, node, dtif, world);

            return node_entity;
        })
    }

    fn spawn_node<'a>(&self, node: &Node, world: &'a mut World) -> EntityWorldMut<'a> {
        match node {
            Node::Frame(node) => world.spawn(node.to_ecs_bundle(&self)),
            Node::Group(node) => world.spawn(node.to_ecs_bundle(&self)),
            Node::Rectangle(node) => world.spawn(node.to_ecs_bundle(&self)),
        }
    }

    fn process_node_children(
        &mut self,
        node_entity: Entity,
        node: &Node,
        dtif: &CompDtif,
        world: &mut World,
    ) {
        if let Node::Frame(FrameNode { children, .. }) | Node::Group(GroupNode { children, .. }) =
            node
        {
            // Process child nodes and collect their Bevy entity ids
            let new_children: Vec<Entity> = children
                .iter()
                .filter_map(|child_sid| self.process_node(child_sid.clone(), dtif, world))
                .collect();

            // Establish Bevy parent-child relationships
            if !new_children.is_empty() {
                world.entity_mut(node_entity).push_children(&new_children);
            }
        }
    }

    fn process_node_fills(&mut self, node_entity: Entity, node: &Node, world: &mut World) {
        let dtif_fills = match node {
            Node::Frame(node) => &node.fills,
            Node::Rectangle(node) => &node.fills,
            _ => return,
        };

        let fills = self.process_fills(node_entity, dtif_fills, world);
        if !fills.is_empty() {
            if let Some(mut node_entity_world) = world.get_entity_mut(node_entity) {
                node_entity_world.insert(FillsMixin(fills));
            }
        }
    }

    fn process_node_stroke(&mut self, node_entity: Entity, node: &Node, world: &mut World) {
        let dtif_stroke = match node {
            Node::Frame(node) => match &node.stroke {
                Some(stroke) => stroke,
                _ => return,
            },
            Node::Rectangle(node) => match &node.stroke {
                Some(stroke) => stroke,
                _ => return,
            },
            _ => return,
        };

        let stroke_fills = self.process_fills(node_entity, &dtif_stroke.fills, world);
        if let Some(mut node_entity_world) = world.get_entity_mut(node_entity) {
            node_entity_world.insert(StrokeMixin(dtif_stroke.to_skia_stroke_without_fills()));
            if !stroke_fills.is_empty() {
                node_entity_world.insert(StrokeFillsMixin(stroke_fills));
            }
        }
    }

    fn process_fills(
        &self,
        entity: Entity,
        dtif_fills: &Vec<DtifFill>,
        world: &mut World,
    ) -> SmallVec<[Fill; 2]> {
        dtif_fills
            .iter()
            .rev()
            .filter_map(|dtif_fill| {
                let fill = dtif_fill.to_fill(&self)?;
                let mut paint_entity_world = world.get_entity_mut(fill.paint)?;

                if let Some(mut paint_parent_mixin) =
                    paint_entity_world.get_mut::<PaintParentMixin>()
                {
                    paint_parent_mixin.0.insert(entity);
                } else {
                    let mut hashset = HashSet::new();
                    hashset.insert(entity);
                    paint_entity_world.insert(PaintParentMixin(hashset));
                }

                Some(fill)
            })
            .collect::<SmallVec<_>>()
    }

    fn process_paints(&mut self, dtif: &CompDtif, world: &mut World) {
        dtif.paints.iter().for_each(|(id, paint)| {
            let paint_entity = self.spawn_paint(&paint, world).id();
            self.sid_to_entity.insert(id.clone(), paint_entity);
        });
    }

    fn spawn_paint<'a>(&self, paint: &Paint, world: &'a mut World) -> EntityWorldMut<'a> {
        match paint {
            Paint::Solid(paint) => world.spawn(paint.to_ecs_bundle(&self)),
            Paint::Image(paint) => world.spawn(paint.to_ecs_bundle(&self)),
            Paint::Gradient(paint) => world.spawn(paint.to_ecs_bundle(&self)),
        }
    }

    fn inject_input_events(&self, events: &Vec<DtifInputEvent>, world: &mut World) {
        events
            .iter()
            .cloned()
            .map(|event| event.to_comp_input_event(&self.sid_to_entity))
            .for_each(|maybe_event| {
                if let Some(event) = maybe_event {
                    event.send_into_ecs(world);
                }
            });
    }

    /// Converts an `Entity` to an Id of type String (sid) used to reference elements in DTIF.
    #[inline]
    pub fn entity_to_sid(entity: &Entity) -> String {
        entity.to_bits().to_string()
    }
}
