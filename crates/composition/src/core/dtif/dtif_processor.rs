use std::collections::HashMap;

use bevy_ecs::{entity::Entity, world::World};
use bevy_hierarchy::BuildWorldChildren;
use glam::Mat3;

use crate::core::modules::{
    composition::events::CoreInputEvent,
    node::components::{
        bundles::{
            FrameNodeBundle, GroupNodeBundle, PaintBundle, RectangleNodeBundle, TextNodeBundle,
        },
        mixins::{AbsoluteTransformMixin, ChildrenMixin, FillMixin, RelativeTransformMixin},
    },
};

use super::{DTIFComposition, NodeBundle};

pub struct DTIFProcessor {
    /// Maps EntityId (eid) of DTIF to actual spawned Bevy entity.
    eid_to_entity: HashMap<String, Entity>,
}

impl DTIFProcessor {
    pub fn new() -> Self {
        DTIFProcessor {
            eid_to_entity: HashMap::new(),
        }
    }

    // =========================================================================
    // Node
    // =========================================================================

    /// Processes the root DTIF node and its children.
    pub fn process_root(
        &mut self,
        node_eid: String,
        world: &mut World,
        dtif: &DTIFComposition,
    ) -> Option<Entity> {
        self.process_node(node_eid, world, dtif, true)
    }

    /// Processes a single DTIF node and its children.
    fn process_node(
        &mut self,
        node_eid: String,
        world: &mut World,
        dtif: &DTIFComposition,
        is_root: bool,
    ) -> Option<Entity> {
        dtif.nodes.get(&node_eid).map(|dtif_node| {
            // Spawn a new node entity from a DTIF node
            // and maintain a mapping from entity id to Bevy entity
            let node_entity = self.spawn_node(world, dtif_node);
            self.eid_to_entity.insert(node_eid, node_entity);

            // Set absolute transform for root node
            if is_root {
                let mut relative_transform: Option<Mat3> = None;
                if let Some(relative_transform_mixin) =
                    world.entity(node_entity).get::<RelativeTransformMixin>()
                {
                    relative_transform = Some(relative_transform_mixin.0.clone());
                }
                if let Some(relative_transform) = relative_transform {
                    world
                        .entity_mut(node_entity)
                        .insert(AbsoluteTransformMixin(relative_transform));
                }
            }

            self.process_fill(node_entity, world, dtif, dtif_node);
            self.process_children(node_entity, world, dtif, dtif_node);

            return node_entity;
        })
    }

    /// Processes the fill mixin of a DTIF node, if present.
    fn process_fill(
        &mut self,
        node_entity: Entity,
        world: &mut World,
        dtif: &DTIFComposition,
        dtif_node: &NodeBundle,
    ) {
        if let NodeBundle::Frame(FrameNodeBundle { fill_mixin, .. })
        | NodeBundle::Rectangle(RectangleNodeBundle { fill_mixin, .. })
        | NodeBundle::Text(TextNodeBundle { fill_mixin, .. }) = dtif_node
        {
            // Process paints and collect their Bevy entity ids
            let new_paints: Vec<Entity> = fill_mixin
                .paint_ids
                .iter()
                .filter_map(|paint_entity| {
                    let paint_eid = DTIFProcessor::entity_to_eid(paint_entity);
                    return self.process_paint(paint_eid, world, dtif);
                })
                .collect();

            // Establish Bevy parent-child relationships for optimized interactions within the ECS.
            // For details, refer to: https://bevy-cheatbook.github.io/fundamentals/hierarchy.html
            if !new_paints.is_empty() {
                world.entity_mut(node_entity).push_children(&new_paints);
            }

            // Remove the temporary `FillMixin` component.
            // Explanation:
            // After successfully establishing Bevy's internal parent-child relationships,
            // the `FillMixin` component, initially used to manage child entities
            // during the transition from the DTIF format, is no longer necessary.
            world.entity_mut(node_entity).remove::<FillMixin>();
        }
    }

    /// Processes the children of a DTIF node, if any.
    fn process_children(
        &mut self,
        node_entity: Entity,
        world: &mut World,
        dtif: &DTIFComposition,
        dtif_node: &NodeBundle,
    ) {
        if let NodeBundle::Frame(FrameNodeBundle { children_mixin, .. })
        | NodeBundle::Group(GroupNodeBundle { children_mixin, .. }) = dtif_node
        {
            // Process child nodes and collect their Bevy entity ids
            let new_children: Vec<Entity> = children_mixin
                .0
                .iter()
                .filter_map(|child_entity| {
                    let child_eid = DTIFProcessor::entity_to_eid(child_entity);
                    return self.process_node(child_eid, world, dtif, false);
                })
                .collect();

            // Establish Bevy parent-child relationships for optimized interactions within the ECS.
            // For details, refer to: https://bevy-cheatbook.github.io/fundamentals/hierarchy.html
            if !new_children.is_empty() {
                world.entity_mut(node_entity).push_children(&new_children);

                // Calculate & apply absolute transform to children based on the parents absolute transform
                let mut transform_updates = Vec::new();
                if let Some(parent_absolute_transform) =
                    world.entity(node_entity).get::<AbsoluteTransformMixin>()
                {
                    for &child in &new_children {
                        if let Some(child_relative_transform) =
                            world.entity(child).get::<RelativeTransformMixin>()
                        {
                            let child_absolute_transform =
                                parent_absolute_transform.0 * child_relative_transform.0;
                            transform_updates.push((child, child_absolute_transform));
                        }
                    }
                }
                for (child, absolute_transform) in transform_updates {
                    world
                        .entity_mut(child)
                        .insert(AbsoluteTransformMixin(absolute_transform));
                }
            }

            // Remove the temporary `ChildrenMixin` component.
            // Explanation:
            // After successfully establishing Bevy's internal parent-child relationships,
            // the `ChildrenMixin` component, initially used to manage child entities
            // during the transition from the DTIF format, is no longer necessary.
            world.entity_mut(node_entity).remove::<ChildrenMixin>();
        }
    }

    /// Spawns a DTIF node into the ECS world.
    fn spawn_node(&self, world: &mut World, node: &NodeBundle) -> Entity {
        match node {
            NodeBundle::Frame(bundle) => world.spawn(bundle.clone()).id(),
            NodeBundle::Rectangle(bundle) => world.spawn(bundle.clone()).id(),
            NodeBundle::Group(bundle) => world.spawn(bundle.clone()).id(),
            NodeBundle::Text(bundle) => world.spawn(bundle.clone()).id(),
        }
    }

    // =========================================================================
    // Paint
    // =========================================================================

    /// Processes a single DTIF paint.
    pub fn process_paint(
        &mut self,
        paint_eid: String,
        world: &mut World,
        dtif: &DTIFComposition,
    ) -> Option<Entity> {
        dtif.paints.get(&paint_eid).map(|paint| {
            // Spawn paint
            let paint_entity = self.spawn_paint(world, paint);
            self.eid_to_entity.insert(paint_eid, paint_entity);

            return paint_entity;
        })
    }

    /// Spawns a DTIF paint into the ECS world.
    fn spawn_paint(&self, world: &mut World, paint: &PaintBundle) -> Entity {
        match paint {
            PaintBundle::Solid(bundle) => world.spawn(bundle.clone()).id(),
            PaintBundle::Image(bundle) => world.spawn(bundle.clone()).id(),
            PaintBundle::Gradient(bundle) => world.spawn(bundle.clone()).id(),
        }
    }

    // =========================================================================
    // Event
    // =========================================================================

    /// Processes and sends the event into the ECS world.
    pub fn send_event_into_world(&self, event: CoreInputEvent, world: &mut World) {
        match event {
            // Composition Events
            CoreInputEvent::CompositionResized(event) => {
                world.send_event(event);
            }
            CoreInputEvent::CompositionViewBoxChanged(event) => {
                world.send_event(event);
            }

            // Node Events
            CoreInputEvent::NodeCreated(mut event) => {
                if let Some(parent_entity) = event
                    .parent_entity
                    .and_then(|entity| self.find_entity(&entity))
                {
                    event.parent_entity = Some(parent_entity);
                    match &mut event.node {
                        NodeBundle::Rectangle(RectangleNodeBundle { fill_mixin, .. })
                        | NodeBundle::Frame(FrameNodeBundle { fill_mixin, .. })
                        | NodeBundle::Text(TextNodeBundle { fill_mixin, .. }) => {
                            fill_mixin.paint_ids = fill_mixin
                                .paint_ids
                                .iter()
                                .filter_map(|paint_id| self.find_entity(paint_id))
                                .collect()
                        }
                        _ => {}
                    };
                    world.send_event(event);
                } else {
                    world.send_event(event);
                }
            }
            CoreInputEvent::NodeDeleted(mut event) => {
                if let Some(entity) = self.find_entity(&event.entity) {
                    event.entity = entity;
                    world.send_event(event);
                }
            }

            // Entity Events
            CoreInputEvent::EntityMoved(mut event) => {
                if let Some(entity) = self.find_entity(&event.entity) {
                    event.entity = entity;
                    world.send_event(event);
                }
            }
            CoreInputEvent::EntitySetPosition(mut event) => {
                if let Some(entity) = self.find_entity(&event.entity) {
                    event.entity = entity;
                    world.send_event(event);
                }
            }
        }
    }

    // =========================================================================
    // Helper
    // =========================================================================

    /// Tries to find the actual spawned entity for a DTIF entity.
    fn find_entity(&self, dtif_entity: &Entity) -> Option<Entity> {
        let eid = DTIFProcessor::entity_to_eid(dtif_entity);
        self.eid_to_entity.get(&eid).cloned()
    }

    /// Converts an `Entity` to an EntityId (eid) used to reference bundles in DTIF.
    #[inline]
    pub fn entity_to_eid(entity: &Entity) -> String {
        entity.to_bits().to_string()
    }
}
