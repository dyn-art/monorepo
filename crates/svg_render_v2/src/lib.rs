use bevy_ecs::{
    entity::Entity,
    system::{ResMut, Resource},
};
use dyn_composition::core::{
    modules::node::components::mixins::{ChildrenMixin, DimensionMixin},
    utils::continuous_id::ContinuousId,
};
use serde::Serialize;
use specta::Type;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    mem::take,
};

#[derive(Serialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum MixinChange {
    Dimension(DimensionMixin),
    Children(MixinChangeChildrenMixin),
}

pub trait ToMixinChange {
    fn to_mixin_change(&self) -> MixinChange;
}

impl ToMixinChange for DimensionMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::Dimension(self.clone())
    }
}

#[derive(Serialize, Clone, Debug, Type)]
pub struct MixinChangeChildrenMixin {
    pub children: ChildrenMixin,
}

#[derive(Debug, Clone, Copy)]
pub enum ChangedEntityType {
    ShapeNode,
    FrameNode,
    SolidPaint,
    ImageFillPaint,
    ImageFitPaint,
    ImageCropPaint,
    ImageTilePaint,
    LinearGradientPaint,
    RadialGradientPaint,
}

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

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum ElementChange {
    ElementCreated,
    ElementDeleted,
    // ..
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementChangeEvent {
    pub id: ContinuousId,
    pub changes: Vec<ElementChange>,
}

#[derive(Debug)]
pub struct SVGElement {
    id: ContinuousId,
    tag_name: String,
    attributes: HashMap<&'static str, String>,
    styles: HashMap<&'static str, String>,
    children: Vec<SVGElementChild>,
    changes: Vec<ElementChange>,
}

impl SVGElement {
    pub fn get_id(&self) -> ContinuousId {
        self.id
    }

    pub fn remove(&mut self) -> () {
        // TODO
    }

    pub fn drain_changes(&mut self) -> Vec<ElementChange> {
        self.changes.drain(..).collect()
    }

    pub fn to_string(&self, bundle: &Box<dyn SVGBundle>, cx: &SVGContext) -> String {
        String::from("todo")
    }
}

#[derive(Debug)]
pub enum SVGElementChild {
    InSVGBundleContext(Entity, ContinuousId),
    InSVGContext(Entity),
}

pub trait SVGBundle: Sync + Send + Debug {
    fn get_entity(&self) -> &Entity;
    fn get_type(&self) -> ChangedEntityType;
    fn append_child(&mut self, svg_bundle: &mut Box<dyn SVGBundle>) -> ();
    fn update(&mut self, changed_entity: ChangedEntity, cx: &mut SVGContext) -> ();
    fn get_child_elements(&self) -> BTreeMap<ContinuousId, &SVGElement>;
    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement>;
    fn get_root_element(&self) -> &SVGElement;
    fn get_root_element_mut(&mut self) -> &mut SVGElement;
    fn drain_changes(&mut self) -> Vec<ElementChangeEvent>;
    fn to_string(&self, cx: &SVGContext) -> String;
}

#[derive(Resource, Debug)]
pub struct SVGCompositionRes {
    pub context: SVGContext,
}

#[derive(Debug)]
pub struct SVGContext {
    root_bundle_ids: Vec<Entity>,
    bundles: HashMap<Entity, Box<dyn SVGBundle>>,
    pub changed_entities: Vec<ChangedEntity>,
}

impl SVGContext {
    pub fn get_bundle(&self, entity: &Entity) -> Option<&Box<dyn SVGBundle>> {
        self.bundles.get(&entity)
    }

    pub fn get_bundle_mut(&mut self, entity: &Entity) -> Option<&mut Box<dyn SVGBundle>> {
        self.bundles.get_mut(&entity)
    }

    pub fn insert_bundle(&mut self, bundle: Box<dyn SVGBundle>) -> () {
        let entity = bundle.get_entity().clone();
        if !self.bundles.contains_key(&entity) {
            self.bundles.insert(bundle.get_entity().clone(), bundle);
        }
    }

    pub fn create_bundle(
        entity: Entity,
        entity_type: ChangedEntityType,
    ) -> Option<Box<dyn SVGBundle>> {
        match entity_type {
            ChangedEntityType::FrameNode => Some(Box::new(FrameSVGBundle::new(entity))),
            _ => None,
        }
    }

    // TODO: Improve so its not necessary to remove element?
    pub fn apply_updates(&mut self) {
        let changed_entities: Vec<ChangedEntity> = self.changed_entities.drain(..).collect();
        for changed_entity in changed_entities {
            if let Some(mut bundle) = self.bundles.remove(&changed_entity.entity) {
                bundle.update(changed_entity, self);
                self.bundles.insert(*bundle.get_entity(), bundle);
            }
        }
    }
}

#[derive(Debug)]
pub struct FrameSVGBundle {
    entity: Entity,

    root: SVGElement,
    defs: SVGElement,

    // Content elements
    content_clip_path: SVGElement,
    content_clipped_rect: SVGElement,
    content_wrapper_g: SVGElement,

    // Children elements
    children_wrapper_g: SVGElement,

    // Fill elements
    fill_clip_path: SVGElement,
    fill_clipped_path: SVGElement,
    fill_wrapper_g: SVGElement,

    // Children
    paint_children: Vec<Entity>,
    node_children: Vec<Entity>,
}

impl SVGBundle for FrameSVGBundle {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn get_type(&self) -> ChangedEntityType {
        ChangedEntityType::ShapeNode
    }

    fn append_child(&mut self, svg_bundle: &mut Box<dyn SVGBundle>) -> () {
        let svg_bundle_type = svg_bundle.get_type();
        match svg_bundle_type {
            ChangedEntityType::SolidPaint
            | ChangedEntityType::ImageFillPaint
            | ChangedEntityType::ImageFitPaint
            | ChangedEntityType::ImageCropPaint
            | ChangedEntityType::ImageTilePaint
            | ChangedEntityType::LinearGradientPaint
            | ChangedEntityType::RadialGradientPaint => {
                // TODO
            }
            _ => {}
        }
    }

    fn update(&mut self, changed_entity: ChangedEntity, cx: &mut SVGContext) -> () {
        for change in &changed_entity.changes {
            match change {
                MixinChange::Dimension(mixin) => {
                    // TODO
                }
                MixinChange::Children(mixin) => {
                    let children = &mixin.children.0;

                    // TODO
                    // 1. Detect removed elements and remove those
                    // 2. Detect newly added elements and add those
                    // 3. Reorder elements
                    //
                    // Note to make this work we first need to create all elements so that they are preent in the SVGContext
                    // and THEN apply the changes via the "update" method

                    self.node_children = children.clone();
                }
                _ => {}
            }
        }
    }

    fn drain_changes(&mut self) -> Vec<ElementChangeEvent> {
        let mut drained_updates: Vec<ElementChangeEvent> = Vec::new();

        // Drain updates from root element
        let root = self.get_root_element_mut();
        let changes = root.drain_changes();
        if !changes.is_empty() {
            drained_updates.push(ElementChangeEvent {
                id: root.get_id(),
                changes,
            });
        }

        // Drain updates from children
        for (_, child_element) in self.get_child_elements_mut() {
            let changes = child_element.drain_changes();
            if !changes.is_empty() {
                drained_updates.push(ElementChangeEvent {
                    id: child_element.get_id(),
                    changes,
                });
            }
        }

        return drained_updates;
    }

    fn get_child_elements(&self) -> BTreeMap<ContinuousId, &SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.id, &self.defs);
        // .. (from top to bottom as updates should be drained from the most top element first)
        return children;
    }

    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.id, &mut self.defs);
        // .. (from top to bottom as updates should be drained from the most top element first)
        return children;
    }

    fn get_root_element(&self) -> &SVGElement {
        return &self.root;
    }

    fn get_root_element_mut(&mut self) -> &mut SVGElement {
        return &mut self.root;
    }

    fn to_string(&self, cx: &SVGContext) -> String {
        self.get_root_element()
            .to_string(cx.get_bundle(&self.entity).unwrap(), cx)
    }
}

impl FrameSVGBundle {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            root: todo!(),
            defs: todo!(),
            content_clip_path: todo!(),
            content_clipped_rect: todo!(),
            content_wrapper_g: todo!(),
            children_wrapper_g: todo!(),
            fill_clip_path: todo!(),
            fill_clipped_path: todo!(),
            fill_wrapper_g: todo!(),
            paint_children: todo!(),
            node_children: todo!(),
        }
    }
}

pub fn queue_element_changes(
    mut changed_entities_res: ResMut<ChangedEntitiesRes>,
    mut svg_composition: ResMut<SVGCompositionRes>,
) {
    let changed_entities = take(&mut changed_entities_res.changed_entities);
    let dependency_tree = build_dependency_tree(changed_entities);

    for root_branch in dependency_tree {
        process_tree_branch(root_branch, &mut svg_composition.context);
    }

    // TODO: apply updates
}

#[derive(Debug)]
struct ChangedEntityBranch {
    entity: Entity,
    changed: ChangedEntity,
    children: Vec<ChangedEntityBranch>,
}

fn build_dependency_tree(
    mut changed_entities: HashMap<Entity, ChangedEntity>,
) -> Vec<ChangedEntityBranch> {
    let mut children_map: HashMap<Entity, Vec<Entity>> = HashMap::new();
    let mut roots: Vec<Entity> = Vec::new();

    // Identify root nodes and prepare a map of children for each parent
    for (entity, changed_entity) in &changed_entities {
        match changed_entity.parent_id {
            Some(parent_id) => {
                children_map
                    .entry(parent_id)
                    .or_insert_with(Vec::new)
                    .push(*entity);
            }
            None => roots.push(*entity),
        }
    }

    // Build trees from the roots
    return roots
        .into_iter()
        .map(|root| build_tree(root, &mut changed_entities, &children_map))
        .collect();
}

fn build_tree(
    entity: Entity,
    changed_entities: &mut HashMap<Entity, ChangedEntity>,
    children_map: &HashMap<Entity, Vec<Entity>>,
) -> ChangedEntityBranch {
    let changed_entity = changed_entities
        .remove(&entity)
        .expect("Entity should exist");

    let children = children_map
        .get(&entity)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|&child_entity| build_tree(child_entity, changed_entities, children_map))
        .collect();

    return ChangedEntityBranch {
        entity,
        changed: changed_entity,
        children,
    };
}

fn process_tree_branch(branch: ChangedEntityBranch, cx: &mut SVGContext) {
    process_entity(branch.entity, branch.changed, cx);

    // Recursively process children, if any
    for child in branch.children {
        process_tree_branch(child, cx);
    }
}

fn process_entity(entity: Entity, changed_entity: ChangedEntity, cx: &mut SVGContext) {
    if let Some(bundle) = SVGContext::create_bundle(entity, changed_entity.entity_type) {
        cx.insert_bundle(bundle);
        cx.changed_entities.push(changed_entity);
    }
}
