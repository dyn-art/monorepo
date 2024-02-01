use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query},
};

use crate::modules::node::components::mixins::PathMixin;

// https://stackoverflow.com/questions/13416693/svg-generate-outline-path/13445211#13445211
// https://stackoverflow.com/questions/13643864/how-to-get-the-outline-of-a-stroke
// https://github.com/fracalo/svg-contour/tree/master
// https://github.com/danmarshall/svg-path-outline
// https://github.com/Microsoft/maker.js
// https://danmarshall.github.io/google-font-to-svg-path/
pub fn update_stroke_path(mut commands: Commands, mut query: Query<(Entity, &mut PathMixin)>) {
    for (entity, path_mixin) in query.iter() {
        // TODO
    }
}
