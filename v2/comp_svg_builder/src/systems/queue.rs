use bevy_ecs::system::ResMut;

use crate::resources::{
    changed_svg_nodes::ChangedSVGNodesRes,
    svg_render_output_event_sender::SVGRenderOutputEventSenderRes,
};

pub fn queue_svg_node_changes(
    mut changed_svg_nodes_res: ResMut<ChangedSVGNodesRes>,
    mut svg_render_output_event_sender_res: ResMut<SVGRenderOutputEventSenderRes>,
) {
    // TODO
}
