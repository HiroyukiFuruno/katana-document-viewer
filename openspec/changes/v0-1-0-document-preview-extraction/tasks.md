# Tasks: katana-document-viewer v0.1.0

## Branch Rule

- **標準ブランチ**: `release/v0.1.0`
- **作業ブランチ**: `feature/v0.1.0-task-x`

---

## 1. neutral interface を確定する

- [ ] 1.1 `DocumentViewer` trait と DTO（`ViewerSource` / `ViewerConfig` / `ViewerTheme` / `ViewerI18n` / `ViewerOutput` / `ViewerDiagnostics` / `ExportConfig`）を本実装向けに拡張する
- [ ] 1.2 `katana-document-viewer` の `cargo tree` に `egui` が含まれないことを確認する
- [ ] 1.3 KatanA が interface crate のみを依存しても型エラーが出ないことを確認する
- [ ] 1.4 editor-viewer同期制御をKDVが持たず、KatanAがviewerまたはeditorへ命令する契約を明記する
- [ ] 1.5 `ViewerConfig` で `ViewerTheme` と `ViewerI18n` をnull不可の必須入力にし、KDV提供のdefault theme presetと英語（en）i18n presetを呼び出し側が明示引数として渡せるようにする

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
- [ ] 2.10 KMM canonical fixtures（`katana_sample.md` / `katana_sample_basic.md` / `katana_readme.md` / `description_list.md`）相当のviewer回帰テストを追加する
- [ ] 2.11 table alignment/width、GitHub alert、寛容なmath、Draw.io直接code block、`.drawio` / `.xml` 添付・参照先の先頭Draw.io判定とSVG化、ZenUMLのviewer回帰テストを追加する
- [ ] 2.12 外部描画失敗時にrawをcode block枠でそのまま表示し、枠borderをtheme由来のerror系カラーにし、preview上でエラーアイコン、代表メッセージ、tooltip詳細を表示する回帰テストを追加する
- [ ] 2.13 preview内の代表メッセージ、tooltip label、空状態など固定表示文言を `ViewerI18n` から取得し、KDV提供の英語（en）presetで表示できる回帰テストを追加する
- [ ] 2.14 KDV AST lintを拡張し、preset定義とtest fixtureを除くrendering code内のhard-coded color literalを禁止する

---

## 3. Export pipeline

- [ ] 3.1 HTML/PDF/PNG/JPG exportをKDV責務として定義する
- [ ] 3.2 viewer表示とexportが同じrender pipelineを使う方針を固定する
- [ ] 3.3 KCF既存exportの維持期間、移譲条件、削除条件を文書化する

---

## 4. v0.1.0 release

- [ ] 4.1 `cargo fmt` / `cargo clippy --workspace -- -D warnings` / `cargo test --workspace` が通る
- [ ] 4.2 release tag `v0.1.0` を切り GitHub Release を作成する
- [ ] 4.3 KatanA が `katana-document-viewer = { git = "...", tag = "v0.1.0" }` でビルドできることを確認する
