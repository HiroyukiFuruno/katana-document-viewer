use std::path::{Path, PathBuf};

const KATANA_FIXTURES: &[&str] = &[
    "sample.md",
    "sample.ja.md",
    "sample_basic.md",
    "sample_basic.ja.md",
    "sample_diagrams.md",
    "sample_diagrams.ja.md",
    "sample_mermaid.md",
    "sample_mermaid_ja.md",
    "sample_html.md",
    "sample_html.ja.md",
    "drawio/README.md",
    "drawio/basic/01-empty-mxfile.drawio",
    "drawio/basic/02-standalone-mxgraphmodel.drawio",
    "drawio/basic/03-basic-flow.drawio",
    "drawio/basic/04-shape-style-matrix.drawio",
    "drawio/basic/05-edge-variants.drawio",
    "drawio/basic/06-multi-page.drawio",
    "drawio/basic/07-html-labels-and-entities.drawio",
    "drawio/basic/08-group-container.drawio",
    "drawio/basic/09-layers-and-swimlane.drawio",
    "drawio/basic/10-userobject-metadata.drawio",
    "drawio/basic/11-japanese-labels.drawio",
    "drawio/basic/12-vars-placeholders.drawio",
];

#[test]
fn katana_fixture_inventory_is_fixed_in_kdv_assets() -> Result<(), Box<dyn std::error::Error>> {
    let root = workspace_root()?;
    for fixture in KATANA_FIXTURES {
        let path = root.join("assets/fixtures/katana").join(fixture);
        assert!(
            path.is_file(),
            "missing KatanA fixture copied into KDV assets: {}",
            path.display()
        );
    }
    Ok(())
}

fn workspace_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    match manifest_dir.parent().and_then(Path::parent) {
        Some(root) => Ok(root.to_path_buf()),
        None => Err(format!(
            "crate manifest must be below workspace crates directory: {}",
            manifest_dir.display()
        )
        .into()),
    }
}
