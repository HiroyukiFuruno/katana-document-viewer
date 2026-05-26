use crate::export_html_ops::ExportHtmlOps;
use crate::forge_diagram_render_types::{DiagramRenderEngine, DiagramRenderRequest};
use crate::{
    DocumentSnapshot, DocumentSnapshotFactory, DocumentSource, KdvThemeSnapshot, RenderedDiagram,
    SourceKind, SourceRevision, SourceUri,
};
use katana_markdown_model::KatanaMarkdownModel;
use katana_markdown_model::{
    ByteRange, CodeBlockRole, DiagramKind, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, ListItemNode, ListNode, MarkdownInput, RawSnippet, SourceSpan,
};
use katana_render_runtime::{RenderThemeMode, RenderThemeSnapshot};
use std::sync::{Arc, Mutex};

pub struct DiagramRenderTestSupport;

impl DiagramRenderTestSupport {
    pub fn app_supplied_dark_theme() -> KdvThemeSnapshot {
        let mut theme = KdvThemeSnapshot::katana_dark();
        theme.name = "app-supplied-dark".to_string();
        theme.diagram_background = "transparent".to_string();
        theme.diagram_text = "#abcdef".to_string();
        theme.diagram_fill = "#123456".to_string();
        theme.diagram_stroke = "#654321".to_string();
        theme.diagram_arrow = "#fedcba".to_string();
        theme.mermaid_theme = "dark".to_string();
        theme
    }

    pub fn assert_app_supplied_theme_forwarded(captured: &RenderThemeSnapshot) {
        assert_eq!(captured.mode, RenderThemeMode::Dark);
        assert_eq!(captured.background, "transparent");
        assert_eq!(captured.text, "#abcdef");
        assert_eq!(captured.fill, "#123456");
        assert_eq!(captured.stroke, "#654321");
        assert_eq!(captured.arrow, "#fedcba");
        assert_eq!(captured.mermaid_theme, "dark");
    }

    pub fn snapshot_from_markdown(
        markdown: &str,
    ) -> Result<DocumentSnapshot, Box<dyn std::error::Error>> {
        let source = DocumentSource {
            uri: SourceUri("file:///diagram-theme.md".to_string()),
            kind: SourceKind::Markdown,
            revision: SourceRevision("rev-1".to_string()),
            content: markdown.to_string(),
        };
        let document =
            KatanaMarkdownModel::parse(MarkdownInput::from_content("diagram-theme.md", markdown))?;
        Ok(DocumentSnapshotFactory::from_kmm(source, document))
    }

    pub fn nested_list_root_with_diagram() -> KmmNode {
        let diagram_node = KmmNode {
            id: KmmNodeId("diagram-node".to_string()),
            kind: KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
                kind: DiagramKind::Mermaid,
            }),
            source: simple_source_span("```mermaid\nA --> B\n```"),
            children: Vec::new(),
        };
        KmmNode {
            id: KmmNodeId("list-root".to_string()),
            kind: KmmNodeKind::List(ListNode {
                ordered: false,
                task_markers: Vec::new(),
                items: vec![nested_item(diagram_node)],
            }),
            source: simple_source_span("- root"),
            children: Vec::new(),
        }
    }
}

fn nested_item(diagram_node: KmmNode) -> ListItemNode {
    ListItemNode {
        marker: "-".to_string(),
        ordered_number: None,
        task_marker: None,
        body: Vec::new(),
        children: vec![diagram_node],
        source: simple_source_span("- nested"),
    }
}

fn simple_source_span(text: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: text.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: text.len() + 1,
            },
        },
        raw: RawSnippet {
            text: text.to_string(),
        },
    }
}

pub struct PanicDiagramEngine;

impl DiagramRenderEngine for PanicDiagramEngine {
    fn render(&self, _request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        std::panic::resume_unwind(Box::new("diagram backend panic".to_string()));
    }
}

pub struct ErrorDiagramEngine;

impl DiagramRenderEngine for ErrorDiagramEngine {
    fn render(&self, _request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        Err("render failed".to_string())
    }
}

pub struct RecordingDiagramEngine {
    pub themes: Arc<Mutex<Vec<RenderThemeSnapshot>>>,
}

impl DiagramRenderEngine for RecordingDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        self.themes
            .lock()
            .map_err(|error| error.to_string())?
            .push(request.theme.krr_theme());
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: ExportHtmlOps::diagram_kind_label(&request.kind).to_string(),
            svg: "<svg data-test=\"theme-forwarded\"></svg>".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_markdown_model::DiagramKind;

    #[test]
    fn recording_engine_reports_poisoned_theme_lock() {
        let themes = Arc::new(Mutex::new(Vec::new()));
        poison_theme_lock(&themes);
        let engine = RecordingDiagramEngine { themes };

        let result = engine.render(DiagramRenderRequest {
            node_id: "node-1",
            document_id: "doc-1",
            kind: DiagramKind::Mermaid,
            source: "graph TD; A-->B".to_string(),
            theme: &KdvThemeSnapshot::katana_light(),
        });

        assert!(matches!(result, Err(message) if message.contains("poisoned")));
    }

    fn poison_theme_lock(themes: &Arc<Mutex<Vec<RenderThemeSnapshot>>>) {
        let _ = std::panic::catch_unwind(|| {
            let _guard = themes.lock();
            std::panic::resume_unwind(Box::new("poison theme lock".to_string()));
        });
    }
}
