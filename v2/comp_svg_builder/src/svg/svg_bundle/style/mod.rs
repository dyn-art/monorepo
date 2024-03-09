pub mod solid;

use self::solid::SolidStyleSvgBundle;
use super::SvgBundle;
use bevy_ecs::{component::Component, entity::Entity};

#[derive(Debug, Clone)]
pub enum StyleSvgBundle {
    Solid(SolidStyleSvgBundle),
    // Gradient
    // Image
    // Drop Shadow
}

impl StyleSvgBundle {
    pub fn get_paint_entity(&self) -> &Entity {
        match self {
            StyleSvgBundle::Solid(bundle) => &bundle.entity,
        }
    }

    pub fn get_svg_bundle(&self) -> &dyn SvgBundle {
        match self {
            StyleSvgBundle::Solid(bundle) => bundle,
        }
    }

    pub fn get_svg_bundle_mut(&mut self) -> &mut dyn SvgBundle {
        match self {
            StyleSvgBundle::Solid(bundle) => bundle,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StyleSvgBundle::Solid(bundle) => bundle.get_root_element().to_string(bundle, None),
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct StyleSvgBundleMixin(pub StyleSvgBundle);
