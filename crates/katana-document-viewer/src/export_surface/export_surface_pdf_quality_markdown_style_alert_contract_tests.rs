use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

#[test]
fn pdf_surface_renders_gfm_alerts_as_alert_blocks() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "alerts.md",
        alerts_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "alert:NOTE:icon=note-circle:Note",
            "alert:TIP:icon=tip-bulb:Tip",
            "alert:IMPORTANT:icon=important-callout:Important",
            "alert:WARNING:icon=warning-triangle:Warning",
            "alert:CAUTION:icon=caution-octagon:Caution",
        ],
    );
    SurfaceTestSupport::assert_not_contains_any(
        &debug,
        &["line:ⓘ Note", "△ Warning", "! Caution", "[!NOTE]"],
    );
    Ok(())
}

fn alerts_markdown() -> String {
    [
        "> [!NOTE]",
        "> note body",
        "",
        "> [!TIP]",
        "> tip body",
        "",
        "> [!IMPORTANT]",
        "> important body",
        "",
        "> [!WARNING]",
        "> warning body",
        "",
        "> [!CAUTION]",
        "> caution body",
    ]
    .join("\n")
}
