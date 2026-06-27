use super::types::{ViewerDiagramKind, ViewerHtmlRole, ViewerNodeKind};
use crate::export_html_ops::ExportHtmlOps as HtmlOps;
use crate::export_surface_text::SurfaceTextParser as TextParser;
use html_role::ViewerHtmlRoleClassifier;
use katana_markdown_model::{CodeBlockRole, DiagramKind, HtmlBlockRole, KmmNode, KmmNodeKind};

pub(super) struct ViewerNodeClassifier;

impl ViewerNodeClassifier {
    pub(super) fn node_kind_for_node(node: &KmmNode) -> Option<ViewerNodeKind> {
        Self::special_node_kind(node).or_else(|| Self::node_kind(&node.kind))
    }

    fn special_node_kind(node: &KmmNode) -> Option<ViewerNodeKind> {
        if Self::standalone_image(node).is_some() {
            return Some(ViewerNodeKind::Image);
        }
        if Self::is_details_html(node) {
            return Some(ViewerNodeKind::Html {
                role: ViewerHtmlRole::Accordion,
            });
        }
        if Self::is_html_table(node) {
            return Some(ViewerNodeKind::Table);
        }
        if let Some(html_kind) = Self::source_html_node_kind(node) {
            return Some(html_kind);
        }
        Self::alert_node_kind(node)
    }

    fn source_html_node_kind(node: &KmmNode) -> Option<ViewerNodeKind> {
        if matches!(node.kind, KmmNodeKind::Paragraph)
            && let Some(role) =
                ViewerHtmlRoleClassifier::from_paragraph_source(&node.source.raw.text)
        {
            return Some(ViewerNodeKind::Html { role });
        }
        if matches!(node.kind, KmmNodeKind::RawBlock { .. })
            && let Some(role) =
                ViewerHtmlRoleClassifier::from_paragraph_source(&node.source.raw.text)
        {
            return Some(ViewerNodeKind::Html { role });
        }
        if let KmmNodeKind::HtmlBlock(role) = &node.kind {
            return Some(ViewerNodeKind::Html {
                role: ViewerHtmlRoleClassifier::from_source(role, &node.source.raw.text),
            });
        }
        None
    }

    fn alert_node_kind(node: &KmmNode) -> Option<ViewerNodeKind> {
        if let KmmNodeKind::Alert { label } = &node.kind {
            if Self::is_gfm_alert(&node.source.raw.text) {
                return Some(ViewerNodeKind::Alert {
                    label: label.clone(),
                });
            }
            return Some(ViewerNodeKind::BlockQuote);
        }
        None
    }

    pub(super) fn node_kind(kind: &KmmNodeKind) -> Option<ViewerNodeKind> {
        match kind {
            KmmNodeKind::Heading(heading) => Some(ViewerNodeKind::Heading {
                level: heading.level,
            }),
            KmmNodeKind::Paragraph => Some(ViewerNodeKind::Paragraph),
            KmmNodeKind::CodeBlock(role) => Some(Self::code_block_kind(role)),
            KmmNodeKind::DollarMathBlock(_) => Some(ViewerNodeKind::Math),
            KmmNodeKind::HtmlBlock(role) => Some(Self::html_block_kind(role)),
            KmmNodeKind::Table(table) if Self::has_table_separator_row(table) => {
                Some(ViewerNodeKind::Table)
            }
            KmmNodeKind::Table(_) => Some(ViewerNodeKind::Paragraph),
            KmmNodeKind::List(_) => Some(ViewerNodeKind::List),
            KmmNodeKind::DescriptionList { .. } => Some(ViewerNodeKind::List),
            KmmNodeKind::BlockQuote => Some(ViewerNodeKind::BlockQuote),
            KmmNodeKind::Alert { label } => Some(ViewerNodeKind::Alert {
                label: label.clone(),
            }),
            KmmNodeKind::Image(_) => Some(ViewerNodeKind::Image),
            KmmNodeKind::ThematicBreak => Some(ViewerNodeKind::Rule),
            KmmNodeKind::RawBlock { .. } => Some(ViewerNodeKind::Raw),
            KmmNodeKind::FootnoteDefinition(definition) => {
                Some(ViewerNodeKind::FootnoteDefinition {
                    label: definition.label.clone(),
                })
            }
            _ => None,
        }
    }

    fn code_block_kind(role: &CodeBlockRole) -> ViewerNodeKind {
        match role {
            CodeBlockRole::Plain { language } => ViewerNodeKind::Code {
                language: language.clone(),
            },
            CodeBlockRole::Diagram { kind } => ViewerNodeKind::Diagram {
                kind: Self::diagram_kind(kind),
            },
            CodeBlockRole::Math => ViewerNodeKind::Math,
        }
    }

    fn html_block_kind(role: &HtmlBlockRole) -> ViewerNodeKind {
        ViewerNodeKind::Html {
            role: ViewerHtmlRoleClassifier::from_role(role),
        }
    }

    pub(super) fn node_text(node: &KmmNode, kind: &ViewerNodeKind) -> String {
        let raw = &node.source.raw.text;
        match (&node.kind, kind) {
            (KmmNodeKind::Heading(heading), _) => heading.text.clone(),
            (KmmNodeKind::CodeBlock(_), _) => HtmlOps::fenced_body(raw),
            (KmmNodeKind::HtmlBlock(_), _) => TextParser::html_fragment_text(raw),
            (KmmNodeKind::RawBlock { .. }, ViewerNodeKind::Html { .. }) => {
                TextParser::html_fragment_text(raw)
            }
            (KmmNodeKind::DollarMathBlock(math), _) => math.expression.clone(),
            (KmmNodeKind::Table(table), _) => Self::table_text(table),
            (KmmNodeKind::List(list), _) => Self::list_text(list),
            (KmmNodeKind::BlockQuote, _) => Self::block_quote_text(node),
            (KmmNodeKind::Alert { .. }, ViewerNodeKind::BlockQuote) => Self::block_quote_text(node),
            (KmmNodeKind::Alert { label }, _) => Self::alert_text(label, raw),
            (KmmNodeKind::DescriptionList { items }, _) => Self::description_list_text(items),
            (KmmNodeKind::Image(image), _) => image.alt.clone(),
            (KmmNodeKind::RawBlock { .. }, ViewerNodeKind::Raw) => raw.clone(),
            (KmmNodeKind::ThematicBreak, _) => String::new(),
            (KmmNodeKind::Paragraph, _) => Self::paragraph_text(node),
            (
                KmmNodeKind::FootnoteDefinition(definition),
                ViewerNodeKind::FootnoteDefinition { .. },
            ) => Self::footnote_definition_text(&definition.label, &definition.text),
            _ => Self::inline_text(node),
        }
    }

    fn diagram_kind(kind: &DiagramKind) -> ViewerDiagramKind {
        match kind {
            DiagramKind::Mermaid => ViewerDiagramKind::Mermaid,
            DiagramKind::DrawIo => ViewerDiagramKind::DrawIo,
            DiagramKind::PlantUml => ViewerDiagramKind::PlantUml,
        }
    }

    fn is_gfm_alert(raw: &str) -> bool {
        raw.lines().any(|line| {
            let quoted = line.trim_start().trim_start_matches('>').trim_start();
            quoted.starts_with("[!")
        })
    }
}

#[path = "classifier_assets.rs"]
mod assets;
#[path = "classifier_html_role.rs"]
mod html_role;
#[path = "classifier_html_table.rs"]
mod html_table;
#[path = "classifier_inline_text.rs"]
mod inline_text;
#[path = "classifier_spans.rs"]
mod spans;
#[path = "classifier_text_helpers.rs"]
mod text_helpers;

#[cfg(test)]
#[path = "classifier_tests.rs"]
mod tests;
