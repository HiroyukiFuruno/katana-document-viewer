# Tasks: katana-document-viewer v0.1.0

## Definition of Ready (DoR)

- [ ] proposal / design / spec / tasks の対象capability名が一致している
- [ ] KMM / KDV / KDR / KatanA の責務境界がdesign.mdとspec.mdで明示されている
- [ ] v0.1.0で実装する範囲と、後続versionへ送る範囲がtasks.mdから判定できる

## Definition of Done (DoD)

- [ ] `scripts/openspec validate v0-1-0-document-preview-extraction --strict` が通る
- [ ] `cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace` が通る
- [ ] KDV AST lintで色literal違反、preset直接参照違反、許容fixtureの正常系を検証している
- [ ] CommonMark / GFM / KatanA互換fixtureのviewer回帰テストが、spec.mdのScenarioと対応している
- [ ] KMM AST由来の目次（TOC）、TOC click scroll、preview/editor active heading連動の回帰テストがspec.mdのScenarioと対応している
- [ ] KatanA統合側が `ViewerConfig` に `ViewerTheme` / `ViewerI18n` / `ViewerInteractionConfig` を明示して渡すことを確認している

---

## Branch Rule

- **標準ブランチ**: `release/v0.1.0`
- **作業ブランチ**: `feature/v0.1.0-task-x`

---

## 1. neutral interface を確定する

- [ ] 1.1 `DocumentViewer` trait と DTO（`ViewerSource` / `ViewerConfig` / `ViewerTheme` / `ViewerI18n` / `ViewerInteractionConfig` / `ViewerOutput` / `ViewerDiagnostics` / `ExportConfig`）を本実装向けに拡張する
- [ ] 1.2 `katana-document-viewer` の `cargo tree` に `egui` が含まれないことを確認する
- [ ] 1.3 KatanA が interface crate のみを依存しても型エラーが出ないことを確認する
- [ ] 1.4 editor-viewer同期制御をKDVが持たず、KatanAがviewerまたはeditorへ命令する契約を明記する
- [ ] 1.5 `ViewerConfig` で `ViewerTheme`、`ViewerI18n`、`ViewerInteractionConfig` をnull不可の必須入力にし、KDV提供のdefault theme preset、英語（en）i18n preset、default interaction presetも呼び出し側が必ず明示引数として渡す制約を型で担保する
- [ ] 1.6 `ViewerOutput` / `ViewerDiagnostics` / hit-test metadata / unresolved metadata表示の最小fieldとAPIを実装し、KMM node id と source range へ戻れることを単体テストで確認する
- [ ] 1.7 KMM node id、source range、heading anchor、scroll fractionを受ける外部scroll制御APIを追加し、成功/失敗結果を返す単体テストを追加する
- [ ] 1.8 KMM AST由来の見出し構造を `ViewerTocItem` 相当のDTOとして公開し、level、表示text、KMM node id、source range、heading anchor候補を保持する

---

## 2. KatanA viewer 実装を移管する

- [ ] 2.1 KMM public DTOをviewer inputとして受け取る
- [ ] 2.2 CommonMark 0.31.2 のblock / inline記法を描画対象表として固定する
- [ ] 2.3 GFM 0.29-gfm のtable、列の左寄せ・中央寄せ・右寄せ、横幅100%固定、task list、strikethrough、autolink、raw HTML safetyを描画対象表として固定する
- [ ] 2.4 GitHub Docs互換のfootnote、alert、emoji shortcode、relative link、heading anchorをKatanA互換対象に含める
- [ ] 2.5 KatanA独自仕様（中央寄せHTML、badge row、legacy note、description list、`[-]` / `[/]` task marker、details accordion、`$$` 内側の半角スペースを許容するmath、inline math、Draw.io直接code block、`.drawio` / `.xml` 添付・参照先の先頭Draw.io判定、ZenUML、長い行、日本語、HTML entity）をKMM canonical fixturesとKatanA現行previewから確認する
- [ ] 2.6 `katana-document-viewer-floem` にKMM node renderingの土台を追加する
- [ ] 2.7 KMMが専用nodeを持たないlink、image、footnote、HTML inline、inline mathはraw snippetとsource rangeを保持し、未描画時もrawをそのまま表示する
- [ ] 2.8 ダイアグラム描画を `katana-diagram-renderer` 経由に統一する（KDV内で独自 Mermaid / Draw.io / ZenUML / PlantUML / math renderer を持たない）
- [ ] 2.9 `egui_commonmark` vendor patchを正規経路にしないことを確認する
- [ ] 2.10 KMM canonical fixtures（`katana_sample.md` / `katana_sample_basic.md` / `katana_readme.md` / `description_list.md`）相当のviewer回帰テストを追加し、対応するspec.md Scenario名をtest名またはtest commentに残す
- [ ] 2.11 table alignment/width、GitHub alert、寛容なmath、Draw.io直接code block、`.drawio` / `.xml` 添付・参照先の先頭Draw.io判定とSVG化、ZenUMLのviewer回帰テストを追加する
- [ ] 2.12 外部描画失敗時にrawをcode block枠でそのまま表示し、枠borderをtheme由来のerror系カラーにし、preview上でエラーアイコン、代表メッセージ、tooltip詳細を表示する回帰テストを追加する
- [ ] 2.13 preview内の代表メッセージ、tooltip label、空状態など固定表示文言を `ViewerI18n` から取得し、KDV提供の英語（en）presetで表示できる回帰テストを追加する。日本語（ja）presetはKatanAなど呼び出し側責務として扱う
- [ ] 2.14 KDV AST lintを拡張し、preset定義、test fixture、lint違反fixtureを除くrendering code内のhard-coded color literalとpreset直接参照を禁止する
- [ ] 2.15 CommonMark 0.31.2 examples と GFM 0.29-gfm examples のうちKMMがDTO化済みの範囲をviewer snapshot testへ追加し、KMM未対応範囲はraw保持テストで担保する
- [ ] 2.16 relative link click、heading anchor、emoji shortcode未知値、raw HTML disallowed要素、prose長文折り返し、code block横スクロールの回帰テストを追加する
- [ ] 2.17 hover highlightをtheme由来のhover色で表示し、`hover_highlight_enabled=false` で非表示になる回帰テストを追加する
- [ ] 2.18 画像制御群と図形制御群をhover/focus時に表示し、`image_controls_enabled=false` / `diagram_controls_enabled=false` で非表示になる回帰テストを追加する
- [ ] 2.19 画像・図形制御群の拡大/fit、open、copy操作がKDV内で副作用を起こさず、viewer commandとしてホストへ渡ることを確認する
- [ ] 2.20 KMM AST由来の見出し構造から目次viewを描画し、Markdown本文を再parseして目次正本を作っていないことを単体テストで確認する
- [ ] 2.21 目次item clickでKDVのプレビューがrendered heading anchorへscrollし、KMM node id、source range、heading anchorを含むviewer commandがホストへ通知されることを回帰テストで確認する
- [ ] 2.22 preview scroll時はlayout後のrendered heading anchor mapからactive headingを決定し、editor scroll時はKatanAが渡したKMM node idまたはsource rangeで目次highlightできることを統合テストで確認する

---

## 3. Export pipeline

- [ ] 3.1 HTML/PDF/PNG/JPG exportをKDV責務として定義する
- [ ] 3.2 viewer表示とexportが同じrender tree、KDR結果、`ViewerTheme`、`ViewerI18n`、`ViewerDiagnostics` を使い、KMM DTOを再parseしないことをテストで確認する
- [ ] 3.3 外部描画失敗時のraw code block、error border、代表メッセージがHTML/PDF/PNG/JPG exportで失われないことを形式別に確認する
- [ ] 3.4 KCF既存exportの維持期間、移譲条件、削除条件を文書化する

---

## 4. Final Verification & Release Work

- [ ] 4.1 `cargo fmt` / `cargo clippy --workspace -- -D warnings` / `cargo test --workspace` が通る
- [ ] 4.2 KDV AST lintを実行し、色literalとpreset直接参照の違反がないことを確認する
- [ ] 4.3 release tag `v0.1.0` を切り GitHub Release を作成する
- [ ] 4.4 KatanA が `katana-document-viewer = { git = "...", tag = "v0.1.0" }` でビルドできることを確認する
