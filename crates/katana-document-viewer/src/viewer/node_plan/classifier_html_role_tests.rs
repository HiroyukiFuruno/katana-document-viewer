use super::super::{ViewerHtmlRole, ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::node;
use crate::ViewerHtmlAlignment;
use katana_markdown_model::{HtmlBlockRole, KmmNodeKind};

#[test]
fn viewer_quality_maps_single_quoted_uppercase_html_align_as_role() {
    let item = node(
        KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        "<P ALIGN='RIGHT'>Right</P>",
        Vec::new(),
    );

    assert_eq!(
        Some(ViewerNodeKind::Html {
            role: ViewerHtmlRole::Right
        }),
        ViewerNodeClassifier::node_kind_for_node(&item)
    );
}

#[test]
fn viewer_quality_maps_center_html_align_as_role() {
    for raw in [
        r#"<p align=center>Center</p>"#,
        r#"<p style="text-align: center">Center</p>"#,
    ] {
        let item = node(
            KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
            raw,
            Vec::new(),
        );

        assert_eq!(
            Some(ViewerNodeKind::Html {
                role: ViewerHtmlRole::Centered
            }),
            ViewerNodeClassifier::node_kind_for_node(&item)
        );
    }
}

#[test]
fn viewer_quality_maps_centered_shields_images_as_badge_row() {
    let item = node(
        KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        r#"<p align="center"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License"></p>"#,
        Vec::new(),
    );

    assert_eq!(
        Some(ViewerNodeKind::Html {
            role: ViewerHtmlRole::BadgeRow
        }),
        ViewerNodeClassifier::node_kind_for_node(&item)
    );
}

#[test]
fn viewer_quality_maps_aligned_html_heading_with_level() {
    let item = node(
        KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        r#"<h1 align="center">KatanA Desktop</h1>"#,
        Vec::new(),
    );

    assert_eq!(
        Some(ViewerNodeKind::Html {
            role: ViewerHtmlRole::Heading {
                level: 1,
                alignment: ViewerHtmlAlignment::Center
            }
        }),
        ViewerNodeClassifier::node_kind_for_node(&item)
    );
}

#[test]
fn viewer_quality_maps_uppercase_single_quoted_heading_alignment_with_level() {
    let item = node(
        KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        "<H2 ALIGN='CENTER'>Heading</H2>",
        Vec::new(),
    );

    assert_eq!(
        Some(ViewerNodeKind::Html {
            role: ViewerHtmlRole::Heading {
                level: 2,
                alignment: ViewerHtmlAlignment::Center
            }
        }),
        ViewerNodeClassifier::node_kind_for_node(&item)
    );
}

#[test]
fn viewer_quality_maps_left_html_align_as_role() {
    let item = node(
        KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        r#"<p style="text-align: left">Left</p>"#,
        Vec::new(),
    );

    assert_eq!(
        Some(ViewerNodeKind::Html {
            role: ViewerHtmlRole::Left
        }),
        ViewerNodeClassifier::node_kind_for_node(&item)
    );
}

#[test]
fn viewer_quality_maps_right_html_align_without_quotes_as_role() {
    let item = node(
        KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        "<p align=right>Right</p>",
        Vec::new(),
    );

    assert_eq!(
        Some(ViewerNodeKind::Html {
            role: ViewerHtmlRole::Right
        }),
        ViewerNodeClassifier::node_kind_for_node(&item)
    );
}
