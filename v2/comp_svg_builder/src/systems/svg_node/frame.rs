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
    entity::Entity,
    query::{Changed, With, Without},
    system::{Commands, Query, ResMut},
};
use bevy_hierarchy::Children;
use dyn_comp_types::{
    mixins::SizeMixin,
    nodes::{CompNode, FrameCompNode},
};
use std::collections::{BTreeMap, HashSet};

use super::SvgNodeVariant;

#[derive(Debug, Clone)]
pub struct FrameSvgNode {
    root: SvgElement,
    defs: SvgElement,

    // Content elements
    content_clip_path: SvgElement,
    content_clipped_rect: SvgElement,
    content_wrapper_g: SvgElement,

    // Children elements
    children_wrapper_g: SvgElement,

    // Fill elements
    fill_clip_path: SvgElement,
    fill_clipped_path: SvgElement,
    fill_wrapper_g: SvgElement,

    // Children
    node_children: Vec<Entity>,
}

impl SvgNode for FrameSvgNode {
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

impl FrameSvgNode {
    pub fn new(entity: Entity, cx: &mut SvgContextRes) -> Self {
        log::info!("[FrameSvgNode::new] {:?}", entity);

        let mut root_element = cx.create_bundle_root_element("group", entity);
        #[cfg(feature = "tracing")]
        root_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(root_element.get_id(), String::from("root"), false),
        });

        let mut defs_element = cx.create_element("dref");
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(defs_element.get_id(), String::from("defs"), false),
        });
        root_element.append_child_in_world_context(entity, &mut defs_element);

        // Create content elements

        let mut content_clip_path_element = cx.create_element("clip-path");
        #[cfg(feature = "tracing")]
        content_clip_path_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                content_clip_path_element.get_id(),
                String::from("content-clip-path"),
                true,
            ),
        });
        defs_element.append_child_in_node_context(entity, &mut content_clip_path_element);

        let mut content_clipped_rect_element = cx.create_element("rect");
        #[cfg(feature = "tracing")]
        content_clipped_rect_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                content_clipped_rect_element.get_id(),
                String::from("content-clipped-rect"),
                false,
            ),
        });
        content_clip_path_element
            .append_child_in_node_context(entity, &mut content_clipped_rect_element);

        let mut content_wrapper_g_element = cx.create_element("group");
        #[cfg(feature = "tracing")]
        content_wrapper_g_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                content_wrapper_g_element.get_id(),
                String::from("content-wrapper-g"),
                false,
            ),
        });
        content_wrapper_g_element.set_attribute(SvgAttribute::ClipPath {
            clip_path: content_clip_path_element.get_id(),
        });
        root_element.append_child_in_node_context(entity, &mut content_wrapper_g_element);

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

        let mut fill_clipped_path_element = cx.create_element("rect");
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
        fill_wrapper_g_element.set_attribute(SvgAttribute::ClipPath {
            clip_path: fill_clip_path_element.get_id(),
        });
        content_wrapper_g_element.append_child_in_node_context(entity, &mut fill_wrapper_g_element);

        // Create children wrapper element

        let mut children_wrapper_g_element = cx.create_element("group");
        #[cfg(feature = "tracing")]
        children_wrapper_g_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                children_wrapper_g_element.get_id(),
                String::from("children-wrapper-g"),
                false,
            ),
        });
        content_wrapper_g_element
            .append_child_in_node_context(entity, &mut children_wrapper_g_element);

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
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: SvgElementId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "_def" } else { "" };
        format!("frame_{}_{}{}", category, id, def_part)
    }
}

pub fn insert_frame_svg_node(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<Entity, (With<CompNode>, With<FrameCompNode>, Without<SvgNodeVariant>)>,
) {
    query.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert(SvgNodeVariant::Frame(FrameSvgNode::new(
                entity,
                &mut svg_context_res,
            )));
    });
}

struct NodeModification {
    parent_entity: Entity,
    added_entities: Vec<Entity>,
    removed_entities: Vec<Entity>,
}

pub fn apply_frame_node_children_change(
    query: Query<
        (Entity, &Children, &mut SvgNodeVariant),
        (With<CompNode>, With<FrameCompNode>, Changed<Children>),
    >,
    // mut node_query: Query<&mut SvgNodeVariant>, // TODO: Put into separate system and store NodeModifications in Resource
) {
    let mut modifications = Vec::new();

    // Identify modifications
    for (entity, children, node_variant) in query.iter() {
        if let SvgNodeVariant::Frame(node) = node_variant {
            let current_node_children_set: HashSet<_> =
                node.node_children.iter().cloned().collect();
            let new_node_children_set: HashSet<_> = children.iter().cloned().collect();

            let removed_node_entities: Vec<_> = current_node_children_set
                .difference(&new_node_children_set)
                .cloned()
                .collect();

            let added_node_entities: Vec<_> = new_node_children_set
                .difference(&current_node_children_set)
                .cloned()
                .collect();

            modifications.push(NodeModification {
                parent_entity: entity,
                added_entities: added_node_entities,
                removed_entities: removed_node_entities,
            });
        }
    }

    // TODO: Put into separate system due to conflicting queries
    // Apply modifications
    // for modification in modifications {
    //     // Process removed entities
    //     for entity in modification.removed_entities {
    //         if let Ok(mut to_remove_node) = node_query.get_mut(entity) {
    //             // TODO
    //         }
    //     }

    //     // Process added entities
    //     for entity in modification.added_entities {
    //         if let Ok(mut added_node) = node_query.get_mut(entity) {
    //             // TODO
    //         }
    //     }
    // }
}

// pub fn apply_frame_node_children_change(
//     mut commands: Commands,
//     mut query: Query<
//         (&Children, &mut SvgNodeVariant),
//         (With<CompNode>, With<FrameCompNode>, Changed<Children>),
//     >,
//     mut node_query: Query<&mut SvgNodeVariant>,
// ) {
//     query.iter_mut().for_each(|(children, mut node_variant)| {
//         let node = match node_variant.as_mut() {
//             SvgNodeVariant::Frame(node) => node,
//             _ => {
//                 return;
//             }
//         };

//         // Identify added and removed node elements
//         let current_node_children_set: HashSet<_> = node.node_children.iter().cloned().collect();
//         let new_node_children_set: HashSet<_> = children.iter().cloned().collect();
//         let removed_node_entities: Vec<_> = node
//             .node_children
//             .iter()
//             .filter(|&e| !new_node_children_set.contains(e))
//             .cloned()
//             .collect();
//         let added_node_enntities: Vec<_> = children
//             .iter()
//             .filter(|&e| !current_node_children_set.contains(e))
//             .cloned()
//             .collect();

//         // Process removed entities
//         for entity in removed_node_entities {
//             if let Ok(to_remove_node) = node_query.get(entity) {
//                 node.children_wrapper_g
//                     .remove_child(to_remove_node.get_svg_node().get_root_element().get_id());
//                 commands.entity(entity).despawn();
//             }
//         }

//         // Process added entities
//         for entity in added_node_enntities {
//             if let Ok(mut added_node) = node_query.get_mut(entity) {
//                 node.children_wrapper_g.append_child_in_world_context(
//                     entity,
//                     added_node.get_svg_node_mut().get_root_element_mut(),
//                 );
//             }
//         }

//         // Reorder entities
//         // TODO

//         node.node_children = children.iter().cloned().collect();
//     });
// }

pub fn apply_frame_node_size_change(
    mut query: Query<(&SizeMixin, &mut SvgNodeVariant), (With<CompNode>, Changed<SizeMixin>)>,
) {
    query
        .iter_mut()
        .for_each(|(SizeMixin(size), mut node_variant)| {
            let node = match node_variant.as_mut() {
                SvgNodeVariant::Frame(node) => node,
                _ => {
                    return;
                }
            };
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
            node.fill_clipped_path.set_attributes(vec![
                SvgAttribute::Width {
                    width,
                    unit: SvgMeasurementUnit::Pixel,
                },
                SvgAttribute::Height {
                    height,
                    unit: SvgMeasurementUnit::Pixel,
                },
            ]);
            node.content_clipped_rect.set_attributes(vec![
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
