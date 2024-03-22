use crate::usvg::{database::FontsCache, resolve_font, resolved_font::ResolvedFont, text::Font};
use std::sync::Arc;

pub fn resolve_font_from_cache<'a>(
    font: &Font,
    fonts_cache: &'a mut FontsCache,
    fontdb: &fontdb::Database,
) -> Option<&'a Arc<ResolvedFont>> {
    // Check if the font is already in the cache
    if !fonts_cache.contains_key(font) {
        if let Some(resolved_font) = resolve_font(font, fontdb) {
            fonts_cache.insert(font.clone(), Arc::new(resolved_font));
        } else {
            return None;
        }
    }

    return fonts_cache.get(font);
}
