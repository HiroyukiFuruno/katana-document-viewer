use crate::canvas::SurfaceArea;
use crate::frame::FrameRenderRequest;
use crate::layout::{
    SIDEBAR_CONTENT_INSET, sidebar_content_height, sidebar_content_width, sidebar_content_x,
};
use katana_ui_core::render_model::UiNode;
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::UiTreeSurfaceHost;
use std::cell::RefCell;

thread_local! {
    static DARK_RENDERER: RefCell<UiTreeSurfaceHost> =
        RefCell::new(UiTreeSurfaceHost::new(ThemeSnapshot::dark()));
    static LIGHT_RENDERER: RefCell<UiTreeSurfaceHost> =
        RefCell::new(UiTreeSurfaceHost::new(ThemeSnapshot::light()));
    static THEME_RENDERERS: RefCell<Vec<ThemeRendererCache>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn render_ui_tree_for_theme_flag(
    canvas: &mut crate::canvas::Canvas,
    root: &UiNode,
    area: SurfaceArea,
    dark: bool,
) {
    if dark {
        DARK_RENDERER.with(|renderer| renderer.borrow().render(canvas, root, area));
        return;
    }
    LIGHT_RENDERER.with(|renderer| renderer.borrow().render(canvas, root, area));
}

pub(crate) fn render_ui_tree_with_theme(
    canvas: &mut crate::canvas::Canvas,
    root: &UiNode,
    area: SurfaceArea,
    theme: &ThemeSnapshot,
) {
    if theme.eq(&ThemeSnapshot::dark()) {
        render_ui_tree_for_theme_flag(canvas, root, area, true);
        return;
    }
    if theme.eq(&ThemeSnapshot::light()) {
        render_ui_tree_for_theme_flag(canvas, root, area, false);
        return;
    }
    THEME_RENDERERS.with(|renderers| {
        let mut renderers = renderers.borrow_mut();
        let index = renderer_index(&mut renderers, theme);
        renderers[index].renderer.render(canvas, root, area);
    });
}

pub(crate) fn sidebar_area(request: &FrameRenderRequest<'_>) -> SurfaceArea {
    SurfaceArea {
        x: sidebar_content_x(),
        y: SIDEBAR_CONTENT_INSET,
        width: sidebar_content_width(),
        height: sidebar_content_height(request.height),
        scroll_y: 0.0,
    }
}

fn renderer_index(renderers: &mut Vec<ThemeRendererCache>, theme: &ThemeSnapshot) -> usize {
    if let Some(index) = renderers.iter().position(|cached| cached.theme.eq(theme)) {
        return index;
    }
    renderers.push(ThemeRendererCache {
        theme: theme.clone(),
        renderer: UiTreeSurfaceHost::new(theme.clone()),
    });
    renderers.len() - 1
}

struct ThemeRendererCache {
    theme: ThemeSnapshot,
    renderer: UiTreeSurfaceHost,
}
