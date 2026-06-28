use crate::canvas::{Canvas, SurfaceArea};
use crate::preview_theme_bridge::KucThemeBridge;
use katana_document_viewer::KdvThemeSnapshot;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiTextProps};
use katana_ui_core_storybook::UiTreeStorybookHost;

#[test]
fn kuc_renderer_uses_kdv_table_theme_tokens() -> Result<(), Box<dyn std::error::Error>> {
    let theme = KucThemeBridge::from_kdv(&KdvThemeSnapshot::katana_light())?;
    let mut canvas = Canvas::new(420, 180, 0xffffff);
    let table = UiNode::new(
        UiNodeKind::Text,
        "Feature | Status\nHTML alignment | covered",
    )
    .text(UiTextProps {
        role: "table".to_string(),
        ..UiTextProps::default()
    })
    .height(UiDimension::Px(0));

    UiTreeStorybookHost::new(theme).render(
        &mut canvas,
        &table,
        SurfaceArea {
            x: 0,
            y: 0,
            width: 420,
            height: 180,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(0xf3f3f3), pixel_at(&canvas, 20, 20));
    assert_eq!(Some(0xffffff), pixel_at(&canvas, 20, 72));
    assert_eq!(0, count_pixel(&canvas, 0x1f2a30));
    Ok(())
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas
        .pixels()
        .get(y.saturating_mul(canvas.width()) + x)
        .copied()
}

fn count_pixel(canvas: &Canvas, expected: u32) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|pixel| **pixel == expected)
        .count()
}
