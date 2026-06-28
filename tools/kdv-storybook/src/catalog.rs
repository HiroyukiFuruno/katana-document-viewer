use std::path::{Path, PathBuf};

const CURATED_FIXTURES: &[&str] = &[
    "katana/sample.md",
    "katana/sample_basic.md",
    "katana/sample_html.md",
    "katana/sample_diagrams.md",
    "katana/sample_mermaid.md",
    "katana/sample.ja.md",
    "katana/drawio/basic/03-basic-flow.drawio",
    "katana/drawio/basic/05-edge-variants.drawio",
    "katana/drawio/basic/06-multi-page.drawio",
    "katana/drawio/basic/07-html-labels-and-entities.drawio",
    "katana/drawio/basic/11-japanese-labels.drawio",
    "direct/html-alignment.html",
    "direct/html-margin-left.html",
    "direct/sample.drawio",
    "direct/sample.mmd",
    "direct/sample.puml",
    "direct/kdv-icon.png",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorybookFixture {
    pub label: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureCatalog {
    pub fixtures: Vec<StorybookFixture>,
}

impl FixtureCatalog {
    pub fn load(root: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut fixtures = Vec::new();
        collect_curated_fixture_paths(root, &mut fixtures)?;
        if fixtures.is_empty() {
            return Err(format!("storybook fixture is empty: {}", root.display()).into());
        }
        Ok(Self { fixtures })
    }
}

fn collect_curated_fixture_paths(
    root: &Path,
    fixtures: &mut Vec<StorybookFixture>,
) -> Result<(), Box<dyn std::error::Error>> {
    for label in CURATED_FIXTURES {
        let path = root.join(label);
        if !path.exists() {
            return Err(format!("missing storybook curated fixture: {}", path.display()).into());
        }
        if !is_supported_fixture(&path) {
            return Err(
                format!("unsupported storybook curated fixture: {}", path.display()).into(),
            );
        }
        fixtures.push(StorybookFixture {
            label: (*label).to_string(),
            path,
        });
    }
    Ok(())
}

fn is_supported_fixture(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(std::ffi::OsStr::to_str) else {
        return false;
    };
    matches!(
        extension.to_ascii_lowercase().as_str(),
        "bmp"
            | "drawio"
            | "drowio"
            | "gif"
            | "htm"
            | "html"
            | "jpeg"
            | "jpg"
            | "markdown"
            | "md"
            | "mermaid"
            | "mmd"
            | "plantuml"
            | "png"
            | "puml"
            | "svg"
            | "txt"
            | "webp"
    )
}

#[cfg(test)]
mod tests {
    use super::{FixtureCatalog, is_supported_fixture};
    use std::path::Path;

    #[test]
    fn markdown_and_direct_visual_sources_are_supported() {
        assert!(is_supported_fixture(Path::new("sample.markdown")));
        assert!(is_supported_fixture(Path::new("sample.md")));
        assert!(is_supported_fixture(Path::new("sample.drawio")));
        assert!(is_supported_fixture(Path::new("sample.png")));
    }

    #[test]
    fn storybook_catalog_uses_curated_representative_fixture_files()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/fixtures");
        let fixtures = FixtureCatalog::load(&root)?.fixtures;
        let labels = fixtures
            .iter()
            .map(|fixture| fixture.label.clone())
            .collect::<Vec<_>>();

        assert_eq!(curated_labels(), labels);
        assert!(!labels.contains(&"katana/drawio/basic/01-empty-mxfile.drawio".to_string()));
        assert!(
            !labels.contains(&"katana/drawio/basic/02-standalone-mxgraphmodel.drawio".to_string())
        );
        assert!(!labels.contains(&"direct/kdv-icon.bmp".to_string()));
        assert!(!labels.contains(&"direct/sample.drowio".to_string()));
        assert!(!labels.contains(&"direct/sample.mermaid".to_string()));
        assert!(!labels.contains(&"direct/sample.plantuml".to_string()));
        Ok(())
    }

    #[test]
    fn storybook_catalog_keeps_direct_fixtures_to_source_kind_representatives()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/fixtures");
        let labels = FixtureCatalog::load(&root)?
            .fixtures
            .into_iter()
            .map(|fixture| fixture.label)
            .filter(|label| label.starts_with("direct/"))
            .collect::<Vec<_>>();

        assert_eq!(
            vec![
                "direct/html-alignment.html",
                "direct/html-margin-left.html",
                "direct/sample.drawio",
                "direct/sample.mmd",
                "direct/sample.puml",
                "direct/kdv-icon.png",
            ],
            labels
        );
        Ok(())
    }

    fn curated_labels() -> Vec<String> {
        [
            "katana/sample.md",
            "katana/sample_basic.md",
            "katana/sample_html.md",
            "katana/sample_diagrams.md",
            "katana/sample_mermaid.md",
            "katana/sample.ja.md",
            "katana/drawio/basic/03-basic-flow.drawio",
            "katana/drawio/basic/05-edge-variants.drawio",
            "katana/drawio/basic/06-multi-page.drawio",
            "katana/drawio/basic/07-html-labels-and-entities.drawio",
            "katana/drawio/basic/11-japanese-labels.drawio",
            "direct/html-alignment.html",
            "direct/html-margin-left.html",
            "direct/sample.drawio",
            "direct/sample.mmd",
            "direct/sample.puml",
            "direct/kdv-icon.png",
        ]
        .into_iter()
        .map(str::to_string)
        .collect()
    }
}
