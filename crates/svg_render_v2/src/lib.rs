use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::{
    modules::node::components::mixins::DimensionMixin, utils::continuous_id::ContinuousId,
};
use serde::Serialize;
use specta::Type;
use std::{
    cell::{RefCell, RefMut},
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    rc::Rc,
};

#[derive(Serialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum MixinChange {
    Dimension(DimensionMixin),
}

pub trait ToMixinChange {
    fn to_mixin_change(&self) -> MixinChange;
}

impl ToMixinChange for DimensionMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::Dimension(self.clone())
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct EntityChanges {
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

#[derive(Debug)]
pub enum SVGElementChild {
    InSVGBundleContext(Entity, ContinuousId),
    InSVGContext(Entity),
}

pub trait SVGBundle {
    fn get_children(&self) -> BTreeMap<ContinuousId, &SVGElement>;
    fn get_root(&self) -> &SVGElement;
    fn drain_changes(&mut self) -> Vec<ElementChangeEvent>;
    fn to_string(&self, cx: &mut SVGContext) -> String;
}

pub trait SVGEntity: SVGBundle + Sync + Send + Debug {
    fn get_type(&self) -> ChangedEntityType;
    fn append_child(&mut self, svg_entity: &Box<dyn SVGEntity>) -> ();
    fn update(&mut self, entity_changes: EntityChanges, cx: &mut SVGContext) -> ();
}

#[derive(Resource, Debug)]
pub struct SVGCompositionRes {
    pub context: SVGContext,
}

#[derive(Debug)]
pub struct SVGContext {
    root_bundle_ids: Vec<Entity>,
    bundles: HashMap<Entity, Box<dyn SVGEntity>>,
}

impl SVGContext {
    pub fn get_bundle(&self, entity: Entity) -> Option<&Box<dyn SVGEntity>> {
        self.bundles.get(&entity)
    }

    pub fn create_bundle(&mut self, entity: Entity) {
        // TODO
    }

    // TODO: Figure out how Zed does that?
    pub fn apply_bundle_changes(&mut self, entity: Entity, entity_changes: EntityChanges) {
        if let Some(bundle) = self.bundles.get_mut(&entity) {
            bundle.update(entity_changes, self);
        }
    }
}

#[derive(Debug)]
pub struct ShapeSVGBundle {
    root: SVGElement,

    defs: SVGElement,

    // Fill elements
    fill_clip_path: SVGElement,
    fill_clipped_path: SVGElement,
    fill_wrapper_g: SVGElement,

    // Click area elements
    click_area_rect: SVGElement,
}

impl SVGBundle for ShapeSVGBundle {
    fn drain_changes(&mut self) -> Vec<ElementChangeEvent> {
        vec![]
    }

    fn get_children(&self) -> BTreeMap<ContinuousId, &SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.id, &self.defs);
        // .. (from top to bottom as updates should be drained from the most top element first)
        return children;
    }

    fn get_root(&self) -> &SVGElement {
        return &self.root;
    }

    fn to_string(&self, cx: &mut SVGContext) -> String {
        String::from("todo")
    }
}

impl SVGEntity for ShapeSVGBundle {
    fn get_type(&self) -> ChangedEntityType {
        ChangedEntityType::ShapeNode
    }

    fn append_child(&mut self, svg_entity: &Box<dyn SVGEntity>) -> () {
        let svg_entity_type = svg_entity.get_type();
        match svg_entity_type {
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

    fn update(&mut self, entity_changes: EntityChanges, cx: &mut SVGContext) -> () {
        for change in &entity_changes.changes {
            match change {
                // TODO
                _ => {}
            }
        }
    }
}
