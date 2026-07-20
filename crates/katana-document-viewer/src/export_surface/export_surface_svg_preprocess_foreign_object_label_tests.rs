use super::to_svg_text;

#[test]
fn foreign_object_right_alignment_uses_end_anchor_and_width() -> Result<(), String> {
    let svg = to_svg_text(
        r#"<foreignObject x="2" y="3" width="10" style="text-align:right;color:&quot;red&quot;"><div>Right</div></foreignObject>"#,
    )
    .ok_or_else(|| "right-aligned label".to_string())?;

    assert!(svg.contains(r#"x="12""#));
    assert!(svg.contains(r#"text-anchor="end""#));
    assert!(svg.contains(r#"fill="&quot;red&quot;""#));
    Ok(())
}

#[test]
fn foreign_object_left_alignment_collects_breaks_cdata_and_ignores_comments() -> Result<(), String>
{
    let svg = to_svg_text(
        r#"<foreignObject x="4" y="5" style="text-align:left"><div>Alpha<br/><![CDATA[Beta]]><!-- hidden --></div></foreignObject>"#,
    )
    .ok_or_else(|| "left-aligned label".to_string())?;

    assert!(svg.contains(r#"x="4""#));
    assert!(svg.contains(r#"text-anchor="start""#));
    assert!(svg.contains(">Alpha Beta</text>"));
    assert!(!svg.contains("hidden"));
    Ok(())
}

#[test]
fn unknown_alignment_defaults_to_middle() -> Result<(), String> {
    let svg = to_svg_text(
        r#"<foreignObject x="1" width="8" style="text-align:justify"><span>Middle</span></foreignObject>"#,
    )
    .ok_or_else(|| "middle-aligned label".to_string())?;

    assert!(svg.contains(r#"x="5""#));
    assert!(svg.contains(r#"text-anchor="middle""#));
    Ok(())
}
