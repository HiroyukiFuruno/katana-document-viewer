use crate::ArtifactId;
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};

#[test]
fn toc_item_becomes_scroll_to_heading_command() -> Result<(), String> {
    let item = crate::viewer::ViewerTocItem {
        level: 2,
        text: "Last heading".to_string(),
        node_id: KmmNodeId("heading-node".to_string()),
        source: source("## Last heading"),
        anchor_rect: rect(8.0, 720.0),
        anchor_index: 7,
    };

    let command = crate::viewer::ViewerCommandFactory::scroll_to_toc_item(item.clone());

    let crate::viewer::ViewerCommand::ScrollToHeading(scroll) = command else {
        return Err("expected TOC scroll command".to_string());
    };
    assert_eq!(item.node_id, scroll.target.node_id);
    assert_eq!(item.source, scroll.target.source);
    assert_eq!(item.anchor_rect, scroll.target.rect);
    assert_eq!(ArtifactId("toc:7".to_string()), scroll.target.artifact_id);
    Ok(())
}

#[test]
fn search_navigation_becomes_search_command() -> Result<(), String> {
    let target = search_target(0, 640.0, Some(ArtifactId("artifact-html".to_string())));
    let state = crate::viewer::ViewerSearchEngine::state("needle", vec![target.clone()], None);

    let command = crate::viewer::ViewerCommandFactory::navigate_search(
        &state,
        crate::viewer::ViewerSearchDirection::Next,
    );

    let Some(crate::viewer::ViewerCommand::Search(command)) = command else {
        return Err("expected search command".to_string());
    };
    assert_eq!(
        crate::viewer::ViewerSearchDirection::Next,
        command.direction
    );
    assert_eq!(target, command.target);
    assert_eq!(
        ArtifactId("artifact-html".to_string()),
        command.scroll.target.artifact_id
    );
    assert_eq!(target.rect, command.scroll.target.rect);
    Ok(())
}

#[test]
fn slideshow_controls_become_slideshow_commands() -> Result<(), String> {
    assert_eq!(
        crate::viewer::ViewerCommand::Slideshow(crate::viewer::SlideshowCommand::NextPage),
        crate::viewer::ViewerCommandFactory::next_slideshow_page()
    );
    assert_eq!(
        crate::viewer::ViewerCommand::Slideshow(crate::viewer::SlideshowCommand::PreviousPage),
        crate::viewer::ViewerCommandFactory::previous_slideshow_page()
    );
    assert_eq!(
        crate::viewer::ViewerCommand::Slideshow(crate::viewer::SlideshowCommand::Close),
        crate::viewer::ViewerCommandFactory::close_slideshow()
    );

    let command = crate::viewer::ViewerCommandFactory::update_slideshow_settings(true, false);
    let crate::viewer::ViewerCommand::Slideshow(crate::viewer::SlideshowCommand::UpdateSettings(
        update,
    )) = command
    else {
        return Err("expected slideshow settings command".to_string());
    };
    assert!(update.hover_highlight_enabled);
    assert!(!update.diagram_controls_enabled);
    Ok(())
}

fn search_target(
    index: usize,
    y: f32,
    artifact_id: Option<ArtifactId>,
) -> crate::viewer::ViewerSearchTarget {
    crate::viewer::ViewerSearchTarget {
        index,
        matched: crate::viewer::ViewerSearchMatch {
            id: crate::viewer::ViewerSearchMatchId(format!("hit-{index}")),
            node_id: KmmNodeId(format!("node-{index}")),
            source: source("needle"),
            range: crate::viewer::ViewerTextRange { start: 0, end: 6 },
            text: "needle".to_string(),
            artifact_id,
        },
        rect: rect(12.0, y),
    }
}

fn rect(x: f32, y: f32) -> crate::viewer::ViewerRect {
    crate::viewer::ViewerRect {
        x,
        y,
        width: 80.0,
        height: 24.0,
    }
}

fn source(raw: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}
