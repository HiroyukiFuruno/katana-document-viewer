## ADDED Requirements

### Requirement: KDVはUI非依存の文書成果物契約を提供しなければならない

システムは、KUC / eguiに依存しない `katana-document-viewer` neutral crateで、文書入力、文書snapshot、成果物（artifact）、診断を表す型を提供しなければならない（MUST）。

#### Scenario: KMM documentをsnapshot化する

- **WHEN** ホストがKMM公開データ型（public DTO）をKDVへ渡す
- **THEN** KDVは `DocumentSource` と `DocumentSnapshot` として保持する
- **THEN** KMM内部parser型やrenderer内部型をKDV public APIへ露出しない
- **THEN** source revisionとdocument idを持ち、後続の評価と書き出し（export）で同じsnapshotを参照できる

#### Scenario: 成果物manifestを返す

- **WHEN** KDVがpreview / HTML / PDF / PNG / JPEG のいずれかを生成する
- **THEN** KDVは `ArtifactManifest` を返す
- **THEN** manifestはartifact id、format、source revision、diagnostics、生成backendを含む
- **THEN** manifestはsnapshot testで比較できる安定した構造を持つ

### Requirement: KDV forge はUI frameworkに依存してはならない

システムは、`katana-document-viewer::forge` をUIなしでcompileできるようにしなければならない（MUST）。

#### Scenario: no-UI dependency guardを実行する

- **WHEN** `cargo tree -p katana-document-viewer` を実行する
- **THEN** `egui`、`katana-ui-core`、`winit`、`vello` は含まれない
- **THEN** KDRへは公開契約またはadapter境界だけで接続する

#### Scenario: forge pipelineを構築する

- **WHEN** ホストが `BuildRequest` を渡す
- **THEN** KDVは `BuildGraph` を作成する
- **THEN** `BuildGraph` は書き出し（export）と描画評価で共有される
- **THEN** KDVはMarkdown本文を独自に再parseして別の正本を作らない

### Requirement: Markdown描画評価を自動テストで検証できなければならない

システムは、KUCの実画面に依存せず、Markdown描画評価を中間成果物で検証できなければならない（MUST）。

#### Scenario: CommonMark / GFM / KatanA互換fixtureを評価する

- **WHEN** CommonMark、GFM、KatanA互換の検証用入力（fixture）をKDVへ渡す
- **THEN** KDVは同じ `DocumentSnapshot` から評価用artifactを生成する
- **THEN** fixtureごとに期待するartifact manifestとdiagnosticsを比較できる
- **THEN** spec Scenario名または対応表から、どの要求を検証しているか追跡できる

#### Scenario: Markdown標準の全記法をKMM入力として評価する

- **WHEN** KMMがCommonMarkのブロック要素とインライン要素を含む公開データ型（public DTO）を渡す
- **THEN** KDVはKMM DTOを正本として `BuildGraph` を作る
- **THEN** KDVはMarkdown本文を独自に再parseしない
- **THEN** Markdown標準の必須記法は、raw sourceとdiagnosticsだけで完了扱いにしてはならない
- **THEN** KDVはCommonMark / GFMのfixture matrixに、KMM DTO化済み、未実装、外部backend待ちのどれに該当するかを記録する

#### Scenario: KMM coverage gapを補完parseしない

- **WHEN** KMM v0がCommonMark / GFMの一部記法を専用DTOへ構造化していない
- **THEN** KDVはその記法を独自parserで補完しない
- **THEN** KDVはcoverage gapを未完了の検証項目として記録する
- **THEN** 後続のKMM改善でDTOが追加された場合、同じfixture idで期待manifestを合格条件へ更新できる

#### Scenario: 数式とGitHub alertを評価対象に含める

- **WHEN** KMM documentにinline math、fenced math、`$$` 内側の半角スペースを許容するKatanA互換math、GitHub alertが含まれる
- **THEN** KDVはGitHub alertをKMM DTOの評価対象として扱う
- **THEN** KDVはmathを評価対象として扱い、raw sourceとdiagnosticsだけで完了扱いにしてはならない
- **THEN** KDVはmathとalertの評価結果をartifact manifestとdiagnosticsへ保持する
- **THEN** HTML/PDF/PNG/JPG書き出し（export）はその評価結果を再利用する

#### Scenario: KatanA独自解釈を評価対象に含める

- **WHEN** KMM documentに中央寄せHTML、badge row、legacy note、description list、`[-]` / `[/]` task marker、Draw.io直接code block、`.drawio` / `.xml` 参照、ZenUML、日本語、HTML entityが含まれる
- **THEN** KDVはKatanA互換fixtureとして評価する
- **THEN** KDVはKMM DTOとKDR結果からartifact manifestを生成する
- **THEN** ZenUMLはMermaid fence内の `zenuml` contentとして扱い、KDVはKMMにZenUML専用enumを要求しない
- **THEN** KDVはKatanA独自解釈をexport形式ごとに別解釈しない

#### Scenario: 外部描画失敗時にsourceを失わない

- **WHEN** Mermaid、Draw.io、ZenUML、PlantUML、mathの外部描画またはKRR境界のSVG生成が失敗する
- **THEN** KDVは元sourceをdiagnosticsとartifact manifestに保持する
- **THEN** KDVは独自rendererへfallbackしない
- **THEN** 後続viewerはraw表示に必要な情報をartifactから取り出せる

### Requirement: export は同じ中間成果物から生成されなければならない

システムは、HTML/PDF/PNG/JPG書き出し（export）を、viewer表示と同じ中間成果物から生成できる契約にしなければならない（MUST）。

#### Scenario: export requestを処理する

- **WHEN** ホストが `ExportRequest` と `ExportFormat` を渡す
- **THEN** KDVは `BuildGraph` から `ExportOutput` を生成する
- **THEN** formatごとの `ArtifactManifest` を返す
- **THEN** export artifact のbytesは0 byteであってはならない
- **THEN** export処理はKMM DTOを独自に再parseしない

#### Scenario: export debug出力を利用者指定ディレクトリに保存する

- **WHEN** 利用側が入力Markdownと出力先ディレクトリを指定する
- **THEN** KDVは出力先ディレクトリ配下にHTML/PDF/PNG/JPGファイルを生成する
- **THEN** 生成後に各ファイルの容量が0 byteではないことを検証する
- **THEN** `*.manifest.toml` のような形式別sidecarファイルを成果物フォルダへ残さない

#### Scenario: native surface exportで評価済みblockを視覚表現として保持する

- **GIVEN** KMM評価済みMarkdownにKDRが返したSVG図形とコードブロックが含まれる
- **WHEN** KDVがPDF/PNG/JPGを生成する
- **THEN** KDVはSVG sourceやSVG内CSSを本文テキストとして出力しない
- **THEN** 図形はRust側surface上でrasterizeされた図形blockとして描画される
- **THEN** コードブロックはテーマ由来の背景と枠を持つblockとして描画される

#### Scenario: 形式別にdiagnosticsを保持する

- **WHEN** HTML/PDF/PNG/JPGのいずれかで外部描画やasset解決に失敗する
- **THEN** KDVは `ForgeDiagnostics` と `ArtifactDiagnostics` に失敗理由を保持する
- **THEN** HTMLでは詳細情報を属性または補助metadataとして保持できる
- **THEN** PDF/PNG/JPGでは代表メッセージとraw sourceを失わない

### Requirement: KDR direct backend境界を固定しなければならない

システムは、KDV v0.1.0でKDRを利用するが、KDR側のpublic API縮小やCLI delegate化を行ってはならない（MUST NOT）。

#### Scenario: KDR direct renderingの対象を限定する

- **WHEN** KDVがKDRへ直接 `RenderInput` を渡す
- **THEN** KDVはKDR public APIで直接表現できるMermaid / Draw.io / PlantUMLを渡す
- **THEN** KDVは上位から受け取った完全テーマ（theme）をKDR `RenderThemeSnapshot` に変換し、`RenderContext.theme` として渡す
- **THEN** KDRの暗色表示判定は `RenderThemeSnapshot.mode` を使い、KDV側で別の暗黙fallbackを持たない
- **THEN** その他未対応kindはKDRに偽装せず、KDV diagnosticsとして返す
- **AND** 数式（math）はKRR境界を使い、HTML/PDF/PNG/JPGで同じSVGを出力・描画する
- **AND** KRR公開前のKDV内stubは、受け取ったTeX文字列をSVG化するだけでMarkdown AST解析を行わない
- **THEN** ZenUMLはKDR Mermaid runtimeの互換入力として扱える場合だけMermaid経路へ渡す

#### Scenario: KDR結果をKDV artifactへ変換する

- **WHEN** KDRがrender outputを返す
- **THEN** KDVはその結果をKDV `Artifact` へ変換する
- **THEN** backend固有型をKDV public APIへ漏らさない
- **THEN** KDR既存利用者の挙動をこのchangeで変更しない
