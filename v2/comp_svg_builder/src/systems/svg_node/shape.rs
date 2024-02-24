use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_element::{
            attributes::{SvgAttribute, SvgMeasurementUnit},
            SvgElement, SvgElementId,
        },
        svg_node::SvgNode,
    },
};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Commands, Query, ResMut},
};
use dyn_comp_types::{
    mixins::SizeMixin,
    nodes::{
        CompNode, EllipseCompNode, PolygonCompNode, RectangleCompNode, StarCompNode, TextCompNode,
    },
};
use std::collections::BTreeMap;

#[derive(Component, Debug, Clone)]
pub struct ShapeSvgNode {
    root: SvgElement,
    defs: SvgElement,

    // Fill elements
    fill_clip_path: SvgElement,
    fill_clipped_path: SvgElement,
    fill_wrapper_g: SvgElement,

    // Click area elements
    click_area_rect: SvgElement,
}

impl SvgNode for ShapeSvgNode {
    fn get_root_element(&self) -> &SvgElement {
        &self.root
    }

    fn get_root_element_mut(&mut self) -> &mut SvgElement {
        &mut self.root
    }

    fn get_child_elements(&self) -> BTreeMap<SvgElementId, &SvgElement> {
        let mut children = BTreeMap::new();

        children.insert(self.defs.get_id(), &self.defs);
        children.insert(self.click_area_rect.get_id(), &self.click_area_rect);
        children.insert(self.fill_clip_path.get_id(), &self.fill_clip_path);
        children.insert(self.fill_clipped_path.get_id(), &self.fill_clipped_path);
        children.insert(self.fill_wrapper_g.get_id(), &self.fill_wrapper_g);

        return children;
    }

    fn get_child_elements_mut(&mut self) -> BTreeMap<SvgElementId, &mut SvgElement> {
        let mut children = BTreeMap::new();

        children.insert(self.defs.get_id(), &mut self.defs);
        children.insert(self.click_area_rect.get_id(), &mut self.click_area_rect);
        children.insert(self.fill_clip_path.get_id(), &mut self.fill_clip_path);
        children.insert(self.fill_clipped_path.get_id(), &mut self.fill_clipped_path);
        children.insert(self.fill_wrapper_g.get_id(), &mut self.fill_wrapper_g);

        return children;
    }
}

impl ShapeSvgNode {
    pub fn new(entity: Entity, cx: &mut SvgContextRes) -> Self {
        let mut root_element = cx.create_bundle_root_element("group", entity);
        #[cfg(feature = "tracing")]
        root_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(root_element.get_id(), String::from("root"), false),
        });

        let mut defs_element = cx.create_element("defs");
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(defs_element.get_id(), String::from("defs"), false),
        });
        root_element.append_child_in_node_context(entity, &mut defs_element);

        // Create click area elements

        let mut click_area_rect_element = cx.create_element("rect");
        #[cfg(feature = "tracing")]
        click_area_rect_element.set_attributes(vec![
            SvgAttribute::Name {
                name: Self::create_element_name(
                    click_area_rect_element.get_id(),
                    String::from("click-area-rect"),
                    false,
                ),
            },
            SvgAttribute::Fill {
                fill: String::from("rgba(255, 204, 203, 0.5)"),
            },
        ]);
        #[cfg(not(feature = "tracing"))]
        click_area_rect_element.set_attribute(SvgAttribute::Fill {
            fill: String::from("transparent"),
        });
        // click_area_rect_element.set_attribute(SvgAttribute::PointerEvents {
        //     pointer_events: SvgPointerEventsVariants::All,
        // });
        root_element.append_child_in_node_context(entity, &mut click_area_rect_element);

        // Create fill elements

        let mut fill_clip_path_element = cx.create_element("clip-path");
        #[cfg(feature = "tracing")]
        fill_clip_path_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                fill_clip_path_element.get_id(),
                String::from("fill-clip-path"),
                true,
            ),
        });
        defs_element.append_child_in_node_context(entity, &mut fill_clip_path_element);

        let mut fill_clipped_path_element = cx.create_element("path");
        #[cfg(feature = "tracing")]
        fill_clipped_path_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                fill_clipped_path_element.get_id(),
                String::from("fill-clipped-path"),
                false,
            ),
        });
        fill_clip_path_element.append_child_in_node_context(entity, &mut fill_clipped_path_element);

        let mut fill_wrapper_g_element = cx.create_element("group");
        #[cfg(feature = "tracing")]
        fill_wrapper_g_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                fill_wrapper_g_element.get_id(),
                String::from("fill-wrapper-g"),
                false,
            ),
        });
        // fill_clip_path_element.set_attribute(SvgAttribute::PointerEvents {
        //     pointer_events: SvgPointerEventsVariants::None,
        // });
        fill_wrapper_g_element.set_attribute(SvgAttribute::ClipPath {
            clip_path: fill_clip_path_element.get_id(),
        });
        root_element.append_child_in_node_context(entity, &mut fill_wrapper_g_element);

        Self {
            root: root_element,
            defs: defs_element,

            // Click area elements
            click_area_rect: click_area_rect_element,

            // Fill elements
            fill_clip_path: fill_clip_path_element,
            fill_clipped_path: fill_clipped_path_element,
            fill_wrapper_g: fill_wrapper_g_element,
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: SvgElementId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "_def" } else { "" };
        format!("shape_{}_{}{}", category, id, def_part)
    }
}

pub fn insert_shape_svg_node(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<
        Entity,
        (
            With<CompNode>,
            Or<(
                With<RectangleCompNode>,
                With<TextCompNode>,
                With<PolygonCompNode>,
                With<EllipseCompNode>,
                With<StarCompNode>,
            )>,
            Without<ShapeSvgNode>,
        ),
    >,
) {
    query.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert(ShapeSvgNode::new(entity, &mut svg_context_res));
    });
}

pub fn apply_shape_node_size_change(
    mut query: Query<(&SizeMixin, &mut ShapeSvgNode), (With<CompNode>, Changed<SizeMixin>)>,
) {
    query.iter_mut().for_each(|(SizeMixin(size), mut node)| {
        let [width, height] = size.0.to_array();

        node.root.set_attributes(vec![
            SvgAttribute::Width {
                width,
                unit: SvgMeasurementUnit::Pixel,
            },
            SvgAttribute::Height {
                height,
                unit: SvgMeasurementUnit::Pixel,
            },
        ]);
        node.click_area_rect.set_attributes(vec![
            SvgAttribute::Width {
                width,
                unit: SvgMeasurementUnit::Pixel,
            },
            SvgAttribute::Height {
                height,
                unit: SvgMeasurementUnit::Pixel,
            },
        ]);
    });
}
