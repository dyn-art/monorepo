use bevy_ecs::{
    entity::Entity,
    query::Changed,
    system::{Commands, Query, ResMut},
};
use dyn_attributed_string::outline::tiny_skia_path_builder::TinySkiaPathBuilder;
use dyn_comp_asset::resources::AssetsRes;
use dyn_comp_bundles::components::mixins::{AttributedStringMixin, PathMixin, WindingRule};

pub fn outline_text(
    mut commands: Commands,
    mut assets_res: ResMut<AssetsRes>,
    mut query: Query<(Entity, &AttributedStringMixin), Changed<AttributedStringMixin>>,
) {
    for (entity, AttributedStringMixin(attributed_string)) in query.iter_mut() {
        let maybe_path =
            TinySkiaPathBuilder::outline(&attributed_string, assets_res.get_fonts_book_mut());

        // Insert or update the PathMixin component for the entity
        if let Some(path) = maybe_path {
            commands.entity(entity).insert(PathMixin {
                path,
                winding_rule: WindingRule::Nonzero,
            });
        }
    }
}
