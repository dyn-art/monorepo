use std::collections::BTreeMap;

use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::{
    events::output_event::ElementChangeEvent,
    mixin_change::MixinChange,
    resources::{
        changed_entities::{ChangedEntity, ChangedEntityType},
        svg_composition::{
            svg_bundle::SVGBundle,
            svg_context::SVGContext,
            svg_element::{SVGElement, SVGElementChildIdentifier, SVGTag},
        },
    },
};

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

    fn get_child_elements(&self) -> BTreeMap<ContinuousId, &SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.get_id(), &self.defs);
        // .. (from top to bottom as updates should be drained from the most top element first)
        return children;
    }

    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.get_id(), &mut self.defs);
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
        self.get_root_element().to_string(self, cx)
    }
}

impl FrameSVGBundle {
    pub fn new(entity: Entity, cx: &mut SVGContext) -> Self {
        let mut root = SVGElement::new(SVGTag::Group, cx.id_generator.next_id());

        let mut defs_element = SVGElement::new(SVGTag::Defs, cx.id_generator.next_id());
        let defs_element_id = defs_element.get_id();
        root.append_child_element(
            &mut defs_element,
            SVGElementChildIdentifier::InSVGBundleContext(entity, defs_element_id),
        );

        let mut content_clip_path_element =
            SVGElement::new(SVGTag::ClipPath, cx.id_generator.next_id());
        let content_clip_path_element_id = content_clip_path_element.get_id();
        defs_element.append_child_element(
            &mut content_clip_path_element,
            SVGElementChildIdentifier::InSVGBundleContext(entity, content_clip_path_element_id),
        );

        // TODO

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
