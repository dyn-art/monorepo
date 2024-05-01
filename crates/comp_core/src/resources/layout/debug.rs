use bevy_ecs::{entity::Entity, system::Query};
use dyn_comp_bundles::components::mixins::LayoutNodeId;
use std::collections::HashMap;
use taffy::{NodeId, PrintTree};

pub fn create_taffy_to_entity_map(
    layout_node_id_query: &Query<(Entity, &LayoutNodeId)>,
) -> HashMap<NodeId, Entity> {
    layout_node_id_query
        .iter()
        .map(|(entity, LayoutNodeId(node_id))| (*node_id, entity))
        .collect()
}

/// Prints a debug representation of the computed layout for a tree of nodes, starting with the passed root node.
///
/// Based on: https://github.com/DioxusLabs/taffy/blob/main/src/util/print.rs
pub fn print_branch(
    tree: &impl PrintTree,
    root: NodeId,
    taffy_to_entity: &HashMap<NodeId, Entity>,
) {
    log::info!("TREE");
    print_node(tree, root, false, String::new(), taffy_to_entity);

    /// Recursive function that prints each node in the tree
    fn print_node(
        tree: &impl PrintTree,
        node_id: NodeId,
        has_sibling: bool,
        lines_string: String,
        taffy_to_entity: &HashMap<NodeId, Entity>,
    ) {
        let entity_string = taffy_to_entity
            .get(&node_id)
            .map(|e| format!("{}v{}", e.index(), e.generation()))
            .unwrap_or(String::from("unknown"));
        let layout = &tree.get_final_layout(node_id);
        let display = tree.get_debug_label(node_id);
        let num_children = tree.child_count(node_id);

        let fork_string = if has_sibling {
            "├── "
        } else {
            "└── "
        };
        log::info!(
                "{lines}{fork} {display} [x: {x:<4} y: {y:<4} w: {width:<4} h: {height:<4} content_w: {content_width:<4} content_h: {content_height:<4} border: l:{bl} r:{br} t:{bt} b:{bb}, padding: l:{pl} r:{pr} t:{pt} b:{pb}] ({key:?} / {entity:?})",
                lines = lines_string,
                fork = fork_string,
                display = display,
                x = layout.location.x,
                y = layout.location.y,
                width = layout.size.width,
                height = layout.size.height,
                content_width = layout.content_size.width,
                content_height = layout.content_size.height,
                bl = layout.border.left,
                br = layout.border.right,
                bt = layout.border.top,
                bb = layout.border.bottom,
                pl = layout.padding.left,
                pr = layout.padding.right,
                pt = layout.padding.top,
                pb = layout.padding.bottom,
                key = node_id,
                entity = entity_string
            );
        let bar = if has_sibling { "│   " } else { "    " };
        let new_string = lines_string + bar;

        // Recurse into children
        for (index, child) in tree.child_ids(node_id).enumerate() {
            let has_sibling = index < num_children - 1;
            print_node(
                tree,
                child,
                has_sibling,
                new_string.clone(),
                taffy_to_entity,
            );
        }
    }
}
