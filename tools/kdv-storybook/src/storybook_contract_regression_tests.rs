#[cfg(test)]
#[test]
fn no_reintroduced_manual_storybook_action_contracts() -> Result<(), Box<dyn std::error::Error>> {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let checks = vec![
        ("no_manual_tree_hit_test", concat!("TEXT", "_HEIGHT")),
        ("no_manual_media_hit_test", concat!("BUTTON", "_WIDTH")),
        ("no_manual_media_hit_test", concat!("BUTTON", "_HEIGHT")),
        (
            "no_manual_host_action_hit_test",
            concat!("rect", "_contains"),
        ),
        (
            "no_storybook_action_synthesis",
            concat!("UiAction", "::", "SetValue"),
        ),
        (
            "no_manual_settings_hit_test",
            concat!("SETTINGS", "_FIELD", "_CONTROL", "_X"),
        ),
        (
            "no_manual_settings_hit_test",
            concat!("SETTINGS", "_TOGGLE", "_WIDTH"),
        ),
        (
            "no_manual_settings_hit_test",
            concat!("SETTINGS", "_TEXT", "_ENTRY", "_WIDTH"),
        ),
        ("no_manual_settings_action", concat!("settings", "-field:")),
        (
            "no_manual_settings_action",
            concat!("mode", "_from", "_label"),
        ),
        (
            "no_manual_settings_action",
            concat!("apply", "_interaction", "_field"),
        ),
        (
            "no_manual_accordion_payload_parse",
            concat!("accordion", "_open", "_payload"),
        ),
        ("no_manual_media_hit_rect", concat!("struct ", "HitRect")),
        (
            "no_style_class_action_contract",
            concat!("parse", "_state", "_id"),
        ),
        (
            "no_style_class_action_contract",
            concat!("viewer", ".", "image", "."),
        ),
        (
            "no_style_class_action_contract",
            concat!("viewer", ".", "diagram", "."),
        ),
        (
            "no_style_class_action_contract",
            concat!("viewer", ".", "code", "."),
        ),
        (
            "no_style_class_action_contract",
            concat!("Ui", "MediaControlTarget"),
        ),
        (
            "no_style_class_action_contract",
            concat!("Ui", "MediaControlAction"),
        ),
        (
            "no_style_class_action_contract",
            concat!("media", "_control", "_action"),
        ),
        (
            "no_style_class_action_contract",
            concat!("kdv", "-task", "-state:"),
        ),
        ("no_manual_settings_action", concat!("= ", "!")),
    ];

    let mut violations = Vec::new();
    let mut queue = vec![source_root];

    while let Some(path) = queue.pop() {
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let file_path = entry.path();
            if file_type.is_dir() {
                queue.push(file_path);
                continue;
            }
            if file_path.extension() != Some(std::ffi::OsStr::new("rs")) {
                continue;
            }
            if file_path
                .file_name()
                .is_some_and(|name| name == "storybook_contract_regression_tests.rs")
            {
                continue;
            }
            let source = std::fs::read_to_string(&file_path)?;
            for (rule, needle) in &checks {
                if !source.contains(needle) {
                    continue;
                }
                let line = source
                    .lines()
                    .enumerate()
                    .find_map(|(index, line)| line.contains(needle).then_some(index + 1))
                    .unwrap_or(0);
                violations.push(format!(
                    "{}: {}:{}: contains `{}` (first hit at line {})",
                    rule,
                    file_path.display(),
                    line,
                    needle,
                    line
                ));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "storybook regression gate: forbidden action-contract terms found\n{}",
        violations.join("\n")
    );
    assert_no_manual_task_action_reconstruction()?;
    assert_sidebar_tree_hover_uses_file_tree_target()?;
    assert_sidebar_hit_preserves_file_tree_action()?;
    assert_sidebar_settings_hover_uses_settings_hit_target()?;
    assert_sidebar_settings_uses_kuc_interaction()?;
    assert_interaction_matrix_sidebar_points_use_named_targets()?;
    assert_sidebar_tests_use_named_kuc_targets()?;
    assert_window_settings_action_uses_apply_result()?;
    assert_storybook_settings_action_uses_kdv_kuc_adapter()?;
    assert_sidebar_settings_content_height_uses_kuc_contract()?;
    assert_frame_hover_uses_kuc_hover_surface_contract()?;
    assert_accordion_click_uses_host_action_router()?;
    assert_media_hit_uses_kdv_kuc_media_action()?;
    assert_mouse_click_uses_host_action_router()?;
    assert_host_action_point_filter_stays_in_router()?;
    assert_runtime_handlers_use_preview_scene_target_lookup()?;
    assert_document_hover_state_uses_single_resolution()?;
    assert_storybook_keeps_diagram_fullscreen_overlay_only()?;
    Ok(())
}

fn assert_no_manual_task_action_reconstruction() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("mouse_task.rs"),
    )?;
    let checks = [
        concat!("task", "_control", "_action", "("),
        concat!("props", "()", ".", "interaction", ".", "value"),
        concat!("props", "()", ".", "context", "_menu"),
        concat!("fn ", "find", "_node"),
        concat!("Viewer", "Task", "State", "::", "from", "_marker"),
    ];
    let violations = checks
        .iter()
        .copied()
        .filter(|needle| source.contains(needle))
        .collect::<Vec<_>>();

    assert!(
        violations.is_empty(),
        "storybook regression gate: mouse_task.rs must consume KUC task action without walking UiNode props, found {:?}",
        violations
    );
    assert!(
        source.contains("UiTaskControlAction"),
        "storybook regression gate: mouse_task.rs must use KUC generic task action"
    );
    assert!(
        !source.contains("KucTaskControlAction"),
        "storybook regression gate: mouse_task.rs must not depend on KDV-KUC task wrapper"
    );
    assert_no_manual_task_context_menu_action_reconstruction()?;
    Ok(())
}

fn assert_no_manual_task_context_menu_action_reconstruction()
-> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("mouse_task_context_menu.rs"),
    )?;
    let checks = [
        "UiTaskMarker",
        "viewer_task_state_from_ui_marker",
        "UiTaskMarker::from_context_menu_item_id",
        "viewer_task_state_from_context_menu_item_id",
        "context_menu_item_id_from_marker",
        "context_menu_item_id_at",
    ];
    let violations = checks
        .iter()
        .copied()
        .filter(|needle| source.contains(needle))
        .collect::<Vec<_>>();

    assert!(
        violations.is_empty(),
        "storybook regression gate: mouse_task_context_menu.rs must consume KUC typed context-menu action without item-id reconstruction, found {:?}",
        violations
    );
    assert!(
        source.contains("context_menu_host_action_at")
            && source.contains("task_control_state_action"),
        "storybook regression gate: task context menu must consume KUC typed item action"
    );
    assert!(
        !source.contains("KucTaskContextMenuAction"),
        "storybook regression gate: task context menu must not depend on KDV-KUC wrapper"
    );
    Ok(())
}

fn assert_media_hit_uses_kdv_kuc_media_action() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("mouse_media_hit.rs"),
    )?;
    let forbidden = [
        concat!("Viewer", "Media", "Control", "Action"),
        concat!("media", "_control", "_from", "_host", "_action"),
        concat!("action", ".", "payload"),
        concat!("payload", ".", "as", "_str"),
    ];
    let violations = forbidden
        .iter()
        .copied()
        .filter(|needle| source.contains(needle))
        .collect::<Vec<_>>();

    assert!(
        violations.is_empty(),
        "storybook regression gate: mouse_media_hit.rs must use KDV-KUC media action conversion instead of parsing host action payloads, found {:?}",
        violations
    );
    assert!(
        source.contains("StorybookMediaHostAction::from_host_action_plan"),
        "storybook regression gate: mouse_media_hit.rs must use Storybook media host action bridge"
    );
    assert!(
        !source.contains("KucMediaControlAction"),
        "storybook regression gate: mouse_media_hit.rs must not depend on KDV-KUC media wrapper"
    );
    assert!(
        source.contains(concat!("media", "_control", "_from", "_viewer", "_action")),
        "storybook regression gate: mouse_media_hit.rs must pass KDV-KUC converted action to ViewerCommandFactory::media_control_from_viewer_action"
    );
    Ok(())
}

fn assert_sidebar_tree_hover_uses_file_tree_target() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("window_mouse.rs"),
    )?;
    let checks = [
        "StorybookSidebar::fixture_hovered_item_id",
        "StorybookSidebar::cursor_at",
        "StorybookSidebar::settings_hit_target",
        "sidebar_content_local_x",
        "sidebar_content_local_y",
        "fn update_sidebar_tree_hover(",
        "fn update_sidebar_settings_hover(",
        "fn sidebar_tree_hovered_item_id(",
        "fn sidebar_settings_hovered_node_id(",
    ];
    let violations = checks
        .iter()
        .copied()
        .filter(|needle| source.contains(needle))
        .collect::<Vec<_>>();

    assert!(
        violations.is_empty(),
        "storybook regression gate: window_mouse.rs must resolve sidebar action/cursor/hover through SidebarHit::interaction, found {:?}",
        violations
    );
    assert!(
        source.contains("sidebar_interaction_for_canvas_point"),
        "storybook regression gate: window_mouse.rs must keep a single sidebar interaction resolver"
    );
    assert!(
        source.contains("fn update_sidebar_hover("),
        "storybook regression gate: window_mouse.rs must update sidebar tree/settings hover from one resolved interaction"
    );
    Ok(())
}

fn assert_sidebar_hit_preserves_file_tree_action() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("sidebar_hit.rs"),
    )?;
    let forbidden = [
        concat!("SidebarHitResult", "::", "Fixture"),
        concat!("position", "(|fixture|"),
        concat!("fixture", ".", "label", " ==", " file_id"),
    ];
    let violations = forbidden
        .iter()
        .copied()
        .filter(|needle| source.contains(needle))
        .collect::<Vec<_>>();
    assert!(
        violations.is_empty(),
        "storybook regression gate: sidebar hit-test must preserve KUC FileTreeAction until window state boundary, found {:?}",
        violations
    );
    assert!(
        source.contains("UiTreeSurfaceHost")
            && source.contains("host_action_hits")
            && source.contains("FileTree::action_from_host_plan"),
        "storybook regression gate: sidebar hit-test must resolve file clicks through rendered KUC host action rects"
    );
    assert!(
        source.contains("self.settings_list.action_from_host_plan"),
        "storybook regression gate: sidebar hit-test must resolve settings clicks through rendered KUC host action rects"
    );
    assert!(
        source.contains(concat!("SidebarHitResult", "::", "FileTree")),
        "storybook regression gate: SidebarHitResult must carry KUC FileTreeAction"
    );
    Ok(())
}

fn assert_sidebar_settings_hover_uses_settings_hit_target() -> Result<(), Box<dyn std::error::Error>>
{
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("window_mouse.rs"),
    )?;
    let checks = [
        concat!("settings", "_hit", "_targets"),
        "StorybookSettingsHitRequest",
        "StorybookSidebar::settings_hit",
        "StorybookSidebar::settings_hit_target",
    ];
    let violations = checks
        .iter()
        .copied()
        .filter(|needle| source.contains(needle))
        .collect::<Vec<_>>();

    assert!(
        violations.is_empty(),
        "storybook regression gate: window_mouse.rs must not own SettingsList hit-test routing, found {:?}",
        violations
    );
    Ok(())
}

fn assert_sidebar_settings_uses_kuc_interaction() -> Result<(), Box<dyn std::error::Error>> {
    let sidebar_hit_source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("sidebar_hit.rs"),
    )?;
    let sidebar_source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("sidebar.rs"),
    )?;
    let sidebar_support_source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("sidebar_test_support.rs"),
    )?;
    let forbidden = [
        "FileTreeHitTestInput",
        "SettingsListHitTestInput",
        "interaction_for_hit",
        "hit_target_with_state",
        "settings_host_action_hits_at",
        "file_tree_host_action_hits_at",
        "fixture_host_action_target",
    ];
    let violations = forbidden
        .iter()
        .copied()
        .filter(|needle| sidebar_hit_source.contains(needle))
        .collect::<Vec<_>>();
    assert!(
        violations.is_empty(),
        "storybook regression gate: sidebar_hit.rs must not route KUC FileTree/SettingsList through separate analytic hit-test contracts, found {:?}",
        violations
    );
    assert!(
        sidebar_hit_source.contains("UiTreeSurfaceHost")
            && sidebar_hit_source.contains("host_action_hits")
            && sidebar_hit_source.contains("FileTree::action_from_host_plan")
            && sidebar_hit_source.contains("self.settings_list.action_from_host_plan"),
        "storybook regression gate: sidebar_hit.rs must resolve runtime clicks from rendered KUC host action rects"
    );
    let sidebar_forbidden = [
        concat!("FileTree::", "hit_target", "_for_item", "_with_state"),
        concat!("list.", "hit_target_for_field"),
        concat!("list.", "hit_target_for_section"),
    ];
    let sidebar_violations = [
        ("sidebar.rs", sidebar_source.as_str()),
        ("sidebar_test_support.rs", sidebar_support_source.as_str()),
    ]
    .iter()
    .flat_map(|(file_name, source)| {
        sidebar_forbidden
            .iter()
            .copied()
            .filter(|needle| source.contains(needle))
            .map(|needle| format!("{file_name}: {needle}"))
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
    assert!(
        sidebar_violations.is_empty(),
        "storybook regression gate: sidebar.rs test helpers must not use KUC analytic hit-target helpers, found {:?}",
        sidebar_violations
    );
    assert!(
        sidebar_support_source.contains("host_action_hit")
            && sidebar_support_source.contains("UiTreeStorybookHost")
            && sidebar_support_source.contains("FileTree::action_from_host_plan")
            && sidebar_support_source.contains("list.action_from_host_plan"),
        "storybook regression gate: sidebar test support must derive points from rendered KUC host action rects"
    );
    Ok(())
}

fn assert_interaction_matrix_sidebar_points_use_named_targets()
-> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("window")
            .join("interaction_matrix_support.rs"),
    )?;
    let forbidden = [
        concat!("settings", "_hit", "_targets"),
        concat!("Storybook", "Settings", "Targets", "Request"),
        concat!("Settings", "List", "Action"),
        concat!("Sidebar", "Hit"),
        "for y in 0..",
    ];
    let violations = forbidden
        .iter()
        .copied()
        .filter(|needle| source.contains(needle))
        .collect::<Vec<_>>();

    assert!(
        violations.is_empty(),
        "storybook regression gate: interaction matrix must use rendered KUC sidebar action points instead of scanning hit targets, found {:?}",
        violations
    );
    assert!(
        source.contains(concat!("settings", "_field", "_canvas", "_point")),
        "storybook regression gate: interaction matrix must use StorybookSidebar::settings_field_canvas_point"
    );
    assert!(
        source.contains(concat!(
            "fixture", "_canvas", "_point", "_for", "_item", "_id"
        )),
        "storybook regression gate: interaction matrix must use StorybookSidebar::fixture_canvas_point_for_item_id"
    );
    Ok(())
}

fn assert_sidebar_tests_use_named_kuc_targets() -> Result<(), Box<dyn std::error::Error>> {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let sidebar_source = std::fs::read_to_string(source_root.join("sidebar.rs"))?;
    let sidebar_hit_source = std::fs::read_to_string(source_root.join("sidebar_hit.rs"))?;
    let window_tests_source = std::fs::read_to_string(source_root.join("window_tests.rs"))?;
    let sidebar_hit_tests_source =
        std::fs::read_to_string(source_root.join("sidebar_hit_tests.rs"))?;
    let forbidden_sources = [
        (
            "sidebar.rs",
            sidebar_source.as_str(),
            [
                concat!("settings", "_hit", "_targets"),
                concat!("Storybook", "Settings", "Targets", "Request"),
            ]
            .as_slice(),
        ),
        (
            "sidebar_hit.rs",
            sidebar_hit_source.as_str(),
            [concat!("fixture", "_index")].as_slice(),
        ),
        (
            "window_tests.rs",
            window_tests_source.as_str(),
            [concat!("settings", "_hit", "_targets"), "for y in 0.."].as_slice(),
        ),
        (
            "sidebar_hit_tests.rs",
            sidebar_hit_tests_source.as_str(),
            [concat!("settings", "_hit", "_targets"), "for y in 0.."].as_slice(),
        ),
    ];
    let violations = forbidden_sources
        .iter()
        .flat_map(|(file_name, source, needles)| {
            needles
                .iter()
                .copied()
                .filter(|needle| source.contains(needle))
                .map(|needle| format!("{file_name}: {needle}"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert!(
        violations.is_empty(),
        "storybook regression gate: sidebar tests must use rendered KUC host action targets\n{}",
        violations.join("\n")
    );
    Ok(())
}

fn assert_mouse_click_uses_host_action_router() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("mouse.rs"),
    )?;
    let start = required_substring_index(
        &source,
        "pub(crate) fn command_for_click",
        "mouse.rs should keep command_for_click",
    )?;
    let end = required_substring_index_after(
        &source,
        start,
        "pub(crate) fn command_label",
        "command_for_click should appear before command_label",
    )?;
    let body = &source[start..end];

    assert!(
        body.contains("StorybookHostActionRouter::for_window"),
        "storybook regression gate: click command routing must create one host action router"
    );
    assert!(
        !body.contains("StorybookHostActionHits::hits"),
        "storybook regression gate: click command routing must not fetch host action hits per handler"
    );
    assert!(
        source.contains("UiTextSpanAction"),
        "storybook regression gate: mouse.rs must consume KUC generic text span action"
    );
    assert!(
        source.contains("OpenLink"),
        "storybook regression gate: mouse.rs must route KUC generic link action"
    );
    assert!(
        !source.contains("KucLinkOpenAction"),
        "storybook regression gate: mouse.rs must not depend on KDV-KUC link wrapper"
    );
    Ok(())
}

fn assert_host_action_point_filter_stays_in_router() -> Result<(), Box<dyn std::error::Error>> {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let checked_files = [
        "mouse.rs",
        "mouse_task.rs",
        "mouse_media.rs",
        "mouse_media_hit.rs",
        "mouse_accordion.rs",
        "mouse_cursor.rs",
    ];
    let mut violations = Vec::new();
    for file_name in checked_files {
        let source = std::fs::read_to_string(source_root.join(file_name))?;
        for needle in ["contains_point(", "router.hits()"] {
            if source.contains(needle) {
                violations.push(format!("{file_name}: contains `{needle}`"));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "storybook regression gate: document action handlers must use StorybookHostActionRouter::hits_at, found {:?}",
        violations
    );

    let router_source = std::fs::read_to_string(source_root.join("mouse_host_action.rs"))?;
    assert!(
        router_source.contains("pub(super) fn hits_at"),
        "storybook regression gate: host action point filtering must be centralized in StorybookHostActionRouter::hits_at"
    );
    assert!(
        !router_source.contains("pub(super) fn hit_at"),
        "storybook regression gate: StorybookHostActionHits::hit_at must not reintroduce a parallel point-filter API"
    );
    assert!(
        !router_source.contains("contains_point("),
        "storybook regression gate: KDV router must delegate rendered host action point filtering to KUC"
    );
    assert!(
        router_source.contains("UiTreeSurfaceHost"),
        "storybook regression gate: KDV router must use KUC surface host contract"
    );
    Ok(())
}

fn assert_runtime_handlers_use_preview_scene_target_lookup()
-> Result<(), Box<dyn std::error::Error>> {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let checked_files = [
        "mouse.rs",
        "mouse_cursor.rs",
        "mouse_task.rs",
        "mouse_media_hit.rs",
    ];
    let mut violations = Vec::new();
    for file_name in checked_files {
        let source = std::fs::read_to_string(source_root.join(file_name))?;
        if source.contains("scene.targets.iter()")
            || source.contains("scene\n            .targets")
            || source.contains("scene.target_for_node_id")
            || source.contains("scene.target_containing_point")
        {
            violations.push(file_name);
        }
    }

    assert!(
        violations.is_empty(),
        "storybook regression gate: document action handlers must use StorybookHostActionRouter target resolution instead of direct PreviewScene target lookup, found {:?}",
        violations
    );
    let router_source = std::fs::read_to_string(source_root.join("mouse_host_action.rs"))?;
    assert!(
        router_source.contains("pub(super) fn target_for_node_id"),
        "storybook regression gate: host action router must own document target lookup"
    );
    assert!(
        router_source.contains("fn viewport_node_hits")
            && router_source.contains(".viewport_node_hits("),
        "storybook regression gate: document hover must use KUC viewport node hit contract"
    );
    assert!(
        !router_source.contains("target_containing_point"),
        "storybook regression gate: document hover must not resolve document target from AST point hit-test"
    );

    let preview_source = std::fs::read_to_string(source_root.join("preview_scene.rs"))?;
    let lookup_start = required_substring_index(
        &preview_source,
        "pub fn target_for_node_id",
        "PreviewScene should keep target_for_node_id",
    )?;
    let lookup_end = required_substring_index_after(
        &preview_source,
        lookup_start,
        "pub fn target_for_internal_anchor",
        "target_for_node_id should appear before target_for_internal_anchor",
    )?;
    let lookup_body = &preview_source[lookup_start..lookup_end];
    assert!(
        lookup_body.contains("target_lookup"),
        "storybook regression gate: target_for_node_id must use the scene target index"
    );
    assert!(
        !lookup_body.contains(".iter()"),
        "storybook regression gate: target_for_node_id must not scan scene.targets"
    );
    assert!(
        !preview_source.contains("hit_test_index")
            && !preview_source.contains("target_containing_point"),
        "storybook regression gate: PreviewScene must not keep AST point hit-test state for hover"
    );
    Ok(())
}

fn assert_document_hover_state_uses_single_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let window_source = std::fs::read_to_string(source_root.join("window_mouse.rs"))?;
    let forbidden_window_helpers = [
        "hovered_node_for_window",
        "hovered_action_node_for_window",
        "hovered_node_for_canvas_point",
        "hovered_action_node_for_canvas_point",
    ];
    let window_violations = forbidden_window_helpers
        .iter()
        .copied()
        .filter(|needle| window_source.contains(needle))
        .collect::<Vec<_>>();
    assert!(
        window_violations.is_empty(),
        "storybook regression gate: document hover must be resolved as one hover state, found {:?}",
        window_violations
    );
    assert!(
        window_source.contains("cached_document_hover_state_for_canvas_point"),
        "storybook regression gate: window hover must use a cached single document hover state resolver"
    );

    let cursor_source = std::fs::read_to_string(source_root.join("mouse_cursor.rs"))?;
    let router_count = cursor_source
        .match_indices("StorybookHostActionRouter::for_window")
        .count();
    assert_eq!(
        1, router_count,
        "storybook regression gate: mouse_cursor.rs must create one host action router through hover_state_for_hover"
    );
    for needle in ["host_action_hit", "host_action_cursor"] {
        assert!(
            !cursor_source.contains(needle),
            "storybook regression gate: mouse_cursor.rs must not keep parallel `{needle}` helpers"
        );
    }
    Ok(())
}

fn assert_window_settings_action_uses_apply_result() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("window_keyboard.rs"),
    )?;
    let start = required_substring_index(
        &source,
        "pub(super) fn apply_settings_action",
        "window_keyboard.rs should keep apply_settings_action",
    )?;
    let end = required_substring_index_after(
        &source,
        start,
        "fn apply_settings_side_effect",
        "apply_settings_action should appear before apply_settings_side_effect",
    )?;
    let body = &source[start..end];
    let forbidden = [
        concat!("SettingsListAction", "::", "UpdateField"),
        concat!("StorybookSettingsField", "::", "from_id"),
        "field_id",
    ];
    let violations = forbidden
        .iter()
        .copied()
        .filter(|needle| body.contains(needle))
        .collect::<Vec<_>>();
    assert!(
        violations.is_empty(),
        "storybook regression gate: window settings action must consume StorybookSettingsApplyResult instead of decoding SettingsListAction, found {:?}",
        violations
    );
    assert!(
        body.contains("result.field"),
        "storybook regression gate: window settings action must use StorybookSettingsApplyResult.field"
    );
    Ok(())
}

fn assert_storybook_settings_action_uses_kdv_kuc_adapter() -> Result<(), Box<dyn std::error::Error>>
{
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let removed_module = source_root.join("settings_action_value.rs");
    assert!(
        !removed_module.exists(),
        "storybook regression gate: settings value conversion must stay inside settings action boundary"
    );

    let source = std::fs::read_to_string(source_root.join("settings_action.rs"))?;
    let forbidden = [
        "KucSettingsAction",
        "KucSettingsUpdateAction",
        "KucSettingsToggleAction",
    ];
    let violations = forbidden
        .iter()
        .copied()
        .filter(|needle| source.contains(needle))
        .collect::<Vec<_>>();
    assert!(
        violations.is_empty(),
        "storybook regression gate: settings_action.rs must not depend on KDV-KUC settings wrapper, found {:?}",
        violations
    );
    assert!(
        source.contains("SettingsListAction::UpdateField"),
        "storybook regression gate: settings_action.rs must consume KUC settings update action"
    );
    assert!(
        source.contains("SettingsListAction::ToggleSection"),
        "storybook regression gate: settings_action.rs must consume KUC settings toggle action"
    );
    Ok(())
}

fn assert_sidebar_settings_content_height_uses_kuc_contract()
-> Result<(), Box<dyn std::error::Error>> {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let removed_module = source_root.join("sidebar_content_height.rs");
    assert!(
        !removed_module.exists(),
        "storybook regression gate: settings content height must come from KUC SettingsList, not sidebar_content_height.rs"
    );

    let source = std::fs::read_to_string(source_root.join("sidebar.rs"))?;
    assert!(
        source.contains(".content_height()"),
        "storybook regression gate: sidebar settings height must call KUC SettingsList::content_height"
    );
    assert!(
        !source.contains("UiNode::from(list"),
        "storybook regression gate: sidebar settings height must not walk rendered UiNode"
    );
    Ok(())
}

fn assert_frame_hover_uses_kuc_hover_surface_contract() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("frame.rs"),
    )?;
    let forbidden = ["KucViewerHoverSurface"];
    let violations = forbidden
        .iter()
        .copied()
        .filter(|needle| source.contains(needle))
        .collect::<Vec<_>>();

    assert!(
        violations.is_empty(),
        "storybook regression gate: frame hover must not depend on KDV-KUC hover wrapper, found {:?}",
        violations
    );
    assert!(
        source.contains("with_hover_surface_for_node_id")
            && source.contains("with_hovered_node_id"),
        "storybook regression gate: frame hover must stage KUC hover surface through UiTree contract"
    );
    assert!(
        !source.contains("StorybookFrameHover::draw_at"),
        "storybook regression gate: frame hover must not draw a KDV-side overlay"
    );
    Ok(())
}

fn assert_accordion_click_uses_host_action_router() -> Result<(), Box<dyn std::error::Error>> {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let window_source = std::fs::read_to_string(source_root.join("window_accordion.rs"))?;
    assert!(
        window_source.contains("StorybookMouseAccordion::toggle_for_click"),
        "storybook regression gate: window accordion click must use StorybookMouseAccordion"
    );
    assert!(
        !window_source.contains("accordion_open_overrides.insert"),
        "storybook regression gate: window accordion click must not write accordion overrides directly"
    );
    assert!(
        !window_source.contains("!hit.open"),
        "storybook regression gate: window accordion click must not synthesize requested accordion state"
    );

    let cursor_source = std::fs::read_to_string(source_root.join("mouse_cursor.rs"))?;
    assert!(
        !cursor_source.contains("accordion_toggle_for_click"),
        "storybook regression gate: mouse_cursor.rs must not own accordion click routing"
    );

    let accordion_source = std::fs::read_to_string(source_root.join("mouse_accordion.rs"))?;
    assert!(
        accordion_source.contains("StorybookHostActionRouter::for_window"),
        "storybook regression gate: accordion click must create one host action router"
    );
    assert!(
        accordion_source.contains("accordion_toggle_action"),
        "storybook regression gate: accordion click must consume KUC generic accordion toggle action"
    );
    assert!(
        !accordion_source.contains("ToggleAccordion"),
        "storybook regression gate: accordion click must not destructure KUC action payload"
    );
    assert!(
        !accordion_source.contains("!open"),
        "storybook regression gate: accordion click must not synthesize requested accordion state"
    );
    assert!(
        !accordion_source.contains("KucAccordionToggleAction"),
        "storybook regression gate: accordion click must not depend on KDV-KUC accordion wrapper"
    );
    assert!(
        !accordion_source.contains("StorybookHostActionHits::hits"),
        "storybook regression gate: accordion click must not fetch raw host action hits directly"
    );
    Ok(())
}

fn assert_storybook_keeps_diagram_fullscreen_overlay_only() -> Result<(), Box<dyn std::error::Error>>
{
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let native_file = source_root.join("window_native_fullscreen.rs");
    assert!(
        !native_file.exists(),
        "storybook regression gate: diagram fullscreen is a KatanA viewer overlay; Storybook must not add a native fullscreen bridge"
    );
    let loop_source = std::fs::read_to_string(source_root.join("window_loop.rs"))?;
    assert!(
        loop_source.contains("drain_diagram_fullscreen_events"),
        "storybook regression gate: live loop must consume diagram fullscreen host events without OS window side effects"
    );
    assert!(
        loop_source.contains("StorybookHostEvent::DiagramFullscreen"),
        "storybook regression gate: fullscreen host notification must stay typed"
    );

    let mut violations = Vec::new();
    let mut queue = vec![source_root];
    let forbidden_native_fullscreen = [
        concat!("toggle", "Full", "Screen"),
        concat!("set", "_native", "_fullscreen"),
        concat!("is", "_native", "_fullscreen"),
        concat!("Storybook", "Native", "Fullscreen", "Host"),
        concat!("diagram", "_native", "_fullscreen", "_state"),
    ];

    while let Some(path) = queue.pop() {
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let file_path = entry.path();
            if file_type.is_dir() {
                queue.push(file_path);
                continue;
            }
            if file_path.extension() != Some(std::ffi::OsStr::new("rs")) {
                continue;
            }
            if file_path
                .file_name()
                .is_some_and(|name| name == "storybook_contract_regression_tests.rs")
            {
                continue;
            }
            let source = std::fs::read_to_string(&file_path)?;
            for needle in forbidden_native_fullscreen {
                if !source.contains(needle) {
                    continue;
                }
                let line = source
                    .lines()
                    .enumerate()
                    .find_map(|(index, line)| line.contains(needle).then_some(index + 1))
                    .unwrap_or(0);
                violations.push(format!(
                    "{}:{} contains `{}`",
                    file_path.display(),
                    line,
                    needle
                ));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "storybook regression gate: Storybook must not perform native fullscreen for diagram overlays\n{}",
        violations.join("\n")
    );
    Ok(())
}

fn required_substring_index(
    source: &str,
    needle: &str,
    message: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    match source.find(needle) {
        Some(index) => Ok(index),
        None => Err(std::io::Error::other(message).into()),
    }
}

fn required_substring_index_after(
    source: &str,
    start: usize,
    needle: &str,
    message: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    required_substring_index(&source[start..], needle, message).map(|index| start + index)
}
