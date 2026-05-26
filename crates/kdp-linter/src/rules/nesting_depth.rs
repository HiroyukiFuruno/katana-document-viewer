use crate::diagnostics::{KdpLintError, Violation};
use crate::span::SpanOps;
use crate::workspace::WorkspaceModel;
use std::path::PathBuf;
use syn::visit::Visit;

const MAX_NESTING_DEPTH: usize = 3;

pub struct NestingDepthRule;

impl NestingDepthRule {
    pub fn check(workspace: &WorkspaceModel) -> Result<Vec<Violation>, KdpLintError> {
        let mut violations = Vec::new();
        for file in workspace.rust_files() {
            let mut visitor = NestingDepthVisitor::new(file.path().to_path_buf());
            visitor.visit_file(file.syntax());
            violations.extend(visitor.into_violations());
        }
        Ok(violations)
    }
}

struct NestingDepthVisitor {
    file: PathBuf,
    violations: Vec<Violation>,
    depth: usize,
}

impl NestingDepthVisitor {
    fn new(file: PathBuf) -> Self {
        Self {
            file,
            violations: Vec::new(),
            depth: 0,
        }
    }

    fn into_violations(self) -> Vec<Violation> {
        self.violations
    }

    fn enter(&mut self, span: proc_macro2::Span) {
        self.depth += 1;
        if self.depth <= MAX_NESTING_DEPTH {
            return;
        }
        let location = SpanOps::start(span);
        self.violations.push(Violation::new(
            self.file.clone(),
            location.line,
            location.column,
            "nesting-depth",
            format!(
                "nesting depth {} exceeds {}.",
                self.depth, MAX_NESTING_DEPTH
            ),
        ));
    }

    fn exit(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }
}

impl<'ast> Visit<'ast> for NestingDepthVisitor {
    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        self.enter(node.if_token.span);
        syn::visit::visit_expr_if(self, node);
        self.exit();
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.enter(node.for_token.span);
        syn::visit::visit_expr_for_loop(self, node);
        self.exit();
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.enter(node.while_token.span);
        syn::visit::visit_expr_while(self, node);
        self.exit();
    }

    fn visit_expr_loop(&mut self, node: &'ast syn::ExprLoop) {
        self.enter(node.loop_token.span);
        syn::visit::visit_expr_loop(self, node);
        self.exit();
    }

    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        self.enter(node.match_token.span);
        syn::visit::visit_expr_match(self, node);
        self.exit();
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::FixtureWorkspace;
    use super::*;

    #[test]
    fn nesting_depth_rule_reports_excessive_nesting() -> Result<(), KdpLintError> {
        let fixture = FixtureWorkspace::new().with_default_manifests()?;
        let source = r#"
fn nested() {
    if true {
        if true {
            if true {
                if true {
                    if true {
                        println!("too deep");
                    }
                }
            }
        }
    }
}
"#;
        fixture.write_rust_file("crates/katana-document-viewer/src/nested.rs", source)?;
        let workspace = fixture.workspace()?;
        let violations = NestingDepthRule::check(&workspace)?;

        assert!(
            violations
                .iter()
                .any(|violation| violation.rule == "nesting-depth")
        );
        Ok(())
    }

    #[test]
    fn nesting_depth_rule_allows_valid_depth() -> Result<(), KdpLintError> {
        let fixture = FixtureWorkspace::new().with_default_manifests()?;
        let source = r#"
fn nested() {
    if true {
        if true {
            if true {
                println!("ok");
            }
        }
    }
}
"#;
        fixture.write_rust_file("crates/katana-document-viewer/src/nested_ok.rs", source)?;
        let workspace = fixture.workspace()?;
        let violations = NestingDepthRule::check(&workspace)?;

        assert!(violations.is_empty());
        Ok(())
    }

    #[test]
    fn nesting_depth_rule_checks_loop_and_match_forms() -> Result<(), KdpLintError> {
        let fixture = FixtureWorkspace::new().with_default_manifests()?;
        let source = r#"
fn nested() {
    for _ in 0..2 {
        while false {
            loop {
                match 0 {
                    0 => {}
                    _ => {}
                }
            }
        }
    }
}
"#;
        fixture.write_rust_file("crates/katana-document-viewer/src/nested_forms.rs", source)?;
        let workspace = fixture.workspace()?;
        let violations = NestingDepthRule::check(&workspace)?;

        assert!(
            violations
                .iter()
                .any(|violation| violation.message.contains("nesting depth"))
        );
        Ok(())
    }
}
