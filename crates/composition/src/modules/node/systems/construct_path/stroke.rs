use bevy_ecs::{
    entity::Entity,
    query::Changed,
    system::{Commands, Query},
};

use crate::modules::node::components::mixins::{Anchor, PathMixin};

// https://github.com/glenzli/paperjs-offset
// https://stackoverflow.com/questions/13416693/svg-generate-outline-path/13445211#13445211
// https://stackoverflow.com/questions/13643864/how-to-get-the-outline-of-a-stroke
// https://github.com/fracalo/svg-contour/tree/master
// https://github.com/danmarshall/svg-path-outline
// https://github.com/Microsoft/maker.js
// https://danmarshall.github.io/google-font-to-svg-path/
pub fn update_stroke_path(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathMixin), Changed<PathMixin>>,
) {
    for (_, mut path_mixin) in query.iter_mut() {
        // TODO
    }
}

fn construct_stroke_shape(path: Vec<Anchor>, stroke_width: f32) -> Vec<Anchor> {
    // TODO
    Vec::new()
}
