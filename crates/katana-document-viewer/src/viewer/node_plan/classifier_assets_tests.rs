use super::{
    super::{ViewerNodeClassifier, ViewerNodeKind},
    test_support::{image, node},
};
use crate::artifact::ArtifactFormat;
use katana_markdown_model::{HtmlBlockRole, ImageNode, KmmNode, KmmNodeKind, TextSpan};

#[test]
fn standalone_image_only_matches_paragraph_image_markdown() {
    assert!(ViewerNodeClassifier::standalone_image(&paragraph_image_node()).is_some());
    assert!(ViewerNodeClassifier::standalone_image(&image_only_child_node()).is_none());
    assert!(ViewerNodeClassifier::standalone_image(&no_image_child_node()).is_none());
    assert!(ViewerNodeClassifier::standalone_image(&duplicated_images_node()).is_none());
}

fn paragraph_image_node() -> KmmNode {
    node(
        KmmNodeKind::Paragraph,
        "![diagram](diagram.svg)",
        vec![node(
            KmmNodeKind::Image(image("diagram")),
            "![diagram](diagram.svg)",
            Vec::new(),
        )],
    )
}

fn image_only_child_node() -> KmmNode {
    let text_node = node(
        KmmNodeKind::Text(TextSpan {
            text: "diagram".to_string(),
        }),
        "diagram",
        Vec::new(),
    );
    node(
        KmmNodeKind::Image(image("diagram")),
        "![diagram](diagram.svg)",
        vec![text_node],
    )
}

fn no_image_child_node() -> KmmNode {
    node(
        KmmNodeKind::Paragraph,
        "![diagram](diagram.svg)",
        vec![node(
            KmmNodeKind::Text(TextSpan {
                text: "diagram".to_string(),
            }),
            "diagram",
            Vec::new(),
        )],
    )
}

fn duplicated_images_node() -> KmmNode {
    node(
        KmmNodeKind::Paragraph,
        "![diagram](diagram.svg)",
        vec![
            node(
                KmmNodeKind::Image(image("diagram")),
                "![diagram](diagram.svg)",
                Vec::new(),
            ),
            node(
                KmmNodeKind::Image(image("diagram")),
                "![diagram](diagram.svg)",
                Vec::new(),
            ),
        ],
    )
}

#[test]
fn image_format_is_none_when_extension_is_unknown() {
    let node = node(
        KmmNodeKind::Image(ImageNode {
            alt: "diagram".to_string(),
            src: "diagram.bin".to_string(),
            title: None,
        }),
        "![diagram](diagram.bin)",
        Vec::new(),
    );

    assert_eq!(
        None,
        ViewerNodeClassifier::asset_format(&node, &ViewerNodeKind::Image)
    );
}

#[test]
fn image_format_respects_query_and_fragment_suffixes() {
    let node = node(
        KmmNodeKind::Image(ImageNode {
            alt: "screen".to_string(),
            src: "screen.webp?cache=1#frag".to_string(),
            title: None,
        }),
        "![screen](screen.webp?cache=1#frag)",
        Vec::new(),
    );

    assert_eq!(
        Some(ArtifactFormat::Webp),
        ViewerNodeClassifier::asset_format(&node, &ViewerNodeKind::Image)
    );
    assert_eq!(
        Some("screen.webp?cache=1#frag"),
        ViewerNodeClassifier::image_source(&node)
    );
}

#[test]
fn image_source_prefers_standalone_image_child() {
    let node = node(
        KmmNodeKind::Paragraph,
        "![screen](screen.png)",
        vec![node(
            KmmNodeKind::Image(image("screen")),
            "![screen](screen.png)",
            Vec::new(),
        )],
    );

    assert_eq!(Some("image.png"), ViewerNodeClassifier::image_source(&node));
    assert_eq!(
        Some("image.png"),
        ViewerNodeClassifier::standalone_image(&node).map(|image| image.src.as_str())
    );
}

#[test]
fn details_html_is_detected_from_source_markup() {
    let node = node(
        KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        "<details><summary>Title</summary><p>Body</p></details>",
        Vec::new(),
    );

    assert!(ViewerNodeClassifier::is_details_html(&node));
}
