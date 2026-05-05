use crate::diagnostics::Violation;
use crate::span::SpanOps;
use crate::workspace::WorkspaceModel;
use std::path::PathBuf;
use syn::visit::Visit;

use super::architecture::EGUI_CRATE;

const LIB_API_NAMES: &[&str] = &[
    "MarkdownPreview",
    "MarkdownSource",
    "PreviewConfig",
    "PreviewDiagnostics",
    "PreviewError",
    "PreviewOutput",
    "PreviewTheme",
    "RenderTarget",
];

pub(super) struct EguiDuplicationRule;

impl EguiDuplicationRule {
    pub(super) fn check(workspace: &WorkspaceModel) -> Vec<Violation> {
        let egui_src = workspace.root().join(EGUI_CRATE).join("src");
        let mut violations = Vec::new();
        for file in workspace.rust_files() {
            if !file.is_under(&egui_src) {
                continue;
            }
            let mut visitor = EguiDuplicationVisitor::new(file.path().to_path_buf());
            visitor.visit_file(file.syntax());
            violations.extend(visitor.into_violations());
        }
        violations
    }
}

struct EguiDuplicationVisitor {
    file: PathBuf,
    violations: Vec<Violation>,
}

impl EguiDuplicationVisitor {
    fn new(file: PathBuf) -> Self {
        Self {
            file,
            violations: Vec::new(),
        }
    }

    fn into_violations(self) -> Vec<Violation> {
        self.violations
    }

    fn check_ident(&mut self, ident: &syn::Ident) {
        if !LIB_API_NAMES.iter().any(|it| ident == it) {
            return;
        }
        let location = SpanOps::start(ident.span());
        self.violations.push(Violation::new(
            self.file.clone(),
            location.line,
            location.column,
            "egui-preview-duplication",
            format!("egui must reuse library API `{ident}` instead of redeclaring it."),
        ));
    }

    fn check_module_name(&mut self, ident: &syn::Ident) {
        if !matches!(
            ident.to_string().as_str(),
            "preview" | "renderer" | "runtime"
        ) {
            return;
        }
        let location = SpanOps::start(ident.span());
        self.violations.push(Violation::new(
            self.file.clone(),
            location.line,
            location.column,
            "egui-preview-duplication",
            format!("egui module `{ident}` would own preview logic. Put it in the neutral crate."),
        ));
    }
}

impl<'ast> Visit<'ast> for EguiDuplicationVisitor {
    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.check_ident(&node.ident);
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        self.check_ident(&node.ident);
        syn::visit::visit_item_enum(self, node);
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        self.check_ident(&node.ident);
        syn::visit::visit_item_trait(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.check_module_name(&node.ident);
        syn::visit::visit_item_mod(self, node);
    }
}
