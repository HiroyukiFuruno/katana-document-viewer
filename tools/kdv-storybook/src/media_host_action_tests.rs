use crate::media_host_action::StorybookMediaHostAction;
use crate::preview::PreviewBuilder;
use katana_document_viewer::{ViewerInteractionConfig, ViewerMediaControlKind, ViewerViewport};
use katana_ui_core::render_model::UiHostActionPlan;
use std::path::{Path, PathBuf};

#[test]
fn media_host_action_reads_typed_surface_control_payload() -> Result<(), Box<dyn std::error::Error>>
{
    let scene = PreviewBuilder::default().build(
        &fixture("direct/kdv-icon.png"),
        ViewerViewport {
            width: 800.0,
            height: 600.0,
        },
        true,
        ViewerInteractionConfig {
            image_controls_enabled: true,
            ..ViewerInteractionConfig::default()
        },
    )?;

    let action = UiHostActionPlan::collect_from_tree(&scene.tree)
        .into_iter()
        .filter_map(|plan| StorybookMediaHostAction::from_host_action_plan(&plan))
        .map(StorybookMediaHostAction::into_viewer_action)
        .find(|action| action.command == "zoom-in")
        .ok_or("image zoom-in typed host action missing")?;

    assert_eq!(ViewerMediaControlKind::Image, action.kind);
    assert!(!action.node_id.is_empty());
    assert_eq!("zoom-in", action.command);
    Ok(())
}

#[test]
fn media_surface_control_payload_is_decoded_only_by_storybook_bridge()
-> Result<(), Box<dyn std::error::Error>> {
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let bridge_source = std::fs::read_to_string(source_root.join("media_host_action.rs"))?;

    assert!(bridge_source.contains("UiHostActionPayload::SurfaceControl"));
    assert!(bridge_source.contains("ViewerMediaControlAction::from_host_action"));

    let violations = source_files(&source_root)?
        .into_iter()
        .filter(|path| {
            path.file_name()
                .is_some_and(|name| name != "media_host_action.rs")
        })
        .filter(|path| {
            path.file_stem()
                .and_then(|name| name.to_str())
                .is_some_and(|name| !name.ends_with("_tests") && !name.ends_with("_tests_support"))
        })
        .filter_map(|path| forbidden_media_surface_decode(&path).transpose())
        .collect::<Result<Vec<_>, _>>()?;

    assert!(
        violations.is_empty(),
        "media SurfaceControl payload must be decoded only in media_host_action.rs\n{}",
        violations.join("\n")
    );
    Ok(())
}

fn fixture(label: &str) -> crate::catalog::StorybookFixture {
    crate::catalog::StorybookFixture {
        label: label.to_string(),
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../assets/fixtures/{label}")),
    }
}

fn source_files(source_root: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    let mut queue = vec![source_root.to_path_buf()];
    while let Some(path) = queue.pop() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if entry.file_type()?.is_dir() {
                queue.push(file_path);
            } else if file_path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension == "rs")
            {
                files.push(file_path);
            }
        }
    }
    Ok(files)
}

fn forbidden_media_surface_decode(
    path: &Path,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(path)?;
    let forbidden = [
        "ViewerMediaControlAction::from_host_action",
        "let UiHostActionPayload::SurfaceControl",
        "if let UiHostActionPayload::SurfaceControl",
        "match action.typed_payload",
        "match &action.typed_payload",
    ];
    let Some(needle) = forbidden
        .iter()
        .copied()
        .find(|needle| source.contains(needle))
    else {
        return Ok(None);
    };
    Ok(Some(format!("{} contains `{needle}`", path.display())))
}
