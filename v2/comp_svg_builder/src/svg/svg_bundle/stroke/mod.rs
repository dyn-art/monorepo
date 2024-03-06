pub mod solid;

use bevy_ecs::entity::Entity;

use self::solid::SolidStrokeSvgBundle;

use super::SvgBundle;

#[derive(Debug, Clone)]
pub enum StrokeSvgBundle {
    Solid(SolidStrokeSvgBundle),
}

impl StrokeSvgBundle {
    pub fn get_paint_entity(&self) -> &Entity {
        match self {
            StrokeSvgBundle::Solid(bundle) => &bundle.paint_entity,
        }
    }

    pub fn get_svg_bundle(&self) -> &dyn SvgBundle {
        match self {
            StrokeSvgBundle::Solid(bundle) => bundle,
        }
    }

    pub fn get_svg_bundle_mut(&mut self) -> &mut dyn SvgBundle {
        match self {
            StrokeSvgBundle::Solid(bundle) => bundle,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StrokeSvgBundle::Solid(bundle) => bundle.get_root_element().to_string(bundle, None),
        }
    }
}
