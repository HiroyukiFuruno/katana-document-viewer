use cosmic_text::FontSystem;

pub(super) fn font_system_with_embedded_fallbacks() -> FontSystem {
    let mut font_system = FontSystem::new();
    load_embedded_fallback_fonts(&mut font_system);
    font_system
}

fn load_embedded_fallback_fonts(font_system: &mut FontSystem) {
    let database = font_system.db_mut();
    database.load_font_data(epaint_default_fonts::UBUNTU_LIGHT.to_vec());
    database.load_font_data(epaint_default_fonts::HACK_REGULAR.to_vec());
    database.load_font_data(epaint_default_fonts::NOTO_EMOJI_REGULAR.to_vec());
    database.load_font_data(epaint_default_fonts::EMOJI_ICON.to_vec());
}
