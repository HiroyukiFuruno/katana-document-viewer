use crate::diagnostics::{KdpLintError, Violation};
use crate::span::SpanOps;
use crate::workspace::WorkspaceModel;
use std::path::PathBuf;
use syn::visit::Visit;

const MAX_FUNCTION_LINES: usize = 30;

pub struct FunctionLengthRule;

impl FunctionLengthRule {
    pub fn check(workspace: &WorkspaceModel) -> Result<Vec<Violation>, KdpLintError> {
        let mut violations = Vec::new();
        for file in workspace.rust_files() {
            let mut visitor = FunctionLengthVisitor::new(file.path().to_path_buf());
            visitor.visit_file(file.syntax());
            violations.extend(visitor.into_violations());
        }
        Ok(violations)
    }
}

struct FunctionLengthVisitor {
    file: PathBuf,
    violations: Vec<Violation>,
}

impl FunctionLengthVisitor {
    fn new(file: PathBuf) -> Self {
        Self {
            file,
            violations: Vec::new(),
        }
    }

    fn into_violations(self) -> Vec<Violation> {
        self.violations
    }

    fn check_block(&mut self, name: &syn::Ident, block: &syn::Block) {
        let start_line = name.span().start().line;
        let end_line = SpanOps::block_end_line(block);
        let lines = end_line.saturating_sub(start_line) + 1;
        if lines <= MAX_FUNCTION_LINES {
            return;
        }
        let location = SpanOps::start(name.span());
        self.violations.push(Violation::new(
            self.file.clone(),
            location.line,
            location.column,
            "function-length",
            format!("function `{name}` has {lines} lines. Extract focused helper methods."),
        ));
    }
}

impl<'ast> Visit<'ast> for FunctionLengthVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.check_block(&node.sig.ident, &node.block);
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.check_block(&node.sig.ident, &node.block);
        syn::visit::visit_impl_item_fn(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::FixtureWorkspace;
    use super::*;

    #[test]
    fn function_length_rule_ignores_short_functions() -> Result<(), KdpLintError> {
        let fixture = FixtureWorkspace::new().with_default_manifests()?;
        let source = "fn short() {\n    let _ = 1;\n}\n";
        fixture.write_rust_file("crates/katana-document-viewer/src/short.rs", source)?;

        let workspace = fixture.workspace()?;
        let violations = FunctionLengthRule::check(&workspace)?;

        assert!(violations.is_empty());
        Ok(())
    }

    #[test]
    fn function_length_rule_flags_long_fn_item() -> Result<(), KdpLintError> {
        let fixture = FixtureWorkspace::new().with_default_manifests()?;
        let body = "    let _ = 1;\n".repeat(33);
        let source = format!("fn long_fn() {{\n{body}}}\n");
        fixture.write_rust_file("crates/katana-document-viewer/src/long.rs", &source)?;

        let workspace = fixture.workspace()?;
        let violations = FunctionLengthRule::check(&workspace)?;
        let found = violations.iter().any(|violation| {
            violation.rule == "function-length" && violation.message.contains("long_fn")
        });

        assert!(found);
        Ok(())
    }

    #[test]
    fn function_length_rule_checks_impl_item_fn() -> Result<(), KdpLintError> {
        let fixture = FixtureWorkspace::new().with_default_manifests()?;
        let body = "        let _ = 1;\n".repeat(33);
        let source =
            format!("struct Holder {{}}\nimpl Holder {{\n    fn long_fn() {{\n{body}    }}\n}}\n");
        fixture.write_rust_file("crates/katana-document-viewer/src/impl_long.rs", &source)?;

        let workspace = fixture.workspace()?;
        let violations = FunctionLengthRule::check(&workspace)?;
        let found = violations.iter().any(|violation| {
            violation.rule == "function-length" && violation.message.contains("long_fn")
        });

        assert!(found);
        Ok(())
    }
}
