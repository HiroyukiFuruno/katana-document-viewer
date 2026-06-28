use crate::KucDiagramControlResolver;
use crate::preview::PreviewScene;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiTextSpan};
use katana_ui_core_storybook::{UiTreeRenderArea, UiTreeSurfaceHost};

const NO_COLOR: [u8; 4] = [0, 0, 0, 0];

pub(crate) struct PreviewRequirementAssertions<'a> {
    scene: &'a PreviewScene,
}

impl<'a> PreviewRequirementAssertions<'a> {
    pub(crate) fn new(scene: &'a PreviewScene) -> Self {
        Self { scene }
    }

    pub(crate) fn assert_role(&self, role: &str) {
        assert!(self.count_role(self.scene.tree.root(), role) > 0, "{role}");
    }

    pub(crate) fn assert_style(&self, style: &str) {
        assert!(
            self.count_style(self.scene.tree.root(), style) > 0,
            "{style}"
        );
    }

    pub(crate) fn assert_indented_list_depths(&self) {
        assert!(
            self.has_list_margin(self.scene.tree.root(), 40),
            "list depth 1 must reach KUC as common margin"
        );
        assert!(
            self.has_list_margin(self.scene.tree.root(), 80),
            "list depth 2 must reach KUC as common margin"
        );
    }

    pub(crate) fn assert_kind(&self, kind: UiNodeKind) {
        assert!(
            self.count_kind(self.scene.tree.root(), kind) > 0,
            "{kind:?}"
        );
    }

    pub(crate) fn assert_action(&self, action: &str) {
        assert!(
            self.count_action(self.scene.tree.root(), action) > 0,
            "{action}"
        );
    }

    pub(crate) fn assert_internal_diagram_action(&self, action: &str) {
        assert!(self.count_internal_diagram_action(action) > 0, "{action}");
    }

    pub(crate) fn assert_link_span(&self) {
        assert!(self.has_link_span(self.scene.tree.root()));
    }

    pub(crate) fn assert_link_target(&self, target: &str) {
        assert!(
            self.has_link_target(self.scene.tree.root(), target),
            "{target}"
        );
    }

    pub(crate) fn assert_accessibility_label(&self, label: &str) {
        assert!(
            self.has_accessibility_label(self.scene.tree.root(), label),
            "{label}"
        );
    }

    pub(crate) fn assert_syntax_highlighted_code(&self) {
        assert!(
            self.has_syntax_highlighted_code(self.scene.tree.root()),
            "syntax-highlighted code spans must reach KUC"
        );
    }

    pub(crate) fn assert_strikethrough_span(&self, text: &str) {
        assert!(
            self.has_strikethrough_span(self.scene.tree.root(), text),
            "{text}"
        );
    }

    fn count_role(&self, node: &UiNode, role: &str) -> usize {
        usize::from(node.props().text.role == role)
            + node
                .children()
                .iter()
                .map(|child| self.count_role(child, role))
                .sum::<usize>()
    }

    fn count_style(&self, node: &UiNode, style: &str) -> usize {
        usize::from(
            node.props()
                .style_classes
                .iter()
                .any(|value| value == style),
        ) + node
            .children()
            .iter()
            .map(|child| self.count_style(child, style))
            .sum::<usize>()
    }

    fn has_list_margin(&self, node: &UiNode, expected: u16) -> bool {
        matches!(
            node.props().common.margin.left,
            UiDimension::Px(value) if value == expected
        ) || node
            .children()
            .iter()
            .any(|child| self.has_list_margin(child, expected))
    }

    fn count_kind(&self, node: &UiNode, expected: UiNodeKind) -> usize {
        usize::from(node.kind() == expected)
            + node
                .children()
                .iter()
                .map(|child| self.count_kind(child, expected))
                .sum::<usize>()
    }

    fn count_action(&self, node: &UiNode, action: &str) -> usize {
        usize::from(node.props().interaction.value == action)
            + node
                .children()
                .iter()
                .map(|child| self.count_action(child, action))
                .sum::<usize>()
    }

    fn count_internal_diagram_action(&self, action: &str) -> usize {
        UiTreeSurfaceHost::new(self.scene.theme.clone())
            .document_node_hits(
                self.scene.tree.root(),
                UiTreeRenderArea {
                    x: 0,
                    y: 0,
                    width: self.surface_width(),
                    height: self.scene.content_height.ceil().max(1.0) as usize,
                    scroll_y: 0.0,
                },
            )
            .iter()
            .filter_map(|hit| {
                KucDiagramControlResolver::internal_action_for_node(
                    self.scene.tree.root(),
                    &hit.node_id,
                )
            })
            .filter(|resolved| resolved.command == action)
            .count()
    }

    fn surface_width(&self) -> usize {
        self.scene
            .surface
            .as_ref()
            .map_or(800, |surface| surface.width.max(1) as usize)
    }

    fn has_link_span(&self, node: &UiNode) -> bool {
        node.props()
            .text
            .spans
            .iter()
            .any(|span| !span.link_target.is_empty())
            || node
                .children()
                .iter()
                .any(|child| self.has_link_span(child))
    }

    fn has_link_target(&self, node: &UiNode, target: &str) -> bool {
        node.props()
            .text
            .spans
            .iter()
            .any(|span| span.link_target == target)
            || node
                .children()
                .iter()
                .any(|child| self.has_link_target(child, target))
    }

    fn has_accessibility_label(&self, node: &UiNode, label: &str) -> bool {
        node.props().accessibility_label == label
            || node
                .children()
                .iter()
                .any(|child| self.has_accessibility_label(child, label))
    }

    fn has_syntax_highlighted_code(&self, node: &UiNode) -> bool {
        let is_code = node.props().text.role == "code";
        let has_colored_span = node.props().text.spans.iter().any(Self::is_syntax_span);
        is_code && has_colored_span
            || node
                .children()
                .iter()
                .any(|child| self.has_syntax_highlighted_code(child))
    }

    fn has_strikethrough_span(&self, node: &UiNode, text: &str) -> bool {
        node.props()
            .text
            .spans
            .iter()
            .any(|span| span.text == text && span.style.strikethrough)
            || node
                .children()
                .iter()
                .any(|child| self.has_strikethrough_span(child, text))
    }

    fn is_syntax_span(span: &UiTextSpan) -> bool {
        span.style.monospace && span.style.color_rgba != NO_COLOR
    }
}
