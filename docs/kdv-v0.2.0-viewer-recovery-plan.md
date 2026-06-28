# KDV v0.2.0 Viewer Recovery 乖離分析・詳細設計

## 結論

KDV v0.2.0 は現時点で DoD 未達である。

未達の主因は、KatanA の Preview が持っている「解析結果」「描画結果」「操作契約」「遅延描画」「検証スコア」を、KDV engine と KUC 共通 UI 契約へ分解し切れていないことにある。

この文書は実装指示ではなく、KatanA 参照仕様に対する現状の乖離を一覧化し、DoD を満たすために必要な設計、証拠、検証ゲートを固定するための文書である。

## 目的

- KatanA の document viewer を v0.2.0 の参照仕様として固定する。
- KDV が担当する viewer engine の責務を明確にする。
- KUC が担当する共通 UI 部品と操作契約を明確にする。
- Storybook が独自 UI 実装ではなく、KUC 実部品を操作する検証画面であることを固定する。
- KatanA export HTML/PDF/screenshots と比較して、visual / semantic / interaction / performance の全カテゴリ 95 点以上を DoD とする。

## 非目的

- この文書作成の時点では実装しない。
- egui / gpui / floem adapter を復帰しない。
- Storybook / KUC 側に独自 TreeView、独自 settings UI、独自 media control、独自 hit-test を追加しない。
- KDV core は viewer 専用の MediaControl specification / command を所有してよい。ただし KUC 汎用 widget へ昇格しない。
- KatanA 固有 namespace を KUC public API に入れない。
- KDV 自己比較だけで score を通さない。

## ユーザー実機指摘の台帳

最新の未対応指摘は `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/user-feedback-todo.md` を正本にする。

この recovery plan は設計と DoD の正本であり、実機確認で出た個別不備の消し込みは user feedback TODO 台帳で管理する。台帳に `[ ]` が残る間、v0.2.0 viewer parity は完了扱いにしない。

## KUC/KDV 境界

結論として、KDV 側に UI adapter は置かない。

KDV は document viewer engine であり、UI 表現を所有しない。KDV が生成してよいものは、source 解析結果、viewer section、viewer node、asset request、search / slideshow / task などの viewer state、viewer command intent までである。`UiNode`、TreeView、Toggle、Button、Linkable、hover、cursor、hit rect、Retina/HiDPI 補正、window input 正規化、canvas 描画は KDV の責務ではない。

KUC は UI core であり、KDV の viewer model を表示可能な KUC render model へ射影する責務を持つ。KDV 向けの projection / host contract が必要な場合も、所有場所は KUC 側、または KUC が提供する viewer projection API である。KDV repo 内の `katana-document-viewer-kuc` のような adapter crate は現在の過渡的実装であり、DoD 完了時点では KDV 側の UI adapter として残してはいけない。

Storybook は KUC host を起動して KDV engine の出力を渡す検証画面である。Storybook は UI 部品、hit-test、action 合成、style class からの action 復元、KUC への見た目補正を実装しない。

KUC 汎用契約は、操作可能 UI の `cursor`、hover border / background、focus / active、click callback dispatch などの interactive preset / mixin までに限定する。

KDV viewer 専用契約は、Markdown / HTML / PDF / image / diagram の viewer model、図形描画、viewer media controller、diagram copy source、pan / zoom / reset、KatanA fixture parity とする。

MediaControl は KDV/KatanA viewer 専用 widget として扱う。KUC 汎用部品ではない。MediaControl は KUC の汎用 interactive preset を利用してよいが、図形描画や viewer controller の配置仕様を KUC 汎用 API へ混ぜない。

KUC public API は `Diagram`、`Image`、`Code` のような viewer media 種別を汎用 UI 契約として持たない。KUC は汎用 host action と hit rect を返す。viewer 固有 command への変換は、KDV engine が定義する command intent と KUC viewer projection が返す action を照合する薄い host layer が担う。KDV core も KDV Storybook も、座標や style class から `Fullscreen`、`CopySource`、`Pan`、`Zoom`、`Reset` を復元しない。

TreeView が file explorer 表示の正本である。`FileTree` を使う場合は、path list を `TreeView` nodes へ変換する薄い facade に限定する。`FileTree` は別の見た目、別の row layout、別の hit-test、別の selection contract を持つ widget ではない。

KUC の generic preset は操作可能 UI の標準挙動であり、利用側が cursor、hover border、hover background、click dispatch を通常は指定しない。KDV が設定するのは「この node は Button / Toggle / Link / TreeView row / SettingsList field として操作可能」という汎用情報までであり、部品ごとの cursor や hover を再定義しない。

### 境界判定ルール

新しい部品、action、state を追加する前に、次の順で所有者を判定する。

1. どの業務画面でも使える操作 UI の挙動か。
   - 該当するなら KUC が所有する。
   - 例: pointer cursor、hover border、focus、active、disabled、click dispatch、link span hit、TreeView row hit、SettingsList field hit。
2. document viewer の source、node、section、media、score、renderer に依存するか。
   - 該当するなら KDV が所有する。
   - 例: Markdown task marker、diagram/image/PDF asset request、viewer media control command、search jump、bottom spacer、KatanA fixture score。
3. vendor の描画 API や window event API に依存するか。
   - 該当するなら将来の vendor adapter が所有する。
   - KDV core と KUC core には入れない。
4. Storybook だけで必要なものか。
   - Storybook は host state と検証だけを持つ。
   - UI 部品、hit-test、action 生成、viewer command 定義は Storybook に置かない。

迷った場合は KUC に汎用化しない。まず KDV viewer 専用として置き、複数ドメインで同じ挙動が必要になった時だけ KUC の generic preset へ上げる。

### Generic Preset と Viewer Widget の分離

KUC に置くもの:

- `UiInteractivePreset` 相当の操作可能 UI の標準挙動。
- Button / Toggle / Checkbox / Radio / Select / Link / TreeView row / SettingsList field の hover、cursor、focus、click dispatch。
- KUC renderer と同じ座標計算に基づく hit rect と cursor。
- 汎用 host action id と payload の保持。
- `FileTree` facade を残す場合は、`TreeView` の model 生成と action 変換だけを担当する。

KDV に置くもの:

- MediaControl の右上/右下配置。
- fullscreen / copy source / copy rendered / pan / zoom / reset / info の command 定義。
- diagram / image / code copy の target 解決。
- Markdown task marker と source mutation。
- search、slideshow、bottom spacer、score gate。

KUC viewer projection / host contract に置くもの:

- KDV viewer model を KUC `UiNode` へ写す変換。
- KDV core が定義した `ViewerMediaControlSpec` を KUC Button / Row / Column へ変換する薄い処理。
- KDV core が選んだ viewer 専用 node を、KUC generic primitive / preset へ割り当てる mapping。

KDV repo 内に置いてはいけないもの:

- KDV 側の UI adapter crate。
- KDV 側の KUC renderer / KUC host action / KUC hit-test wrapper。
- KDV 側の `UiNode` 構築ロジック。ただし DoD までの移行期間に残る既存 crate は、削除または KUC 側へ移設する対象として扱う。
- `viewer.image.` / `viewer.diagram.` / `viewer.code.` prefix の直書き。
- `fullscreen` / `pan-up` / `zoom-in` / `copy-source` など command literal の再定義。
- `cursor(UiCursor::Pointer)` など KUC generic preset の個別上書き。
- `kdv-diagram-*` / `kdv-image-control` / `kdv-code-control` を MediaControl の描画・hit-test・action 契約にすること。
- KUC renderer と別計算の row height、button rect、link span rect。
- `FileTree` を `TreeView` と別仕様の widget として扱うこと。

### KUC への昇格判定

KUC に置くかどうかは「複数画面で使えそう」では判断しない。KUC に昇格してよいのは、viewer source、document section、diagram/image/PDF、KatanA fixture、score gate を一切知らなくても成立する UI 契約だけである。

昇格してよいもの:

- pointer cursor、hover border、hover background、focus ring、disabled 表現。
- click / context menu / keyboard focus の host action dispatch。
- Button、Toggle、Checkbox、Radio、Select、Link span、TreeView row、SettingsList field の標準操作。
- ScrollArea、Stack、Row、Column、absolute overlay、margin、gap の汎用 layout。
- FileTree を残す場合の path list から TreeView nodes への変換 helper。ただし描画と hit-test は TreeView そのものを使う。

昇格してはいけないもの:

- MediaControl、DiagramControl、ImageControl、CodeCopyControl のような viewer 専用 widget。
- pan / zoom / reset / fullscreen / copy source / copy rendered の command 定義。
- `viewer.image.*`、`viewer.diagram.*`、`viewer.code.*`、`ui.diagram.*`、`diagram.zoom` のような viewer 操作に見える action id。
- KatanA fixture、KatanA export HTML/PDF/screenshot、score category、document source span。
- Markdown task marker の `[ ]` / `[x]` / `[/]` / `[-]` の source mutation。

KDV 専用 widget は、KUC 汎用部品を合成して作る。

例:

- KDV `ViewerMediaControlSet` が button の command、slot、size、配置を決める。
- KUC viewer projection はそれを KUC `Button`、`Stack`、absolute overlay、host action へ写す。
- KUC は button と overlay の描画、hover、cursor、hit rect だけを担当する。
- Storybook は KUC が返した host action hit を KDV `ViewerMediaControlAction` へ渡して state / command を反映する。
- MediaControl の transparent base、hover、click は `UiVariant::Icon`、`UiVisualRole::MediaFrame`、absolute overlay、host action hit rect で証明する。`kdv-diagram-*` などの class 名で証明しない。

この流れを逆にして、KUC に MediaControl API を置く、KDV repo 内 adapter が command literal を再定義する、Storybook が style class から action を復元する、KDV repo 内 adapter が media control style class を契約として再導入する、または KDV repo 内に UI adapter を増やす実装は不合格とする。

## DoD

`just storybook` は KUC 実部品で構成された Katana 由来 viewer を interactive 起動する。

次の全カテゴリが 95 点以上でなければ未完了とする。

```text
final_score = min(
  visual_score,
  semantic_score,
  interaction_score,
  performance_score
)
```

合格条件:

- KatanA fixture が KDV assets に固定されている。
- KatanA reference HTML/PDF/screenshots が KDV assets に固定されている。
- Markdown が raw 表示ではなく node 表示される。
- HTML alignment、table、alert、code block、horizontal rule、list、task、footnote、accordion、link が表示と操作の両方で機能する。
- direct image / drawio / mermaid / plantuml / pdf / html が source kind として扱える。
- direct image は png / jpg / jpeg / svg / webp / bmp / gif を扱える。
- diagram/image/PDF は lazy / parallel / cancellation / cache / theme 対応である。
- search highlight と jump が動く。
- scroll / resize / bottom spacer が動く。
- Storybook の左上は KUC TreeView、左下は KUC SettingsList、右側は KUC viewer である。
- Storybook に manual hit-test、manual action synthesis、style class parse による action 判定がない。
- open GitHub issue がDoD項目へ接続されている。特に #7 の日本語長文PDF/PNG/JPEG surface重なりと、#6 のdirect visual source exportは、issue単位の回帰テストとscore gate証跡がなければ未完了とする。

## 参照仕様

KatanA は reference implementation であり、依存先ではない。

参照対象:

| 領域 | 参照ファイル |
|---|---|
| section 分解 | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-core/src/preview/section/mod.rs` |
| local image 分解 | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-core/src/preview/section/local_image.rs` |
| math 前処理 | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-core/src/preview/section/math.rs` |
| diagram AST | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-core/src/preview/diagram_ast.rs` |
| section 型 | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-core/src/preview/types.rs` |
| Markdown hooks | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/section_show/markdown/mod.rs` |
| lazy / pending / worker | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/core_render.rs` |
| render polling | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/background.rs` |
| render worker | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/render_workers.rs` |
| diagram cache | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/diagram_cache/` |
| media controls | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/diagram_controller.rs` |
| fullscreen | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/fullscreen*.rs` |
| slideshow | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/slideshow/` |
| task checkbox / context menu | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/widgets/markdown_hooks/` |
| table | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/section_show/markdown/extension_table/renderer.rs` |
| HTML | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/html.rs` |
| emoji | `/Users/hiroyuki_furuno/works/private/katana/crates/katana-core/src/emoji/raster/` |

## KatanA Preview の参照挙動

### Source / Section

KatanA は Markdown 全体をそのまま UI に渡しているのではない。

Preview section として次を分解する。

- Markdown section
- Diagram section
- Local image section
- HTML block
- footnote definition
- heading / anchor / outline
- source line と section line の対応

要求:

- KDV は source text から `ViewerDocument` を作る。
- 全 section は `section_id`、`source_span`、`line_start`、`line_count` を持つ。
- diagram fence と通常 code fence を混同しない。
- footnote は section をまたいでも意味が保たれる。
- 失敗時は fallback renderer に流さず、diagnostic node と raw string node を返す。

### Markdown

KatanA Markdown は CommonMark 表示だけではない。

参照挙動:

- heading anchor
- block anchor
- active line highlight
- hover 時の block/list item 全体 highlight
- search match highlight
- search active index jump
- footnote rendering
- code copy button
- table renderer
- HTML block renderer
- math renderer
- task checkbox
- task context menu
- shortcut / custom inline text
- OS color emoji

KDV/KUC 側で必要な node/action:

- `Heading`
- `Paragraph`
- `Text`
- `Link`
- `CodeBlock`
- `Table`
- `List`
- `ListItem`
- `TaskCheckbox`
- `Alert`
- `BlockQuote`
- `HorizontalRule`
- `Footnote`
- `Accordion`
- `HtmlBlock`
- `Math`

### HTML

KatanA HTML は raw text 表示ではない。

参照挙動:

- `align="center"` / `align="right"` を反映する。
- link は見た目だけでなく click action を持つ。
- hover 時に cursor が変わる。
- badge row や inline image が後続テキストと重ならない。
- HTML block の後続要素に margin が確保される。
- 画像 path は source file 基準で解決される。

要求:

- KDV は HTML block を node として保持する。
- KUC は alignment / link span / image / spacing を実表示に反映する。
- HTML の結果は KatanA export HTML/PDF と視覚比較する。

### Table

KatanA table は幅の暴走を抑制する。

参照挙動:

- header fill
- separator
- body row
- column alignment
- wide table の horizontal scroll
- viewport 幅に応じた safe width
- table ごとの stable id

要求:

- table を raw markdown として表示しない。
- KUC viewer 上で table node として描画する。
- score は text preservation だけでなく layout bounds を見る。

### Task Checkbox

KatanA task marker は GitHub の `[ ]` / `[x]` だけではない。

参照挙動:

| marker | 意味 |
|---|---|
| `[ ]` | 未実施 |
| `[x]` / `[X]` | 完了 |
| `[/]` | 実施中 |
| `[-]` / `[~]` | 保留または進行系 marker |

操作:

- checkbox click で marker が切り替わる。
- checkbox と本文の間も click 可能領域になる。
- hover cursor が変わる。
- 右クリック context menu で marker を変更できる。
- list item 全体の hover highlight が出る。

要求:

- KDV は marker と source_span を保持する。
- KUC は task control action を返す。
- KDV は座標や文字列から task action を復元しない。

### Alert / BlockQuote

Alert 記法は GitHub 由来の記法を参照する。

要求:

- `> [!NOTE]`
- `> [!TIP]`
- `> [!IMPORTANT]`
- `> [!WARNING]`
- `> [!CAUTION]`

を blockquote の raw text ではなく alert node として扱う。

KatanA 表示との差分は visual / semantic の両方で評価する。

### Diagram / Image / PDF

KatanA は diagram を同期的に全部待つ viewer ではない。

参照挙動:

- diagram section を抽出する。
- pending を先に表示する。
- render worker へ送る。
- generation を使って古い job を無視する。
- render result を polling で取り込む。
- image preload を行う。
- force render 時に cache/viewer state/fullscreen を reset する。
- dark theme を render request に含める。

要求:

- first frame は 0.6 秒以内。
- pending 表示は 0.2 秒以内。
- diagram render は UI thread を止めない。
- file switch 時は stale job を cancellation する。
- cache key は `kind + source_hash + theme + relevant options`。
- 失敗時は diagnostic node と raw string node を返す。

### Direct Source

KatanA の viewer は Markdown 内の media だけでなく、visual source 自体を直接開ける。

KDV v0.2.0 で固定する direct source:

| 種別 | 拡張子 |
|---|---|
| Markdown/text | `md`, `markdown`, `txt` |
| HTML | `html`, `htm` |
| Image | `png`, `jpg`, `jpeg`, `svg`, `webp`, `bmp`, `gif` |
| Draw.io | `drawio`, `drowio` |
| Mermaid | `mermaid`, `mmd` |
| PlantUML | `plantuml`, `puml` |
| PDF | `pdf` |

要求:

- direct source は Markdown fallback で誤魔化さない。
- source kind ごとに `ViewerDocument.source_kind` を持つ。
- image / diagram / PDF は viewer node と asset request に変換する。
- unsupported または render failure は diagnostic node と raw string node を返す。

### Media Controls

図形・画像の controller は見た目だけのボタンではない。

参照挙動:

- 右上に fullscreen。
- 右下に pan / zoom / reset / info 系 control。
- button fill は transparent。
- hover/click 可能である。
- diagram source copy と rendered copy が必要。

要求:

- KUC は汎用 host action と hit rect を返す。
- KDV が viewer model に基づき `ViewerMediaControlAction` を解決する。
- KDV は固定矩形や style class を parse して action を決めない。
- Storybook では実操作で fullscreen / pan / zoom / reset / copy を検証する。

### Search / Anchor / Scroll

KatanA viewer は検索・目次ジャンプ・行ジャンプを持つ。

要求:

- search match highlight。
- next / previous jump。
- active search index 表示。
- anchor jump。
- bottom spacer。
- vertical scroll。
- resize 追従。

bottom spacer は、末尾付近へのジャンプが viewport の都合で機能しない問題を避けるため必須である。

### Viewer Mode / Slideshow

KatanA viewer は単一 SPA ではなく、viewer instance として状態を持つ。

要求:

- document mode と slideshow mode を切り替えられる。
- slideshow mode は現在 page / total page を state として表示する。
- next / previous / close / toggle の key operation を持つ。
- mode 切替時に scroll と active page が破綻しない。
- Storybook settings から mode を変更し、右側 viewer が即時反映する。

### Theme / Font / Emoji

KDV は egui の文字描画制約へ引きずられない。

要求:

- render logic は vendor 非依存。
- KUC 契約に従って OS 由来の color emoji を表示する。
- dark theme を text / table / alert / diagram / media control へ pass-through する。
- Storybook の theme toggle は見た目だけでなく render request と score に反映される。

## 責務分離

### KDV の責務

KDV は viewer engine を担当する。

- source normalization
- fixture/source kind 判定
- KatanA 互換 section pipeline
- source_span / line mapping
- `ViewerDocument` / `ViewerSection` / `ViewerNode`
- diagram/image/PDF asset scheduler
- search model
- anchor / bottom spacer model
- score gate
- KUC へ渡す viewer model

KDV が持ってよいのは document viewer 固有の意味論である。

### KUC の責務

KUC は vendor 非依存の UI 部品契約を担当する。

- TreeView
- FileTree facade
- SettingsList
- Button / Toggle / Checkbox / Radio / Select などの汎用 control primitive
- ContextMenu / Popover などの汎用 action surface
- TextSpanAction
- document 表示に再利用できる汎用 layout primitive
- theme / font / emoji 表示 primitive
- hit-test
- selection
- hover
- interactive preset / mixin
- action emission

KUC は Katana 専用 API にしてはいけない。

KUC は viewer 固有の意味論を持たない。特に `Diagram`、`Image`、`PdfPage`、`ViewerMediaControlAction`、`pan`、`zoom`、`fullscreen`、`copy source`、Markdown task marker の source mutation は KUC の責務ではない。

`FileTree` は KUC が提供してよいが、所有するのは `TreeView` への変換と `TreeViewAction` から `FileTreeAction` への変換だけである。表示、row height、indent line、folder/file icon、hover/selection、scroll-aware hit-test は `TreeView` の契約をそのまま使う。

### Storybook の責務

Storybook は host である。

- KUC TreeView を表示する。
- KUC SettingsList を表示する。
- KUC viewer を表示する。
- KUC から返った action を state に反映する。
- screenshot / interaction / performance / score を検証する。

Storybook は UI 部品を独自実装しない。

### 境界所有マトリクス

| 概念 | 所有者 | KUC に許可 | KDV に許可 | 禁止 |
|---|---|---|---|---|
| TreeView / FileTree | KUC | TreeView の row layout、folder/file icon、indent line、selected/hover state、scroll-aware hit-test。FileTree は TreeView facade に限定する | fixture tree を KUC TreeNode または FileTreeItem へ渡し、返却 action を state に反映する | KDV が row height / indent / visible order を再計算する。FileTree が TreeView と別の見た目/操作契約を持つ |
| SettingsList | KUC | section、field、Toggle、Select、hover、hit-test、action emission | viewer setting model と action result の反映 | KDV が field id から次状態を独自計算する |
| 汎用操作 UI | KUC | cursor、hover border/background、active/focus、callback dispatch の preset / mixin | KUC preset を使う node を生成する | KDV が Button / Toggle / Link ごとに cursor や hover を個別補正する |
| Link / Text span | KUC | span-level target、cursor、hover、host action | Markdown link target を span として渡す | KDV が表示座標や style class から link action を復元する |
| Host action hit rect | KUC | node/action/cursor と同じ描画契約で hit rect を返す | 返却 action を viewer command へ変換する | KDV が link 行高、accordion header 高さ、style class、state id から操作対象を復元する |
| Markdown task | KDV | 汎用 Checkbox / ContextMenu / Radio と host action | `[ ]` `[x]` `[/]` `[-]` の marker model、source_span、source mutation command | KUC が Katana/KDV task marker を public API として所有する |
| MediaControl | KDV core | 汎用 Button primitive、host action、hit rect、interactive preset | 右上/右下配置、diagram/image/code 対象、pan/zoom/reset/fullscreen/copy source/copy rendered、control command / label / size | KUC が `Diagram` / `Image` / `Code` の viewer action enum を持つ、または KDV repo 内 adapter が control command literal を定義する |
| Viewer media action id | KDV core | 汎用 host action id を保持するだけ | `ViewerMediaControlAction` が action id の生成と解析を所有する | KDV repo 内 adapter / Storybook が `viewer.image.` / `viewer.diagram.` / `viewer.code.` prefix を直書きする |
| Diagram / Image / PDF rendering | KDV | 表示 primitive と theme/font/emoji primitive | asset scheduler、lazy/parallel/cancel/cache、theme request、failure diagnostic | KUC が図形レンダリングや renderer option を所有する |
| Score gate | KDV | なし | KatanA reference HTML/PDF/screenshot との差分評価 | KDV Storybook 自己比較だけで合格にする |
| Vendor adapter | adapter repo / future scope | KUC node を vendor に表示するための最小 adapter | KDV core には入れない | egui/gpui/floem 実装を KDV core や KUC core に混ぜる |

この表に反する実装は、見た目が一時的に近くても不合格にする。

KUC core 境界検査の対象は、`KUC_ROOT` が指す `/Users/hiroyuki_furuno/works/private/katana-ui-core` と、KDV repo 配下に存在する `crates/katana-ui-core` の両方とする。どちらにも viewer 専用語彙を入れない。

### 境界固定の判定レベル

境界設計は、文書に責務を書くだけでは完了しない。

各境界は次の 4 層すべてで証明する。1 層だけ通っても完了扱いしない。

| 層 | 目的 | 合格条件 | 不合格例 |
|---|---|---|---|
| Contract | 所有者と public API を固定する | KUC public API に viewer 固有語彙がない。KDV core に viewer 固有 command がある | KUC に `Diagram` / `Image` / `MediaControl` enum を置く |
| Static guard | 明らかな逆流を即 fail する | `kdv-linter` / `kuc-adapter-boundary-check` が禁止 pattern を検出する | KDV repo 内 adapter が `cursor(UiCursor::Pointer)` や `viewer.diagram.` を直書きする |
| Roundtrip contract test | render、hit-test、action、state update が同じ契約を通ることを証明する | KUC が返した hit/action だけで KDV state が変わる | KDV が row height、field id、style class、state id から action を復元する |
| OS window / score gate | 実画面の操作・見た目・性能で破綻を fail する | click / hover / cursor / pixel / latency が KatanA reference と比較される | unit test は通るが Storybook の実画面では押せない、hover しない、score が壊れた UI を通す |

境界ごとの最低証拠:

| 境界 | Contract | Static guard | Roundtrip | OS / score |
|---|---|---|---|---|
| TreeView / FileTree | FileTree は TreeView facade | manual row geometry 禁止。FileTree が TreeView を使わず独自 row UI を作ることも禁止 | file select / directory toggle / hover cursor が KUC action 由来 | 左ペイン click / hover / selected row / icon / indent line |
| SettingsList | KUC が field action と readonly 判定を返す | manual boolean inversion / state id parse 禁止 | section toggle / field update が KUC action value 由来 | Toggle / Select / hover / cursor / state表示 |
| Interactive preset | KUC が操作可能 UI の標準 cursor / hover / callback を持つ | KDV repo 内の個別 cursor / hover override 禁止 | Button / Toggle / Link / Checkbox / Accordion の host action が preset 由来 | 実画面 hover border/background と cursor |
| MediaControl | KDV core が viewer command と layout spec を持つ | KUC への viewer media 語彙逆流、adapter の command literal 再定義禁止 | KUC button hit から KDV `ViewerMediaControlAction` へ変換 | 右上/右下配置、10ボタン全 click / hover、dark/light |
| Markdown task / link / accordion | KDV が source semantics、KUC が control/span/action | style class / state id / label parse 禁止 | task/link/accordion action が KUC host action hit 由来 | link cursor、task context menu、accordion open/close |

`Static guard` は必要条件であって十分条件ではない。台帳で `[/]` にするには、対象境界の `Roundtrip contract test` と必要な `OS window / score gate` まで到達していることを同時に示す。

## 現状確認

### 確認できている資産

KatanA fixture は KDV repo に取り込まれている。

```text
assets/fixtures/katana/sample.md
assets/fixtures/katana/sample.ja.md
assets/fixtures/katana/sample_basic.md
assets/fixtures/katana/sample_basic.ja.md
assets/fixtures/katana/sample_diagrams.md
assets/fixtures/katana/sample_diagrams.ja.md
assets/fixtures/katana/sample_html.md
assets/fixtures/katana/sample_html.ja.md
assets/fixtures/katana/sample_mermaid.md
assets/fixtures/katana/sample_mermaid_ja.md
assets/fixtures/katana/drawio/basic/*.drawio
```

### Reference artifact

`assets/reference/katana/html/`、`assets/reference/katana/pdf/`、`assets/reference/katana/export_png/`、`assets/reference/katana/preview_crops/`、`assets/reference/katana/screenshots/` に実 reference artifact が固定されている。

full-page PNG reference は KatanA screenshot runner の `export_png` step 由来として `assets/reference/katana/export_png/` に分離する。viewport screenshot と full-page export PNG を同じ `screenshots/` contract として扱わない。

preview UI reference は KatanA screenshot runner の `screenshot.crop` 由来として `assets/reference/katana/preview_crops/` に分離する。app chrome、side rail、right toolbar を含む full app screenshot を UI preview visual score の正本にしない。

確認済み artifact:

```text
assets/reference/katana/html/sample.html
assets/reference/katana/html/sample.png
assets/reference/katana/html/sample_html.html
assets/reference/katana/html/sample_html.png
assets/reference/katana/pdf/sample.pdf
assets/reference/katana/pdf/sample.png
assets/reference/katana/pdf/sample_html.pdf
assets/reference/katana/pdf/sample_html.png
assets/reference/katana/export_png/sample.png
assets/reference/katana/preview_crops/sample-top.png
assets/reference/katana/screenshots/sample.png
assets/reference/katana/screenshots/sample_html.png
```

### 現コードで確認できる実装点

次はコード上の実装点である。ただし、製品受け入れの完了証拠ではない。

- `just storybook` は smoke test ではなく `cargo run --release --locked -p kdv-storybook -- --interactive` を起動する。
- `tools/kdv-storybook/src/sidebar.rs` は現在 `FileTree` facade 経由で TreeView を作っている。完了条件は `FileTree` 利用そのものではなく、実体が KUC `TreeView` の表示・hit-test・action 契約をそのまま使うことである。
- `tools/kdv-storybook/src/sidebar_settings.rs` は `SettingsList::new` を呼んでいる。
- `scripts/kuc-adapter-boundary-check.sh` は vendor runtime と独自 tree UI の混入を検出しようとしている。
- `crates/katana-document-viewer/src/dependency_tests.rs` は KDV core から `katana-ui-core` / `egui` / `winit` / `vello` を排除するテストを持つ。
- `crates/katana-document-viewer-kuc/src/lib.rs` は KUC-facing boundary として vendor UI framework 非依存であることを明記している。ただしこの crate は KDV repo 内 UI adapter として残す正規設計ではなく、KUC 側 projection / host contract へ移設または削除する過渡実装である。

### 2026-06-04 指摘時点の未達証拠

次はユーザー実機確認とコード確認で判明した未達である。過去の automated gate 通過は、この未達を覆す証拠にならない。

2026-06-05 時点で、TreeView / SettingsList / MediaControl の一部は KUC action / KDV viewer action 境界へ移した。ただし、実 OS window 上の click / hover / visual / score gate で同じ破綻を fail できる状態までは未達である。したがって、下記は「指摘時点の破綻」として残し、是正済みのコード経路は後続の「境界修正メモ」で管理する。

- 左ペインの `TreeView` は KatanA Explorer 相当の見た目ではない。folder/file icon、indent line、selected row、row hit area、scrollbar、toolbar との乖離が大きい。
- `TreeView` の file row click が実画面で切り替わらない。KDV は暫定 `FileTreeAction::SelectFile` だけを採用し、`ToggleDirectory` を捨てていた。
- `SettingsList` の section header click が動かない。KDV は `SettingsListHitTestResult::ToggleSection` を捨てている。
- Toggle 表示は KUC atom の見た目契約と操作契約を実画面で満たしていない。KDV は行 hit から field id を拾い、`settings_action_value.rs` で次状態を独自計算している。
- Theme / Mode は KUC Select の選択操作ではなく、KDV の独自トグルとして動く。
- media control は右上 fullscreen、右下 pan / zoom / reset / copy の KatanA 仕様を満たさず、ボタンが崩れ、押せない。
- score gate は TreeView / SettingsList / Toggle / MediaControl の実画面破綻を fail できていない。
- `just storybook` で見える状態と `storybook-score-check` の合格が矛盾しているため、score は DoD の証拠として不十分である。

### 現在の score gate

`storybook-score-check` は score 契約と KatanA reference visual gate を含むが、現状では不十分である。

- `StorybookScoreReport` は `visual_score`、`semantic_score`、`interaction_score`、`performance_score` の `min()` を `final_score` とする。
- `storybook_score_visual_uses_katana_export_png_reference` は KatanA `export_png` reference と Storybook surface を比較し、95 点未満で fail する。
- `storybook_score_visual_uses_katana_preview_crop_reference` は KatanA preview pane crop と Storybook top viewport crop を比較し、95 点未満で fail する。
- KatanA preview screenshot は viewport screenshot score として別に扱う。full-page `export_png` と app chrome 込み preview screenshot を混同しない。
- `fixture_score_matrix` と `surface_equivalence` は node / export / surface equivalence の semantic・content 側 regression gate として実行される。
- `storybook-interaction-check` と `storybook-performance-check` は interaction / performance gate として別 recipe で実行される。

不足:

- Storybook の左ペイン自体を visual score 対象にしていない。
- TreeView row の click / toggle / selection / scroll を、実 OS window の click / hover で検証していない。
- SettingsList の Toggle / Select / section toggle を、KUC action 経由かつ実 OS window の click / hover で検証していない。
- MediaControl の layout と hit action を、KatanA 仕様の座標配置と実 OS window の click / hover で検証していない。
- KDV export surface と KUC preview が別々の text raster / rich text layout 実装を持っており、同じ viewer node を描いても `katana/sample.md` の surface parity が `63/95` で止まる。
- ユーザーが確認した broken state でも pass するため、score gate は現状のままでは DoD 判定に使えない。

禁止事項は継続する:

- score threshold を下げない。
- KDV 自己比較へ戻さない。
- visual score test を通常 gate から外さない。

### 補助 gate の性質

`scripts/kuc-adapter-boundary-check.sh` は必要だが十分ではない。

理由:

- grep ベースなので、KUC contract から返った action だけを使っていることまでは証明しない。
- `TreeView` / `SettingsList` の文字列検出は、表示に使っていることの証拠にはなるが、hit-test と state update が同じ契約に乗っていることの証拠にはならない。
- `state_id` parse、style class parse、固定 row height が別ファイルに残っても、pattern が漏れれば通る。
- `cargo test` の filter は、該当 test が 0 件でも command 自体が成功する可能性がある。
- screenshot や Storybook 表示は探索用であり、正しさの根拠にはならない。
- `viewer.image.` / `viewer.diagram.` / `viewer.code.` prefix は KDV core `ViewerMediaControlAction` だけが所有する。KDV repo 内 UI adapter と Storybook の直書きは `viewer_media_action_prefix` rule と boundary check で fail させる。
- `fit`、`fullscreen`、`pan-up`、`copy-source`、`copy-code` などの viewer media control command は KDV core `ViewerMediaControlSet` だけが所有する。KUC viewer projection は `ViewerMediaControlSpec` を KUC Button / Row / Column へ写すだけにし、KDV repo 内で command literal を再定義したら fail させる。
- `cursor(UiCursor::Pointer)` などの汎用操作 UI の個別補正は KDV repo 内に置かない。KUC generic preset が所有し、KDV repo 側へ戻ったら `no_manual_interactive_preset_override` で fail させる。

そのため、lint だけではなく KUC component contract tests と Storybook E2E の両方を Required Commands に含める。

## 乖離一覧

この表は乖離と再発監視項目である。2026-06-04 の automated Required Commands 通過は撤回し、未達を red test 化する。

| ID | 領域 | KatanA 参照仕様 | 当初の問題 / 再発リスク | Owner | 必要な証拠 |
|---|---|---|---|---|---|
| GAP-01 | Reference | HTML/PDF/screenshots が固定される | reference artifact が欠けると visual score の根拠がなくなる | KDV | `assets/reference/katana/**` inventory test |
| GAP-02 | Score | 4カテゴリの最小値が 95 以上 | score の意味が曖昧になると gate が形だけになる | KDV | `just storybook-score-check` がカテゴリ別 score を出す |
| GAP-03 | Self parity | KatanA と比較する | KDV 自己比較で通る余地 | KDV | `no_self_parity_score` |
| GAP-04 | Section | source_span / line mapping / footnote を保持 | KatanA section pipeline 互換が未証明 | KDV | `section_pipeline_tests` |
| GAP-05 | Markdown | alert/table/list/code/hr が node 表示 | raw 表示や不完全表示の指摘あり | KDV/KUC | `markdown_node_contract_tests` |
| GAP-06 | HTML | alignment/link/image/spacing を反映 | 中央寄せ/右寄せ/link 動作が未証明 | KDV/KUC | `html_alignment_contract_tests` |
| GAP-07 | Table | wide table と alignment を制御 | table raw 表示の指摘あり | KUC | `table_layout_contract_tests` |
| GAP-08 | Code | syntax highlight と copy button | syntax/copy が未証明 | KDV/KUC | `code_block_contract_tests` |
| GAP-09 | Task | `[ ] [x] [/] [-]` と context menu | checkbox 表現/操作が未達 | KDV/KUC | `task_control_contract_tests` |
| GAP-10 | Alert | GitHub alert 表現 | KatanA と乖離あり | KDV/KUC | `alert_visual_semantic_tests` |
| GAP-11 | Link | hover cursor と click action | link 機能が未証明 | KUC | `text_span_action_contract_tests` |
| GAP-12 | Footnote | footnote が表示/ジャンプ可能 | 注釈が出ない指摘あり | KDV/KUC | `footnote_contract_tests` |
| GAP-13 | Accordion | 開閉可能 | 開閉不可の指摘あり | KDV/KUC | `accordion_interaction_tests` |
| GAP-14 | Emoji | OS color emoji | 白黒 emoji の指摘あり | KUC | `emoji_color_contract_tests` |
| GAP-15 | Diagram lazy | pending 即時、並列 render | load 遅延/永続 pending の指摘あり | KDV | `diagram_lazy_asset_tests` |
| GAP-16 | Diagram theme | dark theme を renderer request に含める | dark が反映されない指摘あり | KDV | `diagram_theme_request_tests` |
| GAP-17 | Media controls | 右上 fullscreen、右下操作群 | layout/action/button が未達 | KDV | `viewer_media_control_contract_tests` |
| GAP-18 | Direct source | image/drawio/mermaid/plantuml/html/pdf | direct source 表示の互換が未証明 | KDV | `direct_source_contract_tests` |
| GAP-19 | Search | highlight/jump/active index | 検索機能が未証明 | KDV/KUC | `search_anchor_scroll_tests` |
| GAP-20 | Scroll | vertical scroll / bottom spacer / resize | scroll 不可/文字消失/resize 不追従の指摘あり | KUC | `scroll_resize_contract_tests` |
| GAP-21 | TreeView | KUC TreeView 実部品で選択/開閉 | 独自 TreeView / hit-test の混入履歴 | KUC/Storybook | `tree_view_contract_tests` + linter |
| GAP-22 | Settings | KUC SettingsList 実部品で更新 | `SETTINGS_ROW_HEIGHT` と `state_id` parse による Storybook hit/action が残る | KUC/Storybook | `settings_list_contract_tests` + linter |
| GAP-23 | Boundary | vendor 非依存 | egui/gpui/floem 復帰前の境界が要検証 | KDV/KUC | `kuc-adapter-boundary-check` |
| GAP-24 | Storybook UX | 左上 TreeView、左下 Settings、右 viewer | 見た目/操作の劣化指摘あり | Storybook | screenshot + interaction E2E |
| GAP-25 | Performance | file switch <= 0.6s | 10-60s の指摘あり | KDV | `storybook-performance-check` |
| GAP-26 | Media hit-test | KUC host action hit rect と KDV viewer action だけで操作 | Storybook が media layout と action prefix を再構成している | KDV/Storybook | `viewer_media_control_contract_tests` + `no_manual_media_hit_test` |
| GAP-27 | Style/state contract | style/state は見た目と安定識別であり操作契約ではない | style class / state_id が action 判定に使われ得る | KUC/Storybook | `no_style_class_action_contract_tests` |
| GAP-28 | Slideshow | modal / controls / settings / page state が動く | mode 設定と slideshow 操作の KatanA 互換が未証明 | KDV/KUC/Storybook | `slideshow_contract_tests` |
| GAP-29 | Diagram cache | 同一内容 reuse、reorder、deleted prune を扱う | KDV asset cache が KatanA cache 挙動と同等か未証明 | KDV | `diagram_cache_contract_tests` |
| GAP-30 | Link / Accordion hit-test | KUC host action hit rect の action/cursor だけで hover/click | 旧実装で line height や `<details>` target から復元していた | KUC/Storybook | `ui_tree_canvas_hit` + `no_manual_mouse_host_geometry` |
| GAP-31 | Interactive preset | 操作可能 UI の cursor / hover / click dispatch は KUC generic preset が所有する | KDV repo 内 adapter が `UiCursor::Pointer` や hover border を個別補正すると KUC 汎用契約が空洞化する | KUC/KDV | `interactive_preset_contract_tests` + `no_manual_interactive_preset_override` |
| GAP-32 | KDV/KUC boundary | KDV は engine/model のみ、KUC が viewer projection / host interaction を所有する | KDV repo 内に UI adapter / KUC projection / KUC hit wrapper が残ると、KDV が UI 座標と描画都合を背負う | KDV/KUC | `no_kdv_ui_adapter_ownership` + KUC projection contract tests |
| GAP-33 | Text raster / rich layout | KDV export、KUC preview、将来 vendor adapter が同じ text / emoji / rich span layout 契約を使う | KDV `SurfaceTextPainter` と KUC `TextRenderer` が分離しているため、bold/italic/code/emoji/line box/decoration が帯域全体でずれ、surface parity が `63/95` で止まる | KUC/KDV | shared rich text renderer contract tests + `storybook_frame_matches_export_surface_for_katana_viewer_diagrams` |

## 設計

### Viewer Model

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

必須 node kind:

```text
DocumentRoot / Section / Heading / Paragraph / Text / Link /
CodeBlock / Table / List / ListItem / TaskCheckbox /
Alert / BlockQuote / HorizontalRule / Footnote / Accordion /
HtmlBlock / Diagram / Image / PdfPage / Math / Pending / Error
```

### KUC Action Contract

KUC が action を返す。KDV は action を合成しない。

```rust
pub enum TreeViewAction {
    SelectFile { file_id: String },
    ToggleDirectory { directory_id: String },
    FocusItem { item_id: String },
    None,
}

pub enum SettingsListAction {
    SetQuery { query: Option<String> },
    ToggleSection { section_id: String },
    UpdateField { field_id: String, value: SettingsValue },
    ResetField { field_id: String },
}

pub enum ViewerMediaControlAction {
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

pub enum TextSpanAction {
    OpenLink { target: LinkTarget },
    CopyCode { node_id: NodeId },
    ToggleAccordion { node_id: NodeId },
}
```

### Asset Scheduler

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

- first frame は本文と pending を先に表示する。
- pending は 0.2 秒以内。
- file switch は stale job を cancel する。
- render result は current generation のみ採用する。
- cache key は theme を含む。
- dark theme は renderer request に含める。
- failure は diagnostic node と raw string node。

## Storybook Contract

`just storybook` は smoke test ではなく interactive viewer を起動する。

画面構成:

```text
+-----------------------------+--------------------------------------+
| KUC TreeView                | KUC Viewer                           |
| - katana/markdown           | - vertical scroll                    |
| - katana/html               | - dynamic resize                     |
| - katana/diagram            | - bottom spacer                      |
| - katana/image              | - dark theme pass-through            |
| - katana/pdf                | - search highlight/jump              |
| - direct                    | - media controls                     |
|-----------------------------|                                      |
| KUC SettingsList            |                                      |
| - theme                     |                                      |
| - dark                      |                                      |
| - mode                      |                                      |
| - slideshow                 |                                      |
| - hover highlight           |                                      |
| - selection                 |                                      |
| - image controls            |                                      |
| - diagram controls          |                                      |
| - search query              |                                      |
| - active search index       |                                      |
| - viewport                  |                                      |
| - loaded assets             |                                      |
| - failed assets             |                                      |
| - render latency            |                                      |
| - category scores           |                                      |
+-----------------------------+--------------------------------------+
```

禁止:

```text
独自 TreeView
独自 toggle
独自 settings UI
独自 media control
独自 hit-test
独自 action synthesis
style class / state id parse による action 判定
```

補足:

- 禁止する「独自 media control」は Storybook / KUC 側の独自実装を指す。
- KDV core が viewer 専用の MediaControl specification と command を持つことは必須である。
- KUC viewer projection は KDV core の specification を KUC generic Button / Stack / host action へ写すだけにする。KDV repo 内 adapter がこの変換を所有し続ける状態は過渡実装であり、DoD までに移設または削除する。

## Score Gate

### visual_score

入力:

```text
assets/reference/katana/html/*.png
assets/reference/katana/pdf/*.png
assets/reference/katana/preview_crops/*.png
assets/reference/katana/screenshots/*.png
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

実操作:

```text
TreeView select
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
|---|---|
| Fixture | `katana_fixture_inventory_tests` |
| Reference | `katana_reference_artifact_tests` |
| Linter | `no_manual_tree_hit_test_tests` |
| Linter | `no_manual_media_hit_test_tests` |
| Linter | `no_manual_settings_action_tests` |
| Linter | `no_style_class_action_contract_tests` |
| KUC | `tree_view_contract_tests` |
| KUC | `settings_list_contract_tests` |
| KUC | `interactive_preset_contract_tests` |
| KDV | `viewer_media_control_contract_tests` |
| KUC | `task_control_contract_tests` |
| KUC | `text_span_action_contract_tests` |
| KDV | `section_pipeline_tests` |
| KDV | `markdown_node_contract_tests` |
| KDV | `html_alignment_contract_tests` |
| KDV | `diagram_lazy_asset_tests` |
| KDV | `diagram_cache_contract_tests` |
| KDV | `search_anchor_scroll_tests` |
| Storybook | `storybook_tree_view_interaction_tests` |
| Storybook | `storybook_settings_interaction_tests` |
| Storybook | `storybook_media_control_interaction_tests` |
| Storybook | `storybook_task_interaction_tests` |
| Storybook | `storybook_slideshow_interaction_tests` |
| Storybook | `storybook_window_routes_visible_control_matrix` |
| Score | `storybook_score_visual_tests` |
| Score | `storybook_score_semantic_tests` |
| Score | `storybook_score_interaction_tests` |
| Score | `storybook_score_performance_tests` |

## 実装順序

この順序を守る。表示だけ先に直すことは禁止する。

1. reference artifact inventory を赤テスト化する。
2. KatanA HTML/PDF/screenshots reference を KDV assets へ固定する。
3. score gate を四カテゴリ min へ直し、現状を fail させる。
4. manual hit-test / manual action synthesis / style class action parse を lint で fail させる。
5. KUC TreeView / SettingsList / TaskControl / TextSpanAction / interactive preset contract を完成させる。
6. KDV section pipeline を KatanA 参照仕様へ揃える。
7. Markdown node contract を完成させる。
8. HTML alignment / table / alert / task / link / footnote / accordion / emoji を node と action で検証する。
9. AssetScheduler を lazy / parallel / cancellation / cache / theme 対応にする。
10. KDV repo 内の UI adapter / KUC projection / KUC hit wrapper を削除または KUC 側へ移設する。
11. KUC viewer projection / host contract を完成させる。
12. Storybook を KUC 実部品 host として再構築する。
13. interaction E2E を全項目通す。
14. performance E2E を通す。
15. visual / semantic / interaction / performance score を全カテゴリ 95 点以上にする。
16. `just storybook` を interactive viewer として最終確認する。
17. egui / gpui / floem adapter 復帰可否をこの後に判断する。

## 2026-06-04 再計画

### 判定

KDV v0.2.0 は未達である。

過去の automated gate 通過、`pass` 表示、`[x]` checklist は製品受け入れの証拠として撤回する。

理由:

- ユーザー実機確認で TreeView / FileTree / SettingsList / Toggle / MediaControl が動いていない。
- score gate はこの broken state を fail できていない。
- KDV 側の一部経路は KUC action を受ける形へ是正済みだが、実 OS window の click / hover と visual gate で証明できていない。
- KatanA Explorer / Preview と見た目・操作・検証の乖離が残っている。

### 直近の事実と残リスク

| 領域 | 事実 | 影響 |
|---|---|---|
| TreeView 表示 | KatanA Explorer の folder/file icon、indent line、row hover、selected row と乖離している | KUC TreeView としての実利用品質が未達 |
| TreeView 操作 | KDV unit/canvas test では KUC `FileTreeAction` を受けているが、実 OS window の click 証跡が不足している | fixture 選択が Storybook の基本操作として未証明 |
| Directory toggle | KDV unit/canvas test では `ToggleDirectory` を扱っているが、実 OS window の開閉 pixel 検証が不足している | TreeView の開閉操作が未証明 |
| Settings section | KDV unit/canvas test では `SettingsListAction::ToggleSection` を扱っているが、実 OS window の開閉 pixel 検証が不足している | section 開閉が未証明 |
| Toggle | KUC Toggle の default preset はあるが、Storybook 実 window の hover / click point / visual score が不足している | KUC 由来の Toggle として未証明 |
| Select | Theme / Mode は KUC `SettingsListAction::UpdateField` の値を反映する経路に寄ったが、実 OS window と render request 連動の証跡が不足している | 設定 UI の操作契約が未証明 |
| Media controls | KDV viewer action と KUC host action hit rect の境界には寄ったが、右上/右下の KatanA controller layout と全ボタン実clickが未証明 | diagram/image 操作が未達 |
| Score | broken UI でも pass する | DoD 判定として無効 |

### 境界リスクファイル

| ファイル | リスク内容 | 必要な是正 |
|---|---|---|
| `tools/kdv-storybook/src/sidebar.rs` | `FileTree` helper 経由で TreeView を作っている。FileTree が独立 widget 化すると境界が再び崩れる | `FileTree` は path / fixture から KUC `TreeView` node を作る thin helper に限定し、KatanA Explorer 相当の visual / hit gate を追加する |
| `tools/kdv-storybook/src/sidebar_hit.rs` | sidebar content inset と KUC hit-test の接続はあるが、OS window 座標との一致は未証明 | 実 window click / hover の座標を KUC action / cursor へ落とす E2E を追加する |
| `tools/kdv-storybook/src/settings_action.rs` | KUC `SettingsListAction` は受けているが、field id と viewer state の対応がKDV側に残る | field id は viewer setting model への写像だけに限定し、次状態は KUC action の `value` から反映することを回帰テストで固定する |
| `tools/kdv-storybook/src/settings_action_value.rs` | KUC action value を viewer state へ写す境界ファイルであり、独自反転が戻ると破綻する | `!dark` や option順の推測による反転を lint / test で fail させる |
| `tools/kdv-storybook/src/mouse_media.rs` | media command は KDV viewer 専用だが、実表示座標との対応が壊れると押せない | 表示された KDV MediaControl button の実 click で全 command を検証する |
| `tools/kdv-storybook/src/mouse_media_hit.rs` | action 取得はあるが、KatanA 仕様の layout 崩れを fail できない | layout と hit action を同一 test で検証する |
| `crates/kdv-linter/src/rules/storybook_contract.rs` | grep 監視であり、KUC action 経路そのものの証明ではない | lint は補助に落とし、contract / E2E を主証拠にする |

### 2026-06-05 境界修正メモ

左ペインの render / hit / cursor / scroll は同じ sidebar content contract を使う。

旧状態:

- render は `x=8`、`y=8`、`width=SIDEBAR_WIDTH-16`、`height=window_height-16` で描いていた。
- hit / cursor / scroll は `x < SIDEBAR_WIDTH` と `height=window_height-8` を使っていた。
- そのため、KUC `FileTree` を使っていても、描画されていない左右/下余白で hit が成立し得た。

是正:

- `tools/kdv-storybook/src/layout.rs` に `sidebar_content_*` contract を追加した。
- `frame.rs`、`frame_kuc_renderer.rs`、`sidebar_hit.rs`、`window_mouse.rs`、`window_scroll.rs` を同じ contract 参照へ統一した。
- `sidebar_hit_rejects_clicks_outside_rendered_sidebar_content_inset` で、左/右/上下ガターの click が成立しないことを固定した。
- `sidebar_scroll_rejects_unrendered_sidebar_content_y_inset` で、上下ガターの sidebar scroll が state を変えないことを固定した。

検証:

- `rtk cargo test -p kdv-storybook --locked sidebar -- --test-threads=1` は `32 passed`。
- `rtk cargo test -p kdv-storybook --locked sidebar_scroll_rejects_unrendered_sidebar_content_y_inset -- --test-threads=1` は `1 passed`。
- `rtk cargo test -p kdv-storybook --locked window -- --test-threads=1` は `41 passed`。
- `rtk cargo fmt --check -p kdv-storybook` は通過。
- `rtk just kuc-adapter-boundary-check` は `kuc-adapter-boundary-check: ok`。
- `rtk just storybook-window-smoke` は `storybook-window-smoke: ok fixtures=16 checked=16`。

### 2026-06-05 操作マトリクス追加

`storybook_window_routes_visible_control_matrix` は、同じ StorybookWindow 座標経路で次の代表操作を通す。

- KUC FileTree の file select。
- KUC FileTree の directory toggle。
- KUC SettingsList の `Dark` toggle。
- KUC SettingsList の `Mode` select。
- Markdown link の hover cursor と click command。
- Task checkbox click。
- Accordion toggle。
- Diagram control の `zoom-in` click。
- Markdown link の hover block background。
- Accordion header の hover block background。

この test は実 OS event injection ではない。minifb へ入る unscaled canvas 座標と同じ `StorybookWindow::apply_canvas_click` 経路を通す contract test である。したがって、実 OS window 上の cursor/pixel/screenshot 証跡と score gate は引き続き未達として扱う。

検証:

- `rtk cargo test -p kdv-storybook --locked storybook_window_routes_visible_control_matrix -- --test-threads=1` は `1 passed`。
- `rtk cargo test -p kdv-storybook --locked interaction_matrix -- --test-threads=1` は `6 passed`。
- `rtk just storybook-interaction-check` は通過。`window 49 passed` で、上記操作マトリクスも実行される。

残り:

- Toggle atom の visual / hover / click point を KatanA 相当で採点する gate が未完。
- MediaControl は KDV 専用 widget として、KUC generic host action / hit rect だけを使う形の visual + click gate が未完。

### 2026-06-05 Link / Accordion hover 境界修正

原因:

- KUC host action hit は Link / Accordion の target node id を返していた。
- KDV Storybook はその target を `hovered_action_node_id` として KUC tree へ渡していた。
- しかし KUC Storybook renderer は `interaction.hovered` が付いた Text / Accordion に generic hover background を描いていなかったため、cursor は変わるが block hover surface が window pixel に出なかった。

修正:

- KUC Storybook renderer が `interaction.hovered` の付いた Text / Accordion node に `hover_background` を描く。
- KDV Storybook は Link / Accordion の hover 対象を KUC host action hit から解決し、KUC tree の hovered node id として渡すだけにする。
- MediaControl はこの修正対象ではない。MediaControl は KDV viewer 専用 widget のまま、KUC generic Button / host action / hit rect / interactive preset だけを利用する。

検証:

- 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core-storybook --locked generic_text_hover_draws_kuc_hover_background_before_text -- --test-threads=1` は `1 passed`。
- `rtk cargo test -p kdv-storybook --locked hover_resolves_viewer_target -- --test-threads=1` は `2 passed`。
- `rtk cargo test -p kdv-storybook --locked hover_draws_block_hover_surface -- --test-threads=1` は `2 passed`。
- `rtk cargo test -p kdv-storybook --locked interaction_matrix -- --test-threads=1` は `6 passed`。
- `rtk cargo test -p kdv-storybook --locked window -- --test-threads=1` は `49 passed`。
- `rtk just storybook-interaction-check` は通過。`mouse 17 passed`、`window 49 passed` を含む。

KUC interactive preset / hit-test 境界:

- KUC `UiCommonProps.hover_border` は props だけではなく、Storybook renderer の Button / Toggle / entry 系 control の実 pixel に反映する。
- KUC `interactive_preset_contract` は Button / TextButton / SvgButton / IconTextButton / Checkbox / Radio / Toggle / ColorSwatch / SlideControl が、利用側補正なしで `Pointer` と `control.hover.border` を持つことを固定する。
- KUC Storybook canvas renderer は Button / Toggle / Checkbox / Radio / ColorSwatch / SlideControl / SettingsList FormField の hover border を実 pixel へ描く。KDV はこれらの部品ごとに hover border を描かない。
- KUC `UiTree::with_hovered_node_id()` を追加し、host は KUC node id だけを渡して transient hover state を反映する。host が border や hover surface を直接描かない。
- KUC Storybook hit collector は `ScrollArea` と container `common.padding` を renderer と同じ child origin で扱う。表示座標と host action hit rect がずれたら KUC contract test で fail する。
- KUC Storybook hit collector は link action rect を Text node 全幅ではなく link span 単位で返す。KDV は link 行高や Text node 全幅から link action を復元しない。
- link span の hit rect は簡易文字幅ではなく、KUC Storybook renderer と同じ `TextRenderer::measure_width` と `UiTreeTextMetrics` で計算する。
- KUC SettingsList は activation action を持つ field だけ `Pointer` cursor を返す。readonly state row は表示専用として扱い、KDV が action を合成しない。
- 検証: `rtk cargo test -p katana-ui-core --locked --test interactive_preset_contract -- --test-threads=1` は `5 passed`。
- 検証: `rtk cargo test -p katana-ui-core --locked with_hovered_node_id -- --test-threads=1` は `2 passed`。
- 検証: `rtk cargo test -p katana-ui-core-storybook --locked generic_button_hover_draws_kuc_interactive_preset_border -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p katana-ui-core-storybook --locked generic_toggle_hover_draws_kuc_interactive_preset_border -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_interactive_hover -- --test-threads=1` は `5 passed`。
- 検証: `rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_hit -- --test-threads=1` は `13 passed`。
- 検証: `rtk cargo test -p katana-ui-core --locked settings -- --test-threads=1` は `17 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked cursor -- --test-threads=1` は `6 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked sidebar_hit -- --test-threads=1` は `9 passed`。
- 残課題: Link span の実 pixel と callback fallback、全 control の OS window hover/click matrix は、同じ KUC preset 契約でまだ全セル固定できていない。

TreeView hover 境界:

- `UiTreeProps.hovered_id` と `FileTreeState.hovered_item_id` を KUC 側に追加した。
- KUC Storybook renderer は `hovered_id` に一致する row を `hover_background` で描く。
- KDV Storybook は pointer 座標から row を再計算せず、`SidebarHit::hit` が返す KUC `FileTreeAction` 結果だけを `FileTreeState.hovered_item_id` へ反映する。
- 検証: `rtk cargo test -p katana-ui-core --locked file_tree -- --test-threads=1` は `13 passed`。
- 検証: `rtk cargo test -p katana-ui-core-storybook --locked tree_canvas_draws_hover_row_background_from_tree_hovered_id -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked sidebar_tree_hover_updates_kuc_file_tree_hover_state -- --test-threads=1` は `1 passed`。

SettingsList action 境界:

- KUC `SettingsList::action_for_hit()` を追加し、表示行 hit から `SettingsListAction::UpdateField` / `ToggleSection` を直接返す。
- KDV Storybook は settings hit を `StorybookSettingsField` や独自 `ToggleSettingsSection` へ変換せず、`SidebarHitResult::SettingsAction(SettingsListAction)` として window 側へ渡す。
- KDV Storybook は KUC `UpdateField.value` を採用して viewer state へ反映し、field id から次状態を再計算しない。
- KUC `SettingsList::cursor_for_hit()` は readonly field で `Default` を返す。KDV Storybook は readonly field から `SettingsListAction::UpdateField` を合成しない。
- `kdv-linter` は Storybook の `= !` boolean 反転を `no_manual_settings_action` として fail させる。KDV は KUC action の `value` を viewer state に写すだけにする。
- 検証: `rtk cargo test -p katana-ui-core --locked action_for_hit -- --test-threads=1` は `2 passed`。
- 検証: `rtk cargo test -p katana-ui-core --locked settings -- --test-threads=1` は `17 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked settings -- --test-threads=1` は `12 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked sidebar_hit -- --test-threads=1` は `9 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked sidebar -- --test-threads=1` は `33 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked window -- --test-threads=1` は `42 passed`。
- 検証: `rtk cargo test -p kdv-linter --locked storybook_contract -- --test-threads=1` は `2 passed`。

MediaControl 境界:

- `ViewerMediaControlSet`、`ViewerMediaControlAction`、`ViewerMediaControlKind` は KDV core が所有する。
- KUC viewer projection は KDV core の control spec を KUC `Button`、`UiHostActionSpec`、`UiNode` へ写すだけにする。KDV repo 内 adapter は最終所有者ではない。
- KUC core は `Diagram`、`Image`、`Code`、`pan`、`zoom`、`copy-source` などの viewer 固有語彙を所有しない。
- Storybook は KUC Storybook renderer から返る host action hit rect を KDV `ViewerMediaControlAction` へ渡すだけにする。
- MediaControl hover は KDV が矩形や border を直接描かず、KUC host action hit の `target` node id を `UiTree::with_hovered_node_id()` へ渡して KUC interactive preset border として描画する。
- KUC Storybook renderer / hit collector は `kdv-diagram-frame` / `kdv-diagram-toolbar` / `kdv-diagram-top-controls` を layout / hit-test 契約として読まない。generic `Stack` + `UiPosition::Absolute` + `UiEdgeInsets::margin` だけで overlay child を配置する。
- KUC viewer projection は KDV専用 MediaControl の右上 / 右下配置を generic absolute overlay props へ写すだけにする。KUC core は MediaControl の command、対象種別、図形意味論を所有しない。
- 再発防止: `kuc-adapter-boundary-check` は外部KUC Storybook `ui_tree_canvas*` が KDV diagram style class を placement / hit-test 契約として参照したら fail する。
- 再発防止: 外部KUC Storybook `ui_tree_canvas*` は host action id を不透明文字列として扱う。test 例でも `viewer.image.` / `viewer.diagram.` / `viewer.code.` を使わず、KDV viewer media command に見える文字列が戻ったら `kuc-adapter-boundary-check` で fail する。
- 検証: `rtk cargo test -p katana-document-viewer --locked media_control -- --test-threads=1` は `3 passed`。
- 検証: `rtk cargo test -p katana-document-viewer-kuc --locked media_control -- --test-threads=1` は `4 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked media -- --test-threads=1` は `7 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked mouse_left_click_on_every_diagram_control_returns_command -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked media_control_hover_reaches_kuc_interactive_preset_border_pixels -- --test-threads=1` は `1 passed`。
- 残課題: `rtk cargo test -p kdv-storybook --locked media -- --test-threads=1` は `29.57s` かかっており、UI操作性能の DoD には未達。

### 参照すべき KatanA UI

TreeView は KatanA Explorer を参照仕様にする。

`FileTree` を独立 widget として育てない。残す場合でも、path / fixture を `TreeView` node に変換する thin helper に限定する。

必須:

- directory row は folder icon、chevron、indent line を持つ。
- file row は file type icon を持つ。
- selected row は行全体の背景で表現する。
- hover は行全体に薄い背景を当てる。
- directory row click は開閉する。
- file row click は対象 fixture を開く。
- scroll しても行 hit-test と表示順が一致する。
- label が長い時も hit area と selection background が崩れない。

Settings は KUC SettingsList を参照仕様にする。

必須:

- Toggle は KUC `Toggle` atom の見た目と action を使う。
- Select は KUC `SelectBox` の選択 action を使う。
- section header click は KUC `SettingsListAction::ToggleSection` を使う。
- KDV は field id から次状態を独自計算しない。
- KDV は KUC action / event の結果だけを viewer state に反映する。

Media controls は KDV viewer 専用 widget として KatanA diagram controller を参照仕様にする。

必須:

- 右上に fullscreen と copy を配置する。
- 右下に pan up / left / reset / right / down と zoom in / zoom out / copy source を配置する。
- button は transparent base、hover 時だけ背景を出す。
- KUC は汎用 host action と hit rect だけを返す。
- KDV は viewer model から viewer media action を解決する。
- KDV は座標・style class・state id から action を復元しない。

### Red Test First

次の red test を先に追加し、現状で fail することを確認する。

| ID | Test | Fail させる現状 |
|---|---|---|
| RED-01 | `storybook_tree_view_visible_like_katana_explorer` | folder/file icon、indent line、selected row が不足 |
| RED-02 | `storybook_tree_view_click_selects_visible_file_row` | file row click が selected fixture を変えない |
| RED-03 | `storybook_tree_view_directory_click_toggles_directory` | `ToggleDirectory` を KDV が捨てる |
| RED-04 | `storybook_tree_view_scroll_keeps_hit_order` | scroll 後の表示行と hit row が一致しない |
| RED-05 | `storybook_settings_section_click_toggles_section` | `ToggleSection` を KDV が捨てる |
| RED-06 | `storybook_settings_toggle_uses_kuc_toggle_action` | KDV 独自反転で Toggle が動く |
| RED-07 | `storybook_settings_select_uses_kuc_select_action` | Theme / Mode が Select 操作ではない |
| RED-08 | `storybook_toggle_visual_matches_kuc_toggle_contract` | Toggle の見た目が KUC atom と乖離 |
| RED-09 | `storybook_media_controls_match_katana_layout` | 右上/右下 controls が崩れている |
| RED-10 | `storybook_media_controls_click_emit_kuc_actions` | ボタンが押せない |
| RED-11 | `storybook_score_fails_when_sidebar_or_controls_broken` | broken UI でも score が pass する |

### 修正順

1. 計画書から過去の `pass` / `[x]` を撤回する。
2. RED-01 から RED-11 を追加し、現在の broken state で fail させる。
3. KUC TreeView 契約を KatanA Explorer 相当へ拡張する。
4. KDV Storybook は TreeView の表示・hit-test・toggle・selection を KUC action 結果だけで扱う。
5. KUC SettingsList / Toggle / Select の action 契約を実操作へ接続する。
6. KDV Storybook は Settings の次状態を独自計算せず、KUC event の結果を反映する。
7. KDV viewer MediaControl 契約を KatanA diagram controller 相当へ拡張する。
8. KDV Storybook は media controls を KUC host action hit rect と KDV viewer action 結果だけで扱う。
9. score gate を左ペイン、設定 UI、media controls の visual / interaction まで拡張する。
10. `just storybook` で実画面を開き、KatanA fixture をクリック操作で切り替えられることを確認する。

## Required Commands

```text
just clean
just kdv-lint
just kuc-adapter-boundary-check
cargo test --workspace --locked
just storybook-window-smoke
just storybook-treeview-check
just storybook-settings-contract-check
just storybook-media-control-clickability-check
just storybook-interaction-check
just storybook-content-check
just storybook-performance-check
just storybook-score-check
just storybook-score-audit-check
just storybook
```

各 command は「通った」だけでは不十分である。

必要な意味:

- `just kdv-lint`: 設計違反が機械的に検出される。
- `just storybook-entrypoint-check`: `just storybook` が interactive viewer であり、`storybook-check` から境界証拠 gate が外れていない。
- `just kuc-adapter-boundary-check`: vendor runtime と独自 Storybook UI が混入していない。
- `cargo test --workspace --locked`: KDV/KUC/linter/Storybook contract が通る。
- `just storybook-window-smoke`: window が interactive viewer として起動する。
- `just storybook-treeview-check`: TreeView の表示、select、toggle、scroll、hit order が KatanA Explorer 相当で動く。
- `just storybook-settings-contract-check`: SettingsList の section toggle、Toggle、Select が KUC action で動く。
- `just storybook-media-control-clickability-check`: MediaControl の右上/右下配置、重なりなし、全 button click が成立する。
- `just storybook-interaction-check`: 実操作が KUC action 経由で成立する。
- `just storybook-content-check`: raw 表示ではなく node 表示が成立する。
- `just storybook-performance-check`: 0.6s / 0.2s の基準を満たす。
- `just storybook-score-check`: 四カテゴリすべて 95 以上。
- `just storybook-score-audit-check`: 95 未満の category score が pass 扱いにならず、broken sidebar/control が fail になる。
- `just storybook`: ユーザーが確認できる interactive viewer を起動する。

## 2026-06-05 追加是正メモ

境界修正:

- Storybook asset job は `PreviewBuilder::default()` で別 cache を作り直さず、window が持つ `PreviewBuilder` clone の shared source / parsed / artifact cache を使う。
- Storybook viewer viewport は実描画領域 `window - sidebar - padding` と一致させる。scene構築、KUC tree描画、click hit-test は同じ `layout` helper の幅・高さを使う。
- KDV固有 MediaControl はKDV内の viewer command として扱う。KUCは汎用 host action と hit rect を出すだけで、MediaControl widget や diagram semantics を所有しない。
- host action hit rect は `PreviewSceneHostActionCache` で on-demand cache する。scene構築時に全fixtureへ先読みしない。
- KUC generic control の hover border は KUC renderer が描く。KDV は MediaControl hover 時も KUC host action target node id を渡すだけで、hover border を直接描かない。
- KUC hit collector は `ScrollArea` と container `common.padding` を renderer と同じ座標契約へ揃える。表示位置と click / hover hit rect のズレは KUC contract test で fail する。
- KUC hit collector は link span の rect を Text node 全幅ではなく、KUC renderer と同じフォント計測と text metrics で算出する。
- KUC Storybook `TextRasterCache` は通常文字を alpha mask として保持し、OS color emoji など色付き glyph だけ per-pixel color override を保持する。KDV は emoji span を KUC へ渡すだけで、emoji pixel を独自描画しない。
- KUC TreeView の行高は KatanA Explorer と同じ 22px を契約値にする。hit-test、FileTree scroll extent、KUC Storybook 通常 renderer は同じ `TreeView::row_height()` を基準にする。
- KUC Storybook 通常 renderer の TreeView は、13px文字、12px indent、directory chevron + folder icon、file row の chevron 予約幅 + file icon、行全体 selected / hover background を持つ。KDV Storybook はこの描画をそのまま使い、TreeView icon / row background を独自描画しない。

性能実測:

- `direct_media_frames_cover_all_supported_visual_inputs`: 44.33秒から8.41秒へ改善。
- `mouse_left_click_on_every_diagram_control_returns_command`: 13.03秒から6.25秒へ改善。
- `media`: 58.84秒から29.57秒へ改善。MediaControl hover pixel 回帰が増えたため、25.98秒時点よりは増えている。
- ただし、file switch first frame 0.6秒、pending 0.2秒のDoDには未達。diagram scene構築、asset cache設計、score gate実行時間は継続是正対象。

追加検証:

- `rtk cargo test -p katana-ui-core --locked --test interactive_preset_contract -- --test-threads=1` は `5 passed`。
- `rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_hit -- --test-threads=1` は `13 passed`。
- `rtk cargo test -p katana-ui-core --locked --test host_action_plan_contract -- --test-threads=1` は `5 passed`。
- `rtk cargo test -p kdv-linter --locked kuc_core_boundary -- --test-threads=1` は `5 passed`。
- `rtk just storybook-settings-contract-check` は KUC settings `17 passed`、KUC hover `1 passed`、KDV settings_action `3 passed`、KDV settings `14 passed`、window settings hover `1 passed`。
- `rtk just storybook-media-control-clickability-check` は KUC hit `13 passed`、KUC icon transparent `1 passed`、KDV media_control `3 passed`、KDV-KUC media_control `4 passed`、Storybook media `11 passed`、Storybook media_control `7 passed`。
- `rtk just --dry-run storybook-check` で `storybook-settings-contract-check` と `storybook-media-control-clickability-check` が総合 gate に含まれることを確認した。
- `rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_interactive_hover -- --test-threads=1` は `5 passed`。
- `rtk cargo test -p katana-ui-core-storybook --locked generic_button_hover_draws_kuc_interactive_preset_border -- --test-threads=1` は `1 passed`。
- `rtk cargo test -p katana-ui-core-storybook --locked generic_toggle_hover_draws_kuc_interactive_preset_border -- --test-threads=1` は `1 passed`。
- `rtk cargo test -p katana-ui-core-storybook --locked renderer_preserves_color_pixels_for_os_emoji -- --test-threads=1` は `1 passed`。
- `rtk cargo test -p katana-ui-core-storybook --locked text_tests -- --test-threads=1` は `28 passed`。
- `rtk cargo test -p katana-ui-core-storybook --locked text_antialias -- --test-threads=1` は `4 passed`。
- `rtk cargo test -p katana-ui-core --locked row_height_matches_katana_explorer_contract -- --test-threads=1` は `1 passed`。
- `rtk cargo test -p katana-ui-core --locked file_tree -- --test-threads=1` は `13 passed`。
- `rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_tests -- --test-threads=1` は `38 passed`。
- `rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_tree -- --test-threads=1` は `2 passed`。
- `rtk cargo test -p kdv-storybook --locked sidebar_frame_file_tree_selection_uses_kuc_full_row_background -- --test-threads=1` は `1 passed`。
- `rtk just storybook-treeview-check` は `katana-ui-core file_tree 13 passed`、`katana-ui-core tree_view 9 passed`、`katana-ui-core-storybook ui_tree_canvas_tests 38 passed`、`katana-ui-core-storybook ui_tree_canvas_tree 2 passed`、`kdv-storybook sidebar 36 passed`。
- `rtk cargo test -p katana-document-viewer --locked settings_update -- --test-threads=1` は `4 passed`。
- `rtk cargo test -p kdv-storybook --locked settings_action -- --test-threads=1` は `3 passed`。
- `rtk cargo test -p kdv-storybook --locked settings -- --test-threads=1` は `13 passed`。
- `rtk cargo test -p kdv-linter --locked storybook_contract -- --test-threads=1` は `2 passed`。
- `rtk cargo test -p kdv-storybook --locked media_control_hover_reaches_kuc_interactive_preset_border_pixels -- --test-threads=1` は `1 passed`。
- `rtk cargo test -p kdv-storybook --locked html_alignment -- --test-threads=1` は `2 passed`。direct HTML と `katana/sample_html.md` の実frame alignmentを検査する。
- `rtk just storybook-interaction-check` は通過。commands / mouse / scroll / sidebar / search / slideshow / window の既存自動ゲートを確認し、window suite は `47 passed`。
- `rtk just storybook-window-smoke` は `storybook-window-smoke: ok fixtures=16 checked=16`。interactive window entry の fixture 巡回 smoke を確認した。

## 残リスク

現時点では Required Commands を DoD 合格証拠として扱わない。RED-01 から RED-11 を通した後に再評価する。

- `just clean` 実行後は `target/` が削除されるため、追加編集後は再度 gate を回す。
- worktree には v0.2.0 作業由来の未追跡ファイルが多い。commit 前は関心ごとごとに diff を分けて再確認する。
- Storybook screenshot は探索用である。ただし、今回のように test / lint / score gate が broken UI を見逃した場合は、screenshot 由来の不備を red test に変換する。
- egui / gpui / floem adapter はこの recovery gate の後に判断する。
- KDV Storybook の link / task / accordion hover-click 経路は KUC host action rect 経由へ移行済み。label、uri、row rect、line height、state id を action contract として読む再発は `kuc-adapter-boundary-check` で fail する。accordion の `state_id` fallback と task marker の source 行探索は撤去済み。
- KUC `SettingsListAction` 後の viewer state 変換は KDV core `ViewerSettingsState::apply_update()` へ移した。Storybook は KUC `SettingsValue` を KDV `ViewerSettingsValue` へ変換するだけにする。
- `settings_action.rs` の inline tests は `settings_action_tests.rs` へ分離し、実装ファイルを 143 行へ戻した。
- KDV Storybook の検証補助に残っていた `kdv-task-checkbox` style class を action 収集キーにする経路は、KUC `UiHostActionPlan::task_control_target()` 経由へ移行した。
- FileTree の window 経路 hover は `storybook_window_file_tree_hover_draws_kuc_row_background` で固定した。これは `StorybookWindow` の sidebar hover state から再描画した frame pixel で KUC `TreeView` row hover background が増えることを見る。
- FileTree facade の static guard は固定パスではなく、KUC core 内の `pub struct FileTree` / `impl FileTree` を巡回する。外部KUCでは FileTree 実装を必須とし、KDV配下の部分ミラーは存在時のみ同じ facade contract を検査する。
- SettingsList field hover は KUC `SettingsList::field_node_id()` を唯一の stable node id 契約にした。KDV Storybook は `SettingsListAction::UpdateField` の field id をこの KUC API に渡し、`UiTree::with_hovered_node_id()` で KUC interactive preset border を描かせるだけにする。
- SettingsList の window 経路 hover は `storybook_window_settings_toggle_hover_draws_kuc_preset_border` で固定した。これは `StorybookWindow` の sidebar hover state から再描画した frame pixel で KUC preset border が増えることを見る。
- MediaControl の transparent base / layout / hover は KDV viewer 専用契約として `frame_media_control_tests/` に分離した。KDV overlay control は KUC `UiVariant::Icon` を使い、全 hit rect が dark / light で selection fill にならないこと、KatanA 28px + gap 配置、全 control hover border を固定した。KUC renderer 側も `kdv-diagram-button` style class ではなく generic `Icon` / `Outline` variant を透明 base の根拠にする。
- 現存する過渡 adapter の MediaControl / ImageControl / CodeCopyControl から `kdv-diagram-*` / `kdv-image-control` / `kdv-code-control` style class を撤去した。`kdv-linter` の `no_style_class_action_contract` はこれらが実装へ戻ったら fail する。
- MediaControl の window 経路 hover/click は `storybook_window_media_control_hover_draws_interactive_preset_border` で固定した。これは `StorybookWindow` の canvas hover state から再描画した frame pixel で KUC preset border が増えることと、同じ pointer の click が KDV `ViewerMediaControlAction` へ dispatch されることを見る。
- 外部KUC Storybook renderer の `kdv-diagram-*` layout 特例は撤去した。KUC generic overlay / absolute layout contract として `Stack` absolute child + margin を描画 / hit-test へ反映し、KDV専用 MediaControl placement を KUC renderer が class 名で知らない状態へ戻した。
- 外部KUC Storybook の overlay host action test 例から `viewer.diagram.*` を撤去し、`surface.overlay.*` の不透明 action id に置き換えた。KUC Storybook test/helper 名も `diagram_control_*` ではなく `overlay_control_*` へ寄せた。
- 外部KUC Storybook visual renderer / hit collector の KDV prefix 直接分岐は `UF-037` として撤去済み。これにより KUC renderer は KDV 文書 class 名ではなく、KUC 汎用 role / visual role / common props / text props を描画する。
- `kdv-document-rule` は現存する過渡 adapter が汎用 `Divider` の `height` / `width` / `border` props として渡すように変更した。外部KUC Storybook renderer / scroll measurement は `kdv-document-rule` を直接読まない。
- KDV core の `ViewerNodeKind::Rule` 高さは export surface Rule と同じ 34px とする。KUC 側は Divider props を描画するだけで、文書水平線 class を知らない。
- 再発防止: `kuc-adapter-boundary-check` は外部KUC Storybook `ui_tree_canvas*` へ `kdv-document-rule` / `DOCUMENT_RULE_CLASS` が戻ったら fail する。
- `kdv-document-media` は現存する過渡 adapter が `ImageSurface` の汎用 `UiVisualRole::MediaFrame` props として渡すように変更した。外部KUC Storybook renderer / hit collector は `MediaFrame` role だけで media frame の中央配置と高さ計算を行い、KDV document media class を読まない。
- 再発防止: `kuc-adapter-boundary-check` は外部KUC Storybook `ui_tree_canvas*` へ `kdv-document-media` / `DOCUMENT_MEDIA_CLASS` / `DOCUMENT_MEDIA_VERTICAL_MARGIN` が戻ったら fail する。
- 検証: 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_tests -- --test-threads=1` は `39 passed`、`rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_hit -- --test-threads=1` は `13 passed`。
- 検証: `rtk cargo test -p katana-document-viewer-kuc --locked image_surface_node_uses_default_transform_without_viewport_state -- --test-threads=1` は `1 passed`。
- `kdv-code-frame` / `kdv-code-controls` は現存する過渡 adapter が汎用 `Stack` + absolute child + margin props として渡すように変更した。外部KUC Storybook renderer / hit collector は code copy overlay を KDV code frame class ではなく generic absolute overlay layout で処理する。
- `kdv-document-code` は現存する過渡 adapter が code body の汎用 `border` / `padding` props として渡すように変更した。外部KUC Storybook text renderer / text metrics は `common.border.visible`、`common.padding.left`、`common.padding.top` だけで code block box と text inset を描画する。
- `kdv-alert-*` は現存する過渡 adapter が alert body の汎用 `severity` と `border.color_token` props として渡すように変更した。外部KUC Storybook text renderer は `common.border.color_token` と `severity` だけで alert accent を描画する。
- list depth / bullet marker は現存する過渡 adapter が row / marker node の汎用 `common.margin.left` と `text_role("list-marker")` の空ラベルとして渡すように変更した。外部KUC Storybook renderer / hit collector は `kdv-list` class ではなく `common.margin.left` だけで list indent と bullet depth を扱う。
- blockquote depth / bullet は現存する過渡 adapter が blockquote line の汎用 `common.margin.left` と `common.padding.left` として渡すように変更した。外部KUC Storybook text renderer は `common.margin.left` だけで quote depth を、`common.padding.left` だけで quoted bullet offset を扱う。
- heading は現存する過渡 adapter から `kdv-document-heading` class を撤去した。外部KUC Storybook text renderer は heading underline を `common.border.visible` がある場合だけ描く。
- 再発防止: `kuc-adapter-boundary-check` は外部KUC Storybook `ui_tree_canvas*` へ `kdv-alert-*` / `kdv-document-code` / `kdv-code-frame` / `kdv-code-controls` / `kdv-list` / `kdv-document-quote` / `kdv-document-heading` / `QUOTE_DEPTH_CLASS_PREFIX` / `CODE_FRAME_CLASS` / `CODE_CONTROLS_CLASS` / `CODE_CONTROL_MARGIN` / `CODE_CONTROLS_WIDTH` が戻ったら fail する。
- 検証: `rtk cargo test -p katana-document-viewer-kuc --locked code_block_has_copy_control_without_losing_code_body -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p katana-document-viewer-kuc --locked blockquote_node_uses_kuc_lines_with_common_depth_props -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p katana-document-viewer-kuc --locked document_heading_uses_plain_heading_role_and_code_uses_common_props -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p katana-document-viewer-kuc --locked text_roles_cover_alert_and_footnote_nodes -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p katana-document-viewer-kuc --locked list_node_preserves_nested_depth_as_row_common_margin -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p katana-document-viewer-kuc --locked unordered_list_marker_uses_bullet_canvas_marker_without_dash_label -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked katana_preview_feature_matrix_by_fixture -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked katana_required_features_reach_kuc_storybook_scene -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked storybook_window_file_tree_hover_draws_kuc_row_background -- --test-threads=1` は `1 passed`。
- 検証: 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core --locked settings_field_id_reaches_form_field_state_id -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked storybook_window_settings_toggle_hover_draws_kuc_preset_border -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked storybook_window_media_control_hover_draws_interactive_preset_border -- --test-threads=1` は `1 passed`。
- 検証: `rtk cargo test -p kdv-storybook --locked interaction_matrix -- --test-threads=1` は `4 passed`。
- 検証: 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core-storybook --locked document_code -- --test-threads=1` は `2 passed`。
- 検証: 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core-storybook --locked document_alert_uses_alert_kind_accent_for_stripe -- --test-threads=1` は `1 passed`。
- 検証: 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core-storybook --locked list_marker_bullet_uses_text_color_material_dot -- --test-threads=1` は `1 passed`。
- 検証: 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core-storybook --locked nested_list_markers_switch_shape_by_depth -- --test-threads=1` は `1 passed`。
- 検証: 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core-storybook --locked document_blockquote_depth_draws_nested_bars_and_offsets_text -- --test-threads=1` は `1 passed`。
- 検証: 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core-storybook --locked quoted_code_block_draws_quote_bar_before_indented_code_box -- --test-threads=1` は `1 passed`。
- 検証: 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core-storybook --locked quoted_list_item_draws_material_bullet_and_offsets_text -- --test-threads=1` は `1 passed`。
- 検証: 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core` で `rtk cargo test -p katana-ui-core-storybook --locked heading_without_border_suppresses_storybook_heading_underline -- --test-threads=1` は `1 passed`。
- 検証: `rtk rg -n "kdv-list" /Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas*.rs` は一致なし。
- 検証: `rtk rg -n "kdv-document-|kdv-alert-|kdv-code|kdv-list" /Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas*.rs` は一致なし。
- 検証: `rtk rg -n "kdv-|viewer\\.(image|diagram|code)\\." /Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src /Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src /Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/tests` は一致なし。
- 未完了証跡: `rtk cargo test -p kdv-storybook --locked katana_feature_matrix_reaches_storybook_frame_pixels -- --test-threads=1` は `katana/sample_basic.md missing #f85149` で fail。Rule 境界とは別に alert warning pixel gate が未達。
- `kuc-adapter-boundary-check` は `link_action_matches_document_line`、`LinkAction`、`matches_document_line`、`ACCORDION_HEADER_HEIGHT`、`node_cursor`、`props().state_id`、`CHECKBOX_HIT_WIDTH`、`row_rect`、task action の style class 収集が戻ったら fail する。

## Acceptance Checklist

```text
[/] KatanA fixtures が KDV assets に固定されている。
[/] KatanA reference HTML/PDF/screenshots が KDV assets に固定されている。
[ ] Storybook TreeView が KatanA Explorer 相当の folder/file icon、indent line、selected row を表示する。
[ ] Storybook TreeView の file row click で fixture が切り替わる。
[ ] Storybook TreeView の directory row click で開閉する。
[ ] Storybook TreeView の scroll 後も表示行と hit-test が一致する。
[ ] Storybook SettingsList の section header click で開閉する。
[ ] Storybook SettingsList の Toggle が KUC Toggle action で動く。
[ ] Storybook SettingsList の Select が KUC Select action で動く。
[ ] Storybook に manual TreeView hit-test がない。
[ ] Storybook に manual media hit-test がない。
[ ] Storybook に settings action 合成がない。
[ ] style class / state id が action contract になっていない。
[ ] KDV core に vendor crate 依存がない。
[ ] KUC TreeView action で file selection と directory toggle が動く。
[ ] KUC SettingsList action で設定変更が動く。
[ ] KUC host action hit rect と KDV viewer media action で diagram/image 操作が動く。
[ ] MediaControl が KatanA diagram controller 相当の配置と transparent button を持つ。
[ ] MediaControl の各ボタンが押せる。
[ ] Markdown が raw 表示ではなく node 表示される。
[ ] HTML alignment が表示に反映される。
[ ] table / alert / code / horizontal rule が表現される。
[ ] link hover cursor と click が span 単位で動く。
[ ] task checkbox と context menu が動く。
[ ] search highlight と jump が動く。
[ ] accordion が動く。
[ ] diagram pending が即時表示される。
[ ] diagram dark theme が renderer に渡る。
[ ] scroll / resize が動く。
[ ] score gate が TreeView / SettingsList / Toggle / MediaControl の broken UI を fail できる。
[ ] visual / semantic / interaction / performance score が全カテゴリ 95 点以上。
[ ] just storybook が interactive viewer を起動する。
```
