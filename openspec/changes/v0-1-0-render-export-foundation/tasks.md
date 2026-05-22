# Tasks: katana-document-viewer v0.1.0 render/export foundation

## Definition of Ready (DoR)

- [x] proposal / design / spec / tasks の対象capability名が一致している
- [x] KMM / KDV / KDR / KUC / KatanA の責務境界がdesign.mdとspec.mdで明示されている
- [x] `v0.1.0` で実装するUI非依存範囲と、`v0.2.0` へ送る画面操作範囲がtasks.mdから判定できる

## Definition of Done (DoD)

- [x] `npx -y @fission-ai/openspec validate v0-1-0-render-export-foundation --strict` が通る
- [x] `cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings` が通る
- [ ] `cargo test --workspace` が通る
- [x] `katana-document-viewer` のUI非依存dependency guardが通る
- [ ] CommonMark / GFM / 数式（math） / GitHub alert / KatanA互換fixtureのHTML出力UTが、記法ごとの成立を検証している
- [x] KMM DTO coverage gapを完了扱いせず、必須記法の未実装として検証されている
- [x] coverage matrix に `MissingImplementation` / `ExternalBackendRequired` が残っていない
- [x] HTML/PDF/PNG/JPG書き出し（export）のartifact manifestテストが、spec.mdのScenarioと対応している
- [x] HTML/PDF/PNG/JPG書き出し（export）の出力ファイルが0 byteではないことを検証している
- [x] export debug出力は `*.manifest.toml` sidecarを成果物フォルダへ残さない
- [x] KDR backend smokeとKDV export smokeが、成功時と未対応時の両方を検証している

## User Feedback Follow-up

- [/] FB-2026-05-20-001: KMM / KDR / KatanA を照合し、v0.1.0のMarkdown評価と書き出し（export）の責務境界を明確化する
- [/] FB-2026-05-20-002: Markdown標準、GitHub alert、KatanA独自解釈、数式（math）はraw sourceとdiagnosticsだけで完了扱いせず、全カバーをv0.1.0の完了条件にする
- [/] FB-2026-05-20-003: export debugの出力容量を検証し、0 byteを成功扱いしない。成果物フォルダへ `*.manifest.toml` sidecarを残さない
- [/] FB-2026-05-20-004: 素のHTML表示だけで完了扱いせず、標準Markdown、数式（math）、GitHub alert、KatanA独自解釈、KDR図形、コードブロックのHTML出力UTを記法ごとに追加する
- [/] FB-2026-05-20-005: ユーザーの細かな画面指摘に依存せず、Markdown標準、KDR図形、GitHub alert、数式、KatanA独自記法をTDDのREDとして先に失敗再現する
- [/] FB-2026-05-20-006: FB-2026-05-20-005のREDテストをGREEN化し、sample.ja.md出力でも同じ契約を検査する（KMM #3 / #4 待ちを含む）
- [/] FB-2026-05-21-001: イタリック、KatanA独自check、表、引用/alertの色、図形light theme、脚注末尾配置をREDテストで固定してからGREEN化する
- [/] FB-2026-05-21-002: 表の短列/長列レイアウトを実効CSS契約で固定し、コードブロックを言語タグ別のシンタックス強調で出力する
- [/] FB-2026-05-21-003: `sample.ja.md` をKDV repo内fixtureへコピーし、HTML出力テストとexport debugでrepo内fixtureを使う
- [/] FB-2026-05-21-004: アラインメント付き表のbody行が左寄せ/中央寄せ/右寄せに実表示されるCSS契約を追加する
- [/] FB-2026-05-21-005: ネスト引用を深さ固定にせず、blockquoteを段階的な左線とインデントで出力する
- [/] FB-2026-05-21-006: `<details>` / `<summary>` のアコーディオン内Markdownを、既存記法どおりリストやコードとして描画する
- [/] FB-2026-05-21-007: 数式はraw textではなく、Rust数式rendererに委譲してHTML exportへ描画済みSVGとして埋め込む
- [/] FB-2026-05-21-008: PlantUML描画はKDR v0.2.0のdirect rendererを取り込み、KDV側の外部backend待ち扱いを解除する
- [/] FB-2026-05-21-009: table cell内のMarkdown inline記法を有効にし、短列幅を日本語が不自然に縦割れしない値へ調整する
- [/] FB-2026-05-21-010: 脚注参照を標準的な `[1]` / `[2]` リンク表記にし、下線付きリンクとして表示する
- [/] FB-2026-05-21-011: KatanA独自task marker `[-]` / `[/]` をネイティブcheckboxに潰さず、横棒/斜線の見えるmarkerとして出力する
- [/] FB-2026-05-21-012: コードブロックで分断されたordered listも、Markdown source上の番号を `<ol start>` として維持する
- [/] FB-2026-05-21-013: ネスト引用は深さをHTML属性に残し、KatanA側の段階的な左線表示へ寄せられるCSS契約を持つ
- [/] FB-2026-05-21-014: GitHub alert / legacy note のタイトル行に、Note / Tip / Warning などの種別アイコンを出す
- [/] FB-2026-05-21-015: HTML entity は二重escapeせず、KMM textを1回decodeしてからHTMLとしてescapeする
- [/] FB-2026-05-21-016: 旧 `> **Type**` 形式のNoteブロックは通常引用として装飾せず、GFM `[!TYPE]` alertだけ種別ごとのアイコンを出す
- [/] FB-2026-05-21-017: 旧 `> **Type**` 形式のNoteブロックは、Typeと本文を同じ引用行に横並びで表示する
- [/] FB-2026-05-21-018: 旧Noteブロックの説明は、見出し内の `> **Type**` をinline codeとして表示する
- [/] FB-2026-05-21-019: KDR未取り込み分を除き、KDV側のHTML export contract gapを残さない
- [/] FB-2026-05-21-020: PDF/PNG/JPG export payloadはPlaywrightで代替せず、Rust側の正式描画backendで実装する
- [/] FB-2026-05-21-021: KatanA fixtureのdata URI SVGと `.drawio` / `.xml` 参照をKDV側完了条件へ含める
- [/] FB-2026-05-21-022: PDF/PNG/JPGのRust描画backendでOS依存の絵文字を空白にせず、multi platform font fallback対応の描画ライブラリで出力する
- [/] FB-2026-05-21-023: KDR v0.2.0へPlantUMLを接続し、上位からKDVへ渡された完全テーマ（theme）をKDR `RenderContext.theme` として渡す
- [/] FB-2026-05-21-024: PDF/PNG/JPG surfaceで図形SVGをraw text化せず、Rust側でrasterizeしてHTML同等の図形blockとして出力する
- [/] FB-2026-05-21-025: PDF/PNG/JPG surfaceでcode blockを本文行ではなく背景/枠付きblockとして出力する
- [/] FB-2026-05-22-001: HTML→PDF丸投げスパイク（hyper-render / fulgur）は図形欠落、図形消失、数式raw化、将来の注釈/改ページ制御不足により不採用とし、KDV native surface/pagination approachへ戻す
- [/] FB-2026-05-22-002: PDF exportはnative surface全体を巨大1ページ画像にせず、block境界で複数ページへ分割して出力する
- [/] FB-2026-05-22-003: PDF/PNG/JPG native surfaceで数式をraw TeXのまま出さず、表示用テキストへ正規化する
- [/] FB-2026-05-22-004: PDF/PNG/JPG native surfaceでMarkdown inline semantics（太字、斜体、inline code、混在装飾）を失わずに保持する
- [/] FB-2026-05-22-005: PDF/PNG/JPG native surfaceでcode blockを単一blockとして描画し、言語タグごとのsyntax highlightを色付きspanとして保持する
- [/] FB-2026-05-22-006: PDF/PNG/JPG native surfaceで図形SVGを過剰拡大せず、上限幅を持つrasterized diagramとして描画する
- [/] FB-2026-05-22-007: PDF/PNG/JPG native surfaceでaccordion本文を閉じたraw HTMLではなく、開いた状態の本文として展開する
- [/] FB-2026-05-22-008: PDF/PNG/JPG native surfaceで絵文字列を要素ごとに分断せず、同一行のtext runとして描画する
- [/] FB-2026-05-22-009: PDF/PNG/JPG native surfaceでtableをraw pipe textではなく、罫線・header背景・偶数行背景付きtable blockとして描画する
- [ ] FB-2026-05-22-010: HTML/PDF/PNG/JPGの出力を14件の指摘で再検証し、HTMLで成立しているKMM構文木（AST）評価をPDF/PNG/JPGでも同じ意味で使う
  - [ ] 1. native surfaceの不要な `KDV export: ...` ファイル名見出しを出さない
  - [ ] 2. Markdown記法をraw textではなく構文木のinline/block意味として描画する
  - [ ] 3. code block同士が密着しない余白を持つ
  - [ ] 4. raw由来の不要な `:` / `..` / markerを出さない
  - [ ] 5. 長文が右端へはみ出さない
  - [ ] 6. table/code/blockquote内の文字が重ならない
  - [ ] 7. legacy Note記法とGFM alert記法を区別して描画する
  - [ ] 8. accordionは静的exportで開いた本文を出す
  - [ ] 9. 数式は小さすぎない表示サイズにする
  - [ ] 10. 不要な空白ページ/余白を出さない
  - [ ] 11. PlantUML/Draw.ioだけ過剰拡大しない
  - [ ] 12. HTML entityと絵文字列を同一行の評価済みtextとして描画する
  - [ ] 13. 脚注参照と脚注本文を出す
  - [ ] 14. horizontal ruleは `---------` textではなく線として描画する
  - [ ] 15. `English | 日本語` の説明文をtable扱いせず、リンク行そのものを中央揃え同一行で表示する
- [ ] FB-2026-05-22-011: PDF/PNG/JPG native surfaceのinline decorationをHTML版へ寄せ、通常下線・リンク下線・脚注リンク下線・inline code背景・mark背景の位置と幅を自動検査する
- [ ] FB-2026-05-22-012: PDF/PNG/JPG native surfaceのリスト/チェックボックスをHTML版へ寄せ、marker列、本文インデント、KatanA `[-]` / `[/]` の見た目を自動検査する
- [ ] FB-2026-05-22-013: PDF/PNG/JPG native surfaceのtable(grid)をHTML版へ寄せ、上下padding、縦中央揃え、右端padding、header/even row配色を自動検査する
- [/] FB-2026-05-22-014: PDFの内部リンクは外部URI扱いにせず、脚注参照と脚注下部backlinkを同一ドキュメント内遷移として出力する
- [ ] FB-2026-05-22-015: PDF/PNG/JPG native surfaceのbadgeをHTML版の視覚表現とリンク可能領域へ寄せる
- [ ] FB-2026-05-22-016: PDF/PNG/JPG native surfaceの数式はHTML版と同じRust数式rendererのSVG出力を使い、差異が残る場合は同等性を自動検査できる契約へ更新する
- [/] FB-2026-05-22-017: PDF/PNG/JPG native surfaceの空コードブロックは水平線状に潰さず、HTML版と同等にコード領域として認識できる最小高さを持たせる
- [/] FB-2026-05-22-018: PDF/PNG/JPG native surfaceのリンク下線は推定幅ではなく実際の文字配置範囲に追従し、URL、`English`、脚注参照、脚注backlinkで前後文字へ食い込まない
- [ ] FB-2026-05-22-019: `src` 直下に増えすぎたexport関連moduleを、5ファイルを超えるまとまりごとにディレクトリ管理へ移す
- [ ] FB-2026-05-22-020: PDF/PNG/JPG native surfaceの脚注定義はHTML版と同じく番号付き項目として表示し、戻りリンク `↩` は脚注本文の末尾へ置く
- [ ] FB-2026-05-22-021: 数式はHTMLとPDF/PNG/JPGで別々に解釈せず、同じRust数式rendererが生成したSVGをHTMLへ埋め込み、native surfaceでは同じSVGをrasterizeして描画する
- [ ] FB-2026-05-22-022: export全体はHTML用/Surface用でAST評価を二重実装せず、KMM構文木（AST）から共通の描画中間表現を作り、出力先差分は最終描画adapterに閉じ込める
- [/] FB-2026-05-23-001: KRR公開前でもKDV側で `katana-render-runtime-stub` 境界を追加し、数式SVG生成の呼び出し元をHTML/PDF surfaceで共通化する。stubはMarkdown ASTを解析せず、受け取ったTeX文字列のSVG化またはraw返却だけを担当する

---

## Branch Rule

- **標準ブランチ**: `release/v0.1.0`
- **作業ブランチ**: `feature/v0.1.0-task-x`

---

## 1. v0.1.0 / v0.2.0 の境界を確定する

- [x] 1.1 `v0.1.0` はKUC非依存のrender/export foundationと明記する
- [x] 1.2 hover、選択、目次（TOC）、画像・図形操作、KUC viewerを `v0.2.0` へ送る
- [x] 1.3 `adopt-kme-preview-model` との重複を確認し、必要な内容をこのchangeまたは `v0.2.0` へ移す
- [x] 1.4 `openspec/project.md` と `docs/ui-separation-plan.md` のversion順序を同期する

---

## 2. neutral interface と document model を定義する

- [x] 2.1 `crates/katana-document-viewer` を追加する
- [x] 2.2 `DocumentSource` / `SourceUri` / `SourceKind` / `SourceRevision` を定義する
- [x] 2.3 `DocumentId` / `DocumentKind` / `DocumentSnapshot` / `DocumentOutline` / `DocumentMetadataView` を定義する
- [x] 2.4 KMM input conversion を追加する
- [x] 2.5 KMM parse result conversion を追加する
- [x] 2.6 source serialization test を追加する
- [x] 2.7 document snapshot test を追加する

---

## 3. artifact model を定義する

- [x] 3.1 `ArtifactId` / `ArtifactKind` / `ArtifactFormat` を定義する
- [x] 3.2 `ArtifactBytes` / `ArtifactUri` / `ArtifactManifest` を定義する
- [x] 3.3 `ArtifactDiagnostics` を定義する
- [x] 3.4 preview artifact を定義する
- [x] 3.5 export artifact を定義する
- [x] 3.6 image artifact を定義する
- [x] 3.7 PDF artifact を定義する
- [x] 3.8 Office artifact placeholder を定義する
- [x] 3.9 artifact manifest serialization test を追加する

---

## 4. forge / export API を定義する

- [x] 4.1 `forge` module を作る
- [x] 4.2 `BuildRequest` / `BuildProfile` / `BuildGraph` / `TransformStep` を定義する
- [x] 4.3 `ExportRequest` / `ExportFormat` / `ExportOutput` を定義する
- [x] 4.4 `ForgeDiagnostics` / `ForgeError` を定義する
- [x] 4.5 `ForgeBackend` trait と `ForgePipeline` を定義する
- [x] 4.6 no-UI dependency test を追加する

---

## 5. KDR backend integration とKDV export contractを作る

- [x] 5.1 `backend::diagram` module を作る
- [x] 5.2 KDR `RenderInput` への変換を作る
- [x] 5.3 KDR `RenderOutput` から `Artifact` への変換を作る
- [x] 5.4 KDV export output から `ExportOutput` への変換を作る
- [x] 5.5 Mermaid / Draw.io / PlantUML / ZenUML / math の委譲境界を定義する
- [x] 5.6 HTML / PDF / PNG / JPEG export path を接続する
- [x] 5.7 削除予定crateへ依存しないことをREADMEに記載する
- [x] 5.8 KDR compatibility tests を追加する
- [x] 5.9 KDR direct pathはMermaid / Draw.io / PlantUML、ZenUMLはMermaid互換、KDV exportはmanifest contract、mathはKDV内Rust数式renderer委譲というcapability tableを追加する

---

## 6. rendering evaluation fixtures を作る

- [x] 6.1 CommonMark fixture set を作る
- [x] 6.2 GFM fixture set を作る
- [x] 6.3 数式（math）fixture set を作る
- [x] 6.4 GitHub alert fixture set を作る
- [x] 6.5 KatanA互換fixture set を作る
- [x] 6.6 外部描画成功fixture set を作る
- [x] 6.7 外部描画失敗時のraw保持fixture set を作る
- [x] 6.8 fixtureとspec Scenarioの対応表を追加する
- [x] 6.9 KMM DTO化済み / 未実装 / 外部backend待ちを分けたCommonMark / GFM coverage matrixを追加する
- [x] 6.10 coverage matrix の `MissingImplementation` / `ExternalBackendRequired` を0にする

---

## 7. CLI API と品質ゲートを作る

- [x] 7.1 `cli_api` module を作る
- [x] 7.2 `CliRequest` / `CliOutput` / `CliDiagnostics` を定義する
- [x] 7.3 markdown preview build CLI entry を作る
- [x] 7.4 export CLI entry を作る
- [x] 7.5 diagram render CLI entry を作る
- [x] 7.6 export-debug CLI entry を作る
- [x] 7.7 KDV `just check` にforge no-UI dependency guardを追加する
- [x] 7.8 KDV `just check` にKDR backend smokeを追加する
- [x] 7.9 KDV `just check` にCLI API smokeを追加する
- [x] 7.10 artifact fixture tests とexport fixture testsをCIへ追加する
- [x] 7.11 export output の0 byte拒否テストを追加する
- [x] 7.12 export debugの成果物フォルダから `*.manifest.toml` sidecarをなくす
- [x] 7.13 CommonMark inline decoration のHTML出力UTを追加する
- [x] 7.14 fenced code block のHTML出力UTを追加する
- [x] 7.15 GitHub alert のHTML出力UTを追加する
- [x] 7.16 数式（math）のHTML出力UTを追加する
- [x] 7.17 KatanA独自HTML / image / badge のHTML出力UTを追加する
- [x] 7.18 KDR Mermaid / Draw.io SVG埋め込みのHTML出力UTを追加する
- [x] 7.19 KMM公開データ型不足を `katana-markdown-model` issueへ切り出す（https://github.com/HiroyukiFuruno/katana-markdown-model/issues/1）

---

## 8. Final Verification & Release Work

- [ ] 8.1 `cargo fmt` / `cargo clippy --workspace -- -D warnings` / `cargo test --workspace` が通る
- [x] 8.2 KDV AST lintを実行し、UI非依存境界違反がないことを確認する
- [x] 8.3 `npx -y @fission-ai/openspec validate v0-1-0-render-export-foundation --strict` が通る
- [ ] 8.4 release tag `v0.1.0` を切り GitHub Release を作成する
- [ ] 8.5 KatanA が `katana-document-viewer = { git = "...", tag = "v0.1.0" }` でビルドできることを確認する
