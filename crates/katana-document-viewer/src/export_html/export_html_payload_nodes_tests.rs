use super::*;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{
    ByteRange, DescriptionItem, FootnoteDefinitionNode, KatanaMarkdownModel, KmmNode, KmmNodeId,
    KmmNodeKind, LineColumn, LineColumnRange, ListItemNode, ListNode, MarkdownInput, RawSnippet,
    SourceSpan, TableCell, TableNode, TableRow, TextSpan,
};

const ORDERED_START: usize = 3;
const SOURCE_START_OFFSET: usize = 0;
const FIRST_LINE: usize = 1;
const FIRST_COLUMN: usize = 1;

#[test]
fn append_routes_inline_kind_to_inline_writer() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("text".to_string()),
        kind: KmmNodeKind::Text(TextSpan {
            text: "alpha &lt; beta".to_string(),
        }),
        source: source_span("alpha &lt; beta"),
        children: Vec::new(),
    };

    RemainingHtmlNodeWriter::append(&mut html, &graph, &theme, &node);

    assert_eq!(html, "alpha &lt; beta");
}

#[test]
fn append_routes_raw_block_to_preformatted_output() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("raw".to_string()),
        kind: KmmNodeKind::RawBlock {
            reason: "custom".to_string(),
        },
        source: source_span("raw text"),
        children: Vec::new(),
    };

    RemainingHtmlNodeWriter::append(&mut html, &graph, &theme, &node);

    assert_eq!(html, "<pre data-kdv-raw-reason=\"custom\">raw text</pre>\n");
}

#[test]
fn append_routes_structured_nodes() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();

    RemainingHtmlNodeWriter::append(&mut html, &graph, &theme, &structured_list_node());
    RemainingHtmlNodeWriter::append(&mut html, &graph, &theme, &structured_table_node());
    RemainingHtmlNodeWriter::append(&mut html, &graph, &theme, &description_list_node());

    assert!(html.contains("<ol start=\"3\">"));
    assert!(html.contains("<table data-kdv-table=\"katana\">"));
    assert!(html.contains("<dl>"));
}

fn structured_list_node() -> KmmNode {
    let list_item = ListItemNode {
        marker: "-".to_string(),
        ordered_number: Some(ORDERED_START),
        task_marker: None,
        body: Vec::new(),
        children: Vec::new(),
        source: source_span("item"),
    };
    KmmNode {
        id: KmmNodeId("list".to_string()),
        kind: KmmNodeKind::List(ListNode {
            ordered: true,
            task_markers: Vec::new(),
            items: vec![list_item],
        }),
        source: source_span("1. item"),
        children: Vec::new(),
    }
}

fn structured_table_node() -> KmmNode {
    KmmNode {
        id: KmmNodeId("table".to_string()),
        kind: KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: table_rows(),
        }),
        source: source_span("|h|\\n|--|\\n|body|"),
        children: Vec::new(),
    }
}

fn table_rows() -> Vec<TableRow> {
    ["h", "body", "tail"]
        .into_iter()
        .map(|text| TableRow {
            cells: vec![TableCell {
                text: text.to_string(),
                source: source_span(text),
            }],
        })
        .collect()
}

fn description_list_node() -> KmmNode {
    KmmNode {
        id: KmmNodeId("desc".to_string()),
        kind: KmmNodeKind::DescriptionList {
            items: vec![DescriptionItem {
                term: "term".to_string(),
                description: "desc".to_string(),
            }],
        },
        source: source_span("term:desc"),
        children: Vec::new(),
    }
}

#[test]
fn append_panics_for_unsupported_node_kind() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("unsupported".to_string()),
        kind: KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
            label: "x".to_string(),
            text: "bad".to_string(),
        }),
        source: source_span("x"),
        children: Vec::new(),
    };

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        RemainingHtmlNodeWriter::append(&mut html, &graph, &theme, &node);
    }));

    assert!(result.is_err());
}

fn graph() -> BuildGraph {
    let source = crate::DocumentSource {
        uri: crate::SourceUri("file:///test.md".to_string()),
        kind: crate::SourceKind::Markdown,
        revision: crate::SourceRevision("r".to_string()),
        content: "x".to_string(),
    };
    let snapshot = crate::DocumentSnapshotFactory::from_kmm(
        source.clone(),
        match KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "test.md",
            source.content.clone(),
        )) {
            Ok(model) => model,
            Err(error) => {
                std::panic::resume_unwind(Box::new(format!("parse test markdown: {error}")))
            }
        },
    );
    BuildGraph::from_request(&crate::BuildRequest {
        snapshot,
        profile: crate::BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    })
}

fn source_span(text: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: SOURCE_START_OFFSET,
            end: text.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn {
                line: FIRST_LINE,
                column: FIRST_COLUMN,
            },
            end: LineColumn {
                line: FIRST_LINE,
                column: text.len() + FIRST_COLUMN,
            },
        },
        raw: RawSnippet {
            text: text.to_string(),
        },
    }
}
