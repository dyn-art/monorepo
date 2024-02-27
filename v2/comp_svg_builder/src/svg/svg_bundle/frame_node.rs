use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::SvgBundle,
        svg_element::{attributes::SvgAttribute, SvgElement, SvgElementId, SvgTag},
    },
};
use bevy_ecs::entity::Entity;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct FrameNodeSvgBundle {
    pub root: SvgElement,
    pub defs: SvgElement,

    // Content elements
    pub content_clip_path: SvgElement,
    pub content_clipped_rect: SvgElement,
    pub content_wrapper_g: SvgElement,

    // Children elements
    pub children_wrapper_g: SvgElement,

    // Fill elements
    pub fill_clip_path: SvgElement,
    pub fill_clipped_path: SvgElement,
    pub fill_wrapper_g: SvgElement,

    // Children
    pub node_children: Vec<Entity>,
    pub paint_children: Vec<Entity>,
}

impl SvgBundle for FrameNodeSvgBundle {
    fn get_root_element(&self) -> &SvgElement {
        &self.root
    }

    fn get_root_element_mut(&mut self) -> &mut SvgElement {
        &mut self.root
    }

    fn get_child_elements(&self) -> BTreeMap<SvgElementId, &SvgElement> {
        let mut children = BTreeMap::new();

        children.insert(self.defs.get_id(), &self.defs);
        children.insert(self.content_clip_path.get_id(), &self.content_clip_path);
        children.insert(
            self.content_clipped_rect.get_id(),
            &self.content_clipped_rect,
        );
        children.insert(self.content_wrapper_g.get_id(), &self.content_wrapper_g);
        children.insert(self.fill_clip_path.get_id(), &self.fill_clip_path);
        children.insert(self.fill_clipped_path.get_id(), &self.fill_clipped_path);
        children.insert(self.fill_wrapper_g.get_id(), &self.fill_wrapper_g);
        children.insert(self.children_wrapper_g.get_id(), &self.children_wrapper_g);

        return children;
    }

    fn get_child_elements_mut(&mut self) -> BTreeMap<SvgElementId, &mut SvgElement> {
        let mut children = BTreeMap::new();

        children.insert(self.defs.get_id(), &mut self.defs);
        children.insert(self.content_clip_path.get_id(), &mut self.content_clip_path);
        children.insert(
            self.content_clipped_rect.get_id(),
            &mut self.content_clipped_rect,
        );
        children.insert(self.content_wrapper_g.get_id(), &mut self.content_wrapper_g);
        children.insert(self.fill_clip_path.get_id(), &mut self.fill_clip_path);
        children.insert(self.fill_clipped_path.get_id(), &mut self.fill_clipped_path);
        children.insert(self.fill_wrapper_g.get_id(), &mut self.fill_wrapper_g);
        children.insert(
            self.children_wrapper_g.get_id(),
            &mut self.children_wrapper_g,
        );

        return children;
    }
}

impl FrameNodeSvgBundle {
    pub fn new(entity: Entity, cx: &mut SvgContextRes) -> Self {
        log::info!("[FrameNodeSvgBundle::new] {:?}", entity);

        let mut root_element = cx.create_bundle_root_element(SvgTag::Group, entity);
        #[cfg(feature = "tracing")]
        root_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(root_element.get_id(), "root", false),
        });

        let mut defs_element = cx.create_element(SvgTag::Defs);
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(defs_element.get_id(), "defs", false),
        });
        root_element.append_child_in_bundle_context(entity, &mut defs_element);

        // Create content elements

        let mut content_clip_path_element = cx.create_element(SvgTag::ClipPath);
        #[cfg(feature = "tracing")]
        content_clip_path_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                content_clip_path_element.get_id(),
                "content-clip-path",
                true,
            ),
        });
        defs_element.append_child_in_bundle_context(entity, &mut content_clip_path_element);

        let mut content_clipped_rect_element = cx.create_element(SvgTag::Rect);
        #[cfg(feature = "tracing")]
        content_clipped_rect_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                content_clipped_rect_element.get_id(),
                "content-clipped-rect",
                false,
            ),
        });
        content_clip_path_element
            .append_child_in_bundle_context(entity, &mut content_clipped_rect_element);

        let mut content_wrapper_g_element = cx.create_element(SvgTag::Group);
        #[cfg(feature = "tracing")]
        content_wrapper_g_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                content_wrapper_g_element.get_id(),
                "content-wrapper-g",
                false,
            ),
        });
        content_wrapper_g_element.set_attribute(SvgAttribute::ClipPath {
            clip_path: content_clip_path_element.get_id(),
        });
        root_element.append_child_in_bundle_context(entity, &mut content_wrapper_g_element);

        // Create fill elements

        let mut fill_clip_path_element = cx.create_element(SvgTag::ClipPath);
        #[cfg(feature = "tracing")]
        fill_clip_path_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                fill_clip_path_element.get_id(),
                "fill-clip-path",
                true,
            ),
        });
        defs_element.append_child_in_bundle_context(entity, &mut fill_clip_path_element);

        let mut fill_clipped_path_element = cx.create_element(SvgTag::Rect);
        #[cfg(feature = "tracing")]
        fill_clipped_path_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                fill_clipped_path_element.get_id(),
                "fill-clipped-path",
                false,
            ),
        });
        fill_clip_path_element
            .append_child_in_bundle_context(entity, &mut fill_clipped_path_element);

        let mut fill_wrapper_g_element = cx.create_element(SvgTag::Group);
        #[cfg(feature = "tracing")]
        fill_wrapper_g_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                fill_wrapper_g_element.get_id(),
                "fill-wrapper-g",
                false,
            ),
        });
        fill_wrapper_g_element.set_attribute(SvgAttribute::ClipPath {
            clip_path: fill_clip_path_element.get_id(),
        });
        content_wrapper_g_element
            .append_child_in_bundle_context(entity, &mut fill_wrapper_g_element);

        // Create children wrapper element

        let mut children_wrapper_g_element = cx.create_element(SvgTag::Group);
        #[cfg(feature = "tracing")]
        children_wrapper_g_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                children_wrapper_g_element.get_id(),
                "children-wrapper-g",
                false,
            ),
        });
        content_wrapper_g_element
            .append_child_in_bundle_context(entity, &mut children_wrapper_g_element);

        Self {
            root: root_element,
            defs: defs_element,

            // Content elements
            content_clip_path: content_clip_path_element,
            content_clipped_rect: content_clipped_rect_element,
            content_wrapper_g: content_wrapper_g_element,

            // Children elements
            children_wrapper_g: children_wrapper_g_element,

            // Fill elements
            fill_clip_path: fill_clip_path_element,
            fill_clipped_path: fill_clipped_path_element,
            fill_wrapper_g: fill_wrapper_g_element,

            node_children: Vec::new(),
            paint_children: Vec::new(),
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: SvgElementId, category: &str, is_definition: bool) -> String {
        let def_part = if is_definition { "_def" } else { "" };
        format!("frame-node_{}_{}{}", category, id, def_part)
    }
}
