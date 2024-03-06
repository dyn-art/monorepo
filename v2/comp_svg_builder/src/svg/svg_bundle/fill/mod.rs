pub mod gradient;
pub mod solid;

use self::solid::SolidFillSvgBundle;
use super::SvgBundle;
use bevy_ecs::entity::Entity;

#[derive(Debug, Clone)]
pub enum FillSvgBundle {
    Solid(SolidFillSvgBundle),
}

impl FillSvgBundle {
    pub fn get_paint_entity(&self) -> &Entity {
        match self {
            FillSvgBundle::Solid(bundle) => &bundle.paint_entity,
        }
    }

    pub fn get_svg_bundle(&self) -> &dyn SvgBundle {
        match self {
            FillSvgBundle::Solid(bundle) => bundle,
        }
    }

    pub fn get_svg_bundle_mut(&mut self) -> &mut dyn SvgBundle {
        match self {
            FillSvgBundle::Solid(bundle) => bundle,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FillSvgBundle::Solid(bundle) => bundle.get_root_element().to_string(bundle, None),
        }
    }
}
