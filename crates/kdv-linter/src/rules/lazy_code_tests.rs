use super::super::test_helpers::FixtureWorkspace;
use super::*;
use syn::punctuated::Punctuated;

#[test]
fn lazy_code_rule_flags_prohibited_macros() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let source = r#"
fn pending() {
    todo!();
    unimplemented!();
    dbg!("value");
    other!(1, 2, 3);
}
"#;
    fixture.write_rust_file("crates/katana-document-viewer/src/lazy_code.rs", source)?;
    let workspace = fixture.workspace()?;
    let violations = LazyCodeRule::check(&workspace)?;

    assert_eq!(
        violations
            .iter()
            .filter(|violation| violation.rule == "lazy-code")
            .count(),
        3
    );
    Ok(())
}

#[test]
fn lazy_code_check_macro_ignores_empty_path() {
    let mut visitor = LazyCodeVisitor::new("kdv-linter.rs".into());
    let mut macro_invocation: syn::Macro = syn::parse_quote!(todo!());
    macro_invocation.path.segments = Punctuated::new();
    visitor.check_macro(&macro_invocation);

    assert!(visitor.into_violations().is_empty());
}

#[test]
fn lazy_code_rule_ignores_other_macros() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let source = r#"
fn ready() {
    assert!(true);
    println!("ok");
}
"#;
    fixture.write_rust_file("crates/katana-document-viewer/src/lazy_code.rs", source)?;
    let workspace = fixture.workspace()?;
    let violations = LazyCodeRule::check(&workspace)?;

    assert!(
        !violations
            .iter()
            .any(|violation| violation.rule == "lazy-code")
    );
    Ok(())
}
