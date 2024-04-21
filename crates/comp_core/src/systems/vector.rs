use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Local, Query},
};
use dyn_comp_bundles::components::{
    mixins::{PathMixin, SizeMixin},
    nodes::VectorCompNode,
};
use dyn_utils::properties::size::Size;
use std::collections::HashMap;

pub fn resize_vector_node(
    mut query: Query<
        (Entity, &mut PathMixin, &SizeMixin),
        (Changed<SizeMixin>, With<VectorCompNode>),
    >,
    mut prev_sizes: Local<HashMap<Entity, Size>>,
) {
    for (entity, mut path_mixin, SizeMixin(size)) in query.iter_mut() {
        if let Some(&prev_size) = prev_sizes.get(&entity) {
            let transform = tiny_skia_path::Transform {
                sx: size.width() / prev_size.width(),
                sy: size.height() / prev_size.height(),
                kx: 0.0,
                ky: 0.0,
                tx: 0.0,
                ty: 0.0,
            };

            // Apply the transformation to the path
            if let Some(transformed_path) = path_mixin.path.clone().transform(transform) {
                path_mixin.path = transformed_path;
            }
        }

        // Update the previous size for the next iteration
        prev_sizes.insert(entity, *size);
    }
}
