use crate::{
    events::DtifInputEvent, nodes::Node, paints::Paint, styles::Style, DtifComposition,
    SpawnBundleImpl,
};
use bevy_ecs::{
    entity::Entity,
    world::{EntityWorldMut, World},
};
use bevy_hierarchy::BuildWorldChildren;
use dyn_comp_asset::{asset_id::AssetId, resources::AssetsRes};
use dyn_comp_bundles::{
    components::{
        marker::Root,
        mixins::{PaintChildMixin, PaintParentMixin, StyleChildrenMixin, StyleParentMixin},
    },
    events::InputEvent,
};
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;

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

    pub fn load_assets(&mut self, dtif: &DtifComposition, asset_db: &mut AssetsRes) {
        for (sid, asset) in &dtif.assets {
            if let Some(asset_id) = asset_db.insert_asset(asset.clone()) {
                self.sid_to_asset_id.insert(sid.clone(), asset_id);
            }
        }
    }

    pub fn inject_from_root(
        &mut self,
        dtif: &DtifComposition,
        world: &mut World,
    ) -> Option<Entity> {
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
        dtif: &DtifComposition,
        world: &mut World,
    ) -> Option<Entity> {
        dtif.nodes.get(&node_sid).map(|node| {
            let node_entity = self.spawn_node(node, world).id();
            self.sid_to_entity.insert(node_sid, node_entity);

            self.process_node_styles(node_entity, node, world);
            self.process_node_children(node_entity, node, dtif, world);

            return node_entity;
        })
    }

    fn spawn_node<'a>(&self, node: &Node, world: &'a mut World) -> EntityWorldMut<'a> {
        match node {
            Node::Frame(node) => node.spawn(&self, world),
            // Node::Group(node) => node.spawn(&self, world), // TODO: Group
            Node::Rectangle(node) => node.spawn(&self, world),
            Node::Ellipse(node) => node.spawn(&self, world),
            Node::Star(node) => node.spawn(&self, world),
            Node::Polygon(node) => node.spawn(&self, world),
            Node::Text(node) => node.spawn(&self, world),
            Node::Vector(node) => node.spawn(&self, world),
        }
    }

    fn process_node_children(
        &mut self,
        node_entity: Entity,
        node: &Node,
        dtif: &DtifComposition,
        world: &mut World,
    ) {
        let dtif_children = match node {
            Node::Frame(node) => &node.children,
            // Node::Group(node) => &node.children, // TODO: Group
            _ => return,
        };

        // Process child nodes and collect their Bevy entity ids
        let child_entities: Vec<Entity> = dtif_children
            .iter()
            .filter_map(|child_sid| self.process_node(child_sid.clone(), dtif, world))
            .collect();

        // Establish Bevy parent-child relationships
        if !child_entities.is_empty() {
            world.entity_mut(node_entity).push_children(&child_entities);
        }
    }

    fn process_node_styles(&mut self, node_entity: Entity, node: &Node, world: &mut World) {
        let dtif_styles = match node {
            Node::Frame(node) => &node.styles,
            Node::Rectangle(node) => &node.styles,
            Node::Ellipse(node) => &node.styles,
            Node::Star(node) => &node.styles,
            Node::Polygon(node) => &node.styles,
            Node::Text(node) => &node.styles,
            Node::Vector(node) => &node.styles,
            _ => return,
        };

        // Process styles and collect their Bevy entity ids
        let style_entities: SmallVec<[Entity; 2]> = dtif_styles
            .iter()
            .filter_map(|style| self.process_style(style, node_entity, world))
            .collect();

        // Establish Bevy parent-child relationship between node and style
        if !style_entities.is_empty() {
            world
                .entity_mut(node_entity)
                .insert(StyleChildrenMixin(style_entities));
        }
    }

    fn process_style(
        &self,
        style: &Style,
        node_entity: Entity,
        world: &mut World,
    ) -> Option<Entity> {
        // Spawn style
        let mut style_entity_world_mut = self.spawn_style(style, world);
        style_entity_world_mut.insert(StyleParentMixin(node_entity));
        let style_entity = style_entity_world_mut.id();

        // Reference style entity in paint
        if let Some(paint_entity) = style_entity_world_mut
            .get::<PaintChildMixin>()
            .and_then(|paint| paint.0)
        {
            let mut paint_entity_world_mut = world.get_entity_mut(paint_entity)?;
            if let Some(mut paint_parent_mixin) =
                paint_entity_world_mut.get_mut::<PaintParentMixin>()
            {
                paint_parent_mixin.0.push(style_entity);
            } else {
                paint_entity_world_mut.insert(PaintParentMixin(smallvec![style_entity]));
            }
        }

        return Some(style_entity);
    }

    fn spawn_style<'a>(&self, style: &Style, world: &'a mut World) -> EntityWorldMut<'a> {
        match style {
            Style::Fill(style) => style.spawn(&self, world),
            Style::Stroke(style) => style.spawn(&self, world),
            Style::DropShadow(style) => style.spawn(&self, world),
        }
    }

    fn process_paints(&mut self, dtif: &DtifComposition, world: &mut World) {
        for (id, paint) in dtif.paints.iter() {
            let paint_entity = self.spawn_paint(&paint, world).id();
            self.sid_to_entity.insert(id.clone(), paint_entity);
        }
    }

    fn spawn_paint<'a>(&self, paint: &Paint, world: &'a mut World) -> EntityWorldMut<'a> {
        match paint {
            Paint::Solid(paint) => paint.spawn(&self, world),
            Paint::Image(paint) => paint.spawn(&self, world),
            Paint::Gradient(paint) => paint.spawn(&self, world),
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
