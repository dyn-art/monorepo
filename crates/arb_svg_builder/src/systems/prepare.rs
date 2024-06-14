use crate::{
    resources::svg_context::SvgContextRes,
    svg::svg_bundle::{
        node::{frame::FrameNodeSvgBundle, shape::ShapeNodeSvgBundle},
        style::{
            drop_shadow_effect::DropShadowEffectStyleSvgBundle,
            gradient_fill::GradientFillStyleSvgBundle, image_fill::ImageFillStyleSvgBundle,
            solid_fill::SolidFillStyleSvgBundle,
        },
        SvgBundleVariant,
    },
};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Commands, Query, ResMut},
};
use dyn_arb_bundles::components::{
    mixins::{PaintChildMixin, SizeMixin, StyleChildrenMixin, StyleParentMixin},
    nodes::{ArbNode, ArbNodeVariant},
    paints::{ArbPaint, ArbPaintVariant, GradientArbPaint, ImageArbPaint},
    styles::{ArbStyle, ArbStyleVariant},
};

pub fn insert_node_svg_bundle(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    query: Query<(Entity, &ArbNode), (With<ArbNode>, Without<SvgBundleVariant>)>,
) {
    for (entity, ArbNode { variant }) in query.iter() {
        let bundle_variant =
            match variant {
                ArbNodeVariant::Frame => Some(SvgBundleVariant::FrameNode(
                    FrameNodeSvgBundle::new(entity, &mut svg_context_res),
                )),
                ArbNodeVariant::Rectangle
                | ArbNodeVariant::Ellipse
                | ArbNodeVariant::Polygon
                | ArbNodeVariant::Star
                | ArbNodeVariant::Text
                | ArbNodeVariant::Vector => Some(SvgBundleVariant::ShapeNode(
                    ShapeNodeSvgBundle::new(entity, &mut svg_context_res),
                )),
            };

        if let Some(bundle_variant) = bundle_variant {
            commands.entity(entity).insert(bundle_variant);
        }
    }
}

pub fn insert_style_svg_bundle(
    mut commands: Commands,
    mut svg_context_res: ResMut<SvgContextRes>,
    #[cfg(feature = "output_svg_element_changes")] mut changed_svg_bundles_res: ResMut<
        crate::resources::changed_svg_bundles::ChangedSvgBundlesRes,
    >,
    mut query: Query<
        (
            Entity,
            &ArbStyle,
            Option<&PaintChildMixin>,
            Option<&StyleParentMixin>,
            Option<&mut SvgBundleVariant>,
        ),
        (
            With<ArbStyle>,
            Without<ArbNode>,
            Or<(Without<SvgBundleVariant>, Changed<PaintChildMixin>)>,
        ),
    >,
    paint_query: Query<(&ArbPaint, Option<&GradientArbPaint>, Option<&ImageArbPaint>)>,
    mut svg_bundle_query: Query<&mut SvgBundleVariant, (Without<ArbStyle>, With<ArbNode>)>,
) {
    for (entity, style, maybe_paint_mixin, maybe_style_parent_mixin, maybe_svg_bundle_variant) in
        query.iter_mut()
    {
        let maybe_new_svg_bundle_variant: Option<SvgBundleVariant> = match style.variant {
            ArbStyleVariant::Fill | ArbStyleVariant::Stroke => {
                if let Some(paint_entity) =
                    maybe_paint_mixin.map(|paint_child_mixin| paint_child_mixin.0)
                {
                    if let Ok((paint, maybe_gradient_paint, maybe_image_paint)) =
                        paint_query.get(paint_entity)
                    {
                        match (style.variant, paint.variant) {
                            (
                                ArbStyleVariant::Fill | ArbStyleVariant::Stroke,
                                ArbPaintVariant::Solid,
                            ) => Some(SvgBundleVariant::SolidFill(SolidFillStyleSvgBundle::new(
                                entity,
                                &mut svg_context_res,
                            ))),
                            (
                                ArbStyleVariant::Fill | ArbStyleVariant::Stroke,
                                ArbPaintVariant::Gradient,
                            ) => Some(SvgBundleVariant::GradientFill(
                                GradientFillStyleSvgBundle::new(
                                    entity,
                                    maybe_gradient_paint.unwrap().variant,
                                    &mut svg_context_res,
                                ),
                            )),
                            (
                                ArbStyleVariant::Fill | ArbStyleVariant::Stroke,
                                ArbPaintVariant::Image,
                            ) => Some(SvgBundleVariant::ImageFill(ImageFillStyleSvgBundle::new(
                                entity,
                                maybe_image_paint.unwrap().scale_mode,
                                &mut svg_context_res,
                            ))),
                            _ => None,
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            ArbStyleVariant::DropShadow => Some(SvgBundleVariant::DropShadowEffect(
                DropShadowEffectStyleSvgBundle::new(entity, &mut svg_context_res),
            )),
        };

        // TODO: Very dirty (See README.md for future rewrite)
        if let Some(mut new_svg_bundle_variant) = maybe_new_svg_bundle_variant {
            if let Some(mut svg_bundle_variant) = maybe_svg_bundle_variant {
                if let Some(parent_style_mixin) = maybe_style_parent_mixin {
                    if let Ok(mut parent_svg_bundle) =
                        svg_bundle_query.get_mut(parent_style_mixin.0)
                    {
                        if let Some(parent_style_wrapper_element) =
                            parent_svg_bundle.get_styles_wrapper_element_mut()
                        {
                            parent_style_wrapper_element.append_child_in_bundle_context(
                                new_svg_bundle_variant.get_root_element_mut(),
                            );
                            parent_style_wrapper_element.reorder_children_mut(|children| {
                                if let (Some(idx), Some(new_idx)) = (
                                    children.iter().position(|c| {
                                        c.id == svg_bundle_variant.get_root_element().get_id()
                                    }),
                                    children.iter().position(|c| {
                                        c.id == new_svg_bundle_variant.get_root_element().get_id()
                                    }),
                                ) {
                                    children.swap(idx, new_idx);
                                    children.remove(new_idx);
                                }
                            });
                            parent_style_wrapper_element
                                .remove_child_element(svg_bundle_variant.get_root_element_mut());

                            // Already register changes because the old SvgBundleVariant is removed
                            #[cfg(feature = "output_svg_element_changes")]
                            changed_svg_bundles_res.drain_removed_bundle_changes(
                                svg_bundle_variant.get_svg_bundle_mut(),
                            );
                        }
                    }
                }
            }

            commands.entity(entity).insert(new_svg_bundle_variant);
        } else {
            log::warn!(
                "Failed to create bundle for style variant: {:?}",
                style.variant
            );
        }
    }
}

pub fn propagate_size_mixin_to_style(
    mut commands: Commands,
    node_query: Query<
        (Entity, &SizeMixin, &StyleChildrenMixin),
        (With<ArbNode>, Without<ArbStyle>, Changed<SizeMixin>),
    >,
    mut style_with_size_query: Query<
        (Entity, &StyleParentMixin, &mut SizeMixin),
        (With<ArbStyle>, Without<ArbNode>, With<SizeMixin>),
    >,
    style_without_size_query: Query<
        (Entity, &StyleParentMixin),
        (With<ArbStyle>, Without<ArbNode>, Without<SizeMixin>),
    >,
) {
    for (node_entity, SizeMixin(size), StyleChildrenMixin(children)) in node_query.iter() {
        // Update existing DimensionMixin for children with DimensionMixin
        for (paint_entity, StyleParentMixin(parent), mut size_mixin) in
            style_with_size_query.iter_mut()
        {
            if children.contains(&paint_entity) && *parent == node_entity {
                size_mixin.0 = *size;
            }
        }

        // Add DimensionMixin for children without DimensionMixin
        for (paint_entity, StyleParentMixin(parent)) in style_without_size_query.iter() {
            if children.contains(&paint_entity) && *parent == node_entity {
                commands.entity(paint_entity).insert(SizeMixin(*size));
            }
        }
    }
}
