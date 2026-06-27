# KDV v0.2.0 Viewer Recovery Plan

最終更新: 2026-06-13

## 結論

KDV v0.2.0 は、KatanA の document viewer / slideshow を参照仕様として、vendor 非依存の KDV engine と KUC 共通 UI 契約へ移植する。

`just storybook` は smoke ではなく、KUC 実部品で構成された Katana 由来 viewer を interactive 起動する。完了判定は `visual_score`、`semantic_score`、`interaction_score`、`performance_score` の全カテゴリ 95 点以上とし、平均点ではなく最小値で判定する。

現時点では DoD 未達である。`just storybook-score-check` が通る状態でも、ユーザー実機指摘の TreeView、Toggle、link、accordion、diagram control、task checkbox、OS emoji、scroll / resize、score false positive が残る限り完了扱いにしない。

## 参照仕様

KatanA 側の以下を参照仕様として固定する。

- section 分解: `/Users/hiroyuki_furuno/works/private/katana/crates/katana-core/src/preview/section/mod.rs`
- section 型: `/Users/hiroyuki_furuno/works/private/katana/crates/katana-core/src/preview/types.rs`
- Markdown hooks: `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/section_show/markdown/mod.rs`
- lazy / pending / worker: `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/core_render.rs`
- render polling: `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/background.rs`
- media controls: `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/diagram_controller.rs`
- task checkbox / context menu: `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/widgets/markdown_hooks/`
- table / HTML / emoji: `extension_table/renderer.rs`, `preview_pane/html.rs`, `katana-core/src/emoji/raster/`

KatanA は reference implementation。KUC public API に KatanA 固有 namespace は入れない。

## Fixture / Reference Inventory

KatanA fixture は KDV repo 内へコピーして固定する。他 repository 相対パス参照は禁止する。

配置先:

```text
assets/fixtures/katana/
assets/fixtures/katana/drawio/basic/
assets/fixtures/direct/
assets/reference/katana/html/
assets/reference/katana/pdf/
assets/reference/katana/screenshots/
```

取り込む fixture:

```text
assets/fixtures/sample.md
assets/fixtures/sample.ja.md
assets/fixtures/sample_basic.md
assets/fixtures/sample_basic.ja.md
assets/fixtures/sample_diagrams.md
assets/fixtures/sample_diagrams.ja.md
assets/fixtures/sample_mermaid.md
assets/fixtures/sample_mermaid_ja.md
assets/fixtures/sample_html.md
assets/fixtures/sample_html.ja.md
assets/fixtures/drawio/README.md
assets/fixtures/drawio/basic/01-empty-mxfile.drawio
assets/fixtures/drawio/basic/02-standalone-mxgraphmodel.drawio
assets/fixtures/drawio/basic/03-basic-flow.drawio
assets/fixtures/drawio/basic/04-shape-style-matrix.drawio
assets/fixtures/drawio/basic/05-edge-variants.drawio
assets/fixtures/drawio/basic/06-multi-page.drawio
assets/fixtures/drawio/basic/07-html-labels-and-entities.drawio
assets/fixtures/drawio/basic/08-group-container.drawio
assets/fixtures/drawio/basic/09-layers-and-swimlane.drawio
assets/fixtures/drawio/basic/10-userobject-metadata.drawio
assets/fixtures/drawio/basic/11-japanese-labels.drawio
assets/fixtures/drawio/basic/12-vars-placeholders.drawio
```

`direct` fixture は source kind の最小検証だけに限定する。冗長な独自 Markdown coverage fixture は残さない。

## Current Violations To Block

実装前または修正前に、以下を red test / lint で fail させる。

| 違反 | 対象 | gate |
| --- | --- | --- |
| TreeView row を KDV が推測する | Storybook sidebar | `no_manual_tree_hit_test` |
| KDV が `UiAction::SetValue` を合成する | Storybook sidebar | `no_storybook_action_synthesis` |
| media button を固定矩形で判定する | media mouse handling | `no_manual_media_hit_test` |
| media action を `state_id` / `style_class` で復元する | media bridge | `no_style_class_action_contract` |
| SettingsList action を合成する | settings handling | `no_manual_settings_action` |
| score が自己比較や空画面で通る | score gate | `no_self_parity_score` |
| `just storybook` が起動せず smoke だけになる | just entrypoint | `storybook_entrypoint_contract_tests` |

## KDV Core Contract

KDV は viewer engine のみを担当する。

```rust
pub struct ViewerDocument {
    pub document_id: DocumentId,
    pub source_kind: SourceKind,
    pub source_text: String,
    pub sections: Vec<ViewerSection>,
    pub outline: Vec<DocumentOutlineItem>,
}

pub enum ViewerSection {
    Markdown(MarkdownSection),
    Html(HtmlSection),
    Pdf(PdfSection),
    Diagram(DiagramSection),
    Image(ImageSection),
}

pub enum RenderedSection {
    Loaded(RenderedNode),
    Pending(PendingRender),
    Failed(RenderFailure),
}
```

全 section は `section_id`、`source_span`、`line_start`、`line_count` を持つ。失敗時は log + diagnostic node + raw string node を返す。別 renderer への fallback は禁止する。

必須 node kind:

```text
DocumentRoot / Section / Heading / Paragraph / Text / Link /
CodeBlock / Table / List / ListItem / TaskCheckbox /
Alert / BlockQuote / HorizontalRule / Footnote / Accordion /
HtmlBlock / Diagram / Image / PdfPage / Math / Pending / Error
```

## KUC API Migration

KUC が実操作 action を返し、KDV は座標から file id、button action、task row、link span を復元しない。

```rust
pub enum FileTreeAction {
    SelectFile { file_id: String },
    ToggleDirectory { directory_id: String },
    FocusItem { item_id: String },
    None,
}
```

```rust
pub enum SettingsListAction {
    SetQuery { query: Option<String> },
    ToggleSection { section_id: String },
    UpdateField { field_id: String, value: SettingsValue },
    ResetField { field_id: String },
}
```

```rust
pub enum MediaControlAction {
    Fullscreen,
    CopySource,
    CopyRendered,
    PanUp,
    PanDown,
    PanLeft,
    PanRight,
    ZoomIn,
    ZoomOut,
    Reset,
    Info,
}
```

```rust
pub enum TaskMarker {
    Todo,
    Done,
    InProgress,
    Blocked,
}

pub enum TaskControlAction {
    Toggle { task_id: TaskId, source_span: SourceSpan },
    SetMarker { task_id: TaskId, marker: TaskMarker, source_span: SourceSpan },
}
```

```rust
pub enum TextSpanAction {
    OpenLink { target: LinkTarget },
    CopyCode { node_id: NodeId },
    ToggleAccordion { node_id: NodeId },
}
```

移行条件:

- KUC が row height / indent / virtual range / scroll offset と同じ計算で hit-test する。
- KDV は KUC action を viewer state / command に反映するだけにする。
- Storybook は KUC 実部品を host し、独自 TreeView、独自 toggle、独自 settings UI、独自 media control、独自 hit-test、独自 action 合成を追加しない。

## Storybook Contract

`just storybook` は interactive viewer を起動する。

左上:

```text
KUC FileTree / TreeView
- katana/markdown
- katana/html
- katana/diagram
- katana/image
- katana/pdf
- direct
```

左下:

```text
KUC SettingsList
- theme
- dark
- mode
- slideshow
- hover highlight
- selection
- image controls
- diagram controls
- search query
- active search index
- viewport
- loaded assets
- failed assets
- render latency
- category scores
```

右側:

```text
KUC viewer
- vertical scroll
- dynamic resize
- bottom spacer
- dark theme passthrough
- KatanA compatible document/slideshow rendering
```

## Asset Scheduler

図形・画像・PDF は lazy / parallel load にする。

```rust
pub struct AssetRequest {
    pub request_id: AssetRequestId,
    pub section_id: SectionId,
    pub kind: AssetKind,
    pub source_hash: SourceHash,
    pub theme: ViewerTheme,
}

pub enum AssetState {
    Pending,
    Loaded(AssetSurface),
    Failed(RenderFailure),
}
```

条件:

- first frame は本文と pending を先に出す。
- pending 表示は 0.2 秒以内。
- diagram render は UI thread を止めない。
- file switch 時は古い job を cancellation する。
- cache key は `kind + source_hash + theme + relevant options`。
- dark theme は renderer request に含める。
- failed は raw string node + diagnostic node。

## Score Gate

`just storybook-score-check` は最終 score を平均ではなく最小値で扱う。

```text
final_score = min(
  visual_score,
  semantic_score,
  interaction_score,
  performance_score
)
```

全カテゴリ 95 点以上必須。

### visual_score

入力:

```text
assets/reference/katana/html/*.png
assets/reference/katana/pdf/*.png
KDV Storybook screenshot
```

採点:

```text
visual_score = min(
  pixel_similarity_score,
  content_preservation_score,
  layout_bounds_score,
  dark_theme_delta_score
)
```

KDV export と KDV Storybook の自己比較だけでは fail。

### semantic_score

対象:

```text
node kind
source span
anchor
link target
task marker
code language
syntax span
search match
accordion state
media action
copy action
```

### interaction_score

対象:

```text
FileTree select
SettingsList update
scroll
resize
link hover cursor
link click
search next/previous jump
task click
task context menu
accordion toggle
diagram fullscreen
diagram pan / zoom / reset
diagram copy source
code copy
```

### performance_score

基準:

```text
file switch first frame <= 0.6s
pending display <= 0.2s
scroll / resize: 操作不能停止なし
diagram completion: lazy completion 可
```

## Test Matrix

| Phase | Test |
| --- | --- |
| Fixture | `katana_fixture_inventory_tests` |
| Reference | `katana_reference_artifact_tests` |
| Linter | `no_manual_tree_hit_test_tests` |
| Linter | `no_manual_media_hit_test_tests` |
| Linter | `no_manual_settings_action_tests` |
| Linter | `no_style_class_action_contract_tests` |
| KUC | `file_tree_contract_tests` |
| KUC | `settings_list_contract_tests` |
| KUC | `media_control_contract_tests` |
| KUC | `task_control_contract_tests` |
| KUC | `text_span_action_contract_tests` |
| KDV | `section_pipeline_tests` |
| KDV | `markdown_node_contract_tests` |
| KDV | `html_alignment_contract_tests` |
| KDV | `diagram_lazy_asset_tests` |
| KDV | `search_anchor_scroll_tests` |
| Storybook | `storybook_file_tree_interaction_tests` |
| Storybook | `storybook_settings_interaction_tests` |
| Storybook | `storybook_media_control_interaction_tests` |
| Storybook | `storybook_task_interaction_tests` |
| Score | `storybook_score_visual_tests` |
| Score | `storybook_score_semantic_tests` |
| Score | `storybook_score_interaction_tests` |
| Score | `storybook_score_performance_tests` |

## Implementation Order

1. fixture / reference artifact の正本化。
2. linter rules を追加し、現行 Storybook の不正実装で fail させる。
3. KUC FileTree / SettingsList / MediaControl / TaskControl / TextSpanAction contract を追加する。
4. KDV viewer model を section pipeline に整理する。
5. Markdown / HTML / PDF / diagram / image node conversion を KatanA 互換にする。
6. AssetScheduler を lazy / parallel / cancellation / cache / theme 対応にする。
7. KDV-KUC adapter を model -> KUC UiNode 変換専用に戻す。
8. Storybook を KUC 実部品 host として再構築する。
9. interaction E2E を全項目通す。
10. `just storybook-score-check` を全カテゴリ 95 点以上にする。
11. `just storybook` を interactive viewer として最終確認する。
12. egui / gpui / floem adapter 復帰可否はこの後に判断する。

## Required Commands

```text
just clean
just kdv-lint
just kuc-adapter-boundary-check
cargo test --workspace --locked
just storybook-window-smoke
just storybook-interaction-check
just storybook-content-check
just storybook-performance-check
just storybook-score-check
just storybook
```

## Acceptance Criteria

- [ ] KatanA fixtures が KDV assets に固定されている。
- [ ] KatanA reference HTML/PDF/screenshots が KDV assets に固定されている。
- [ ] Storybook に manual TreeView hit-test がない。
- [ ] Storybook に manual media hit-test がない。
- [ ] Storybook に settings action 合成がない。
- [ ] style class / state id が action contract になっていない。
- [ ] KDV core に vendor crate 依存がない。
- [ ] KUC FileTree / TreeView action で file selection が動く。
- [ ] KUC SettingsList action で設定変更が動く。
- [ ] KUC MediaControl action で diagram/image 操作が動く。
- [ ] Markdown が raw 表示ではなく node 表示される。
- [ ] HTML alignment が表示に反映される。
- [ ] table / alert / code / horizontal rule が表現される。
- [ ] link hover cursor と click が span 単位で動く。
- [ ] task checkbox と context menu が動く。
- [ ] search highlight と jump が動く。
- [ ] accordion が動く。
- [ ] diagram pending が即時表示される。
- [ ] diagram dark theme が renderer に渡る。
- [ ] scroll / resize が動く。
- [ ] visual / semantic / interaction / performance score が全カテゴリ 95 点以上。
- [ ] `just storybook` が interactive viewer を起動する。

## 正本リンク

- 実装方針: `design.md`
- 残作業正本: `remaining-plan.md`
- ユーザー実機指摘台帳: `user-feedback-todo.md`
- 現在の未達引き継ぎ: `handoff-unresolved-2026-06-12.md`
- 最新入口: `handoff-current-2026-06-13.md`
