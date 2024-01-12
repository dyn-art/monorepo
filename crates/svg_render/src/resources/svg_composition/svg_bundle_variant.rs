use crate::events::output_event::RenderUpdateEvent;

use super::{svg_bundle::BaseSVGBundle, svg_node::SVGNode, svg_paint::SVGPaint, SVGCompositionRes};

/// All bundle variants must implement the SVGBundle trait
#[derive(Debug)]
pub enum SVGBundleVariant {
    Node(Box<dyn SVGNode>),
    Paint(Box<dyn SVGPaint>),
}

pub fn get_bundle(bundle_variant: &SVGBundleVariant) -> &BaseSVGBundle {
    match bundle_variant {
        SVGBundleVariant::Node(node) => node.get_bundle(),
        SVGBundleVariant::Paint(paint) => paint.get_bundle(),
    }
}

pub fn get_bundle_mut(bundle_variant: &mut SVGBundleVariant) -> &mut BaseSVGBundle {
    match bundle_variant {
        SVGBundleVariant::Node(node) => node.get_bundle_mut(),
        SVGBundleVariant::Paint(paint) => paint.get_bundle_mut(),
    }
}

pub fn drain_bundle_updates(bundle_variant: &mut SVGBundleVariant) -> Vec<RenderUpdateEvent> {
    match bundle_variant {
        SVGBundleVariant::Node(node) => node.drain_updates(),
        SVGBundleVariant::Paint(paint) => paint.drain_updates(),
    }
}

pub fn bundle_to_string(
    bundle_variant: &SVGBundleVariant,
    composition: &SVGCompositionRes,
) -> String {
    match bundle_variant {
        SVGBundleVariant::Node(node) => node.to_string(composition),
        SVGBundleVariant::Paint(paint) => paint.to_string(composition),
    }
}
