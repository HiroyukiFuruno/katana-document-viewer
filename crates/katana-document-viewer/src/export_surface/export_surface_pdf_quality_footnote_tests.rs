use crate::ExportFormat;
use crate::export_payload::ExportPayloadFactory;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

#[test]
fn pdf_surface_footnote_definitions_have_backlinks() -> Result<(), Box<dyn std::error::Error>> {
    let text = SurfaceTestSupport::surface_text(&SurfaceTestSupport::graph_from_markdown(
        "footnote.md",
        footnote_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &text,
        &[
            "これは脚注付きのテキストです[1]。",
            "1. 最初の脚注の内容。 ↩",
        ],
    );
    SurfaceTestSupport::assert_not_contains_any(&text, &["[1] ↩ 最初の脚注の内容。"]);
    Ok(())
}

#[test]
fn pdf_payload_uses_document_internal_links_for_footnotes() -> Result<(), Box<dyn std::error::Error>>
{
    let graph = SurfaceTestSupport::graph_from_markdown("footnote.md", footnote_markdown())?;
    let pdf = ExportPayloadFactory::create(
        &graph,
        ExportFormat::Pdf,
        &crate::KdvThemeSnapshot::katana_light(),
    )?;
    let text = String::from_utf8_lossy(&pdf);

    assert!(text.contains("/Dest ["), "{text}");
    assert!(!text.contains("/S /GoTo"), "{text}");
    assert!(!text.contains("/URI (#fn-1)"), "{text}");
    assert!(!text.contains("/URI (#fnref-1)"), "{text}");
    Ok(())
}

#[test]
fn pdf_surface_places_footnotes_after_following_body() -> Result<(), Box<dyn std::error::Error>> {
    let text = SurfaceTestSupport::surface_text(&SurfaceTestSupport::graph_from_markdown(
        "footnote-bottom.md",
        footnote_with_following_body_markdown(),
    )?);

    let body_position = text
        .find("脚注定義の後に続く本文。")
        .ok_or("following body is missing")?;
    let footnote_position = text
        .find("1. 脚注本文。 ↩")
        .ok_or("footnote definition is missing")?;

    assert!(
        footnote_position > body_position,
        "footnotes must be collected at the bottom instead of interrupting body flow: {text}"
    );
    Ok(())
}

fn footnote_markdown() -> String {
    [
        "# footnote",
        "",
        "これは脚注付きのテキストです[^1]。",
        "",
        "[^1]: 最初の脚注の内容。",
    ]
    .join("\n")
}

fn footnote_with_following_body_markdown() -> String {
    [
        "# footnote",
        "",
        "これは脚注付きのテキストです[^1]。",
        "",
        "[^1]: 脚注本文。",
        "",
        "脚注定義の後に続く本文。",
    ]
    .join("\n")
}
