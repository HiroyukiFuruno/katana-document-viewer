use super::*;

#[test]
fn load_collects_kuc_core_files_separately() -> Result<(), KdvLintError> {
    let root = workspace_fixture("kuc-core");
    let kuc_source = root.join("crates/katana-ui-core/src");
    std::fs::create_dir_all(&kuc_source).map_err(|source| KdvLintError::Read {
        path: kuc_source.clone(),
        source,
    })?;
    write_text(&kuc_source.join("lib.rs"), "pub struct KucOnly;")?;

    let workspace = WorkspaceModel::load(&root)?;

    assert!(workspace.rust_files().is_empty());
    assert_eq!(workspace.kuc_core_files().len(), 1);
    std::fs::remove_dir_all(&root).map_err(|source| KdvLintError::Read {
        path: root.clone(),
        source,
    })?;
    Ok(())
}
